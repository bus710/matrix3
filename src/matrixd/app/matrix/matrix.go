package matrix

import (
	"log"
	"runtime"
	"strings"
	"sync"
	"time"

	"github.com/bus710/matrixd/src/matrixd/app/common"
	"periph.io/x/periph/conn"
	"periph.io/x/periph/conn/i2c"
	"periph.io/x/periph/conn/i2c/i2creg"
	"periph.io/x/periph/host"
)

// Matrix ...
var Matrix SenseHatMatrix

// SenseHatMatrix ...
type SenseHatMatrix struct {
	// Join
	wait *sync.WaitGroup
	// Mutex
	Mux sync.RWMutex
	// Channels
	chanStop  chan bool
	chanData  chan common.MatrixData
	observers []common.Observer
	// Local items
	isARM  bool
	i2cBus i2c.BusCloser
	i2cDev i2c.Dev
	i2cCon conn.Conn
	// Elements for the matrix
	matrixAddr    uint16
	bufRaw        [193]byte
	butMatrixData common.MatrixData
}

// Init
func (mx *SenseHatMatrix) Init(wait *sync.WaitGroup) {
	// Store join
	mx.wait = wait
	// Channels
	mx.chanStop = make(chan bool, 1)
	mx.chanData = make(chan common.MatrixData, 3)
	// Buffers
	mx.butMatrixData = common.MatrixData{}

	// Confirm the archtecture
	if strings.Contains(runtime.GOARCH, "arm") ||
		strings.Contains(runtime.GOARCH, "arm64") {
		// Indicate the arcitecture
		mx.isARM = true

		// Initialize the baseline drivers
		_, err := host.Init()
		if err != nil {
			log.Println(err)
		}

		// Open the i2c of RPI
		bus, err := i2creg.Open("")
		if err != nil {
			log.Println(err)
		}

		// Initialize some numbers
		mx.matrixAddr = uint16(0x0046) // SensorHat's AVR MCU uses ID 0x46 for the LED matrix

		// Initialize the i2c bus
		mx.i2cBus = bus
		mx.i2cDev = i2c.Dev{Bus: mx.i2cBus, Addr: mx.matrixAddr}
		mx.i2cCon = &mx.i2cDev

		// Test
		mx.display_test()

		// Turn off all
		d := common.MatrixData{}
		err = mx.display(d)
		if err != nil {
			log.Println("Cannot use the i2c bus")
		}
	} else {
		// If the arch is not ARM...
		mx.isARM = false
	}
}

func (mx *SenseHatMatrix) Shutdown() {
	mx.chanStop <- true
}

// Run ...
func (mx *SenseHatMatrix) Run() {
	tick := time.NewTicker(1000 * time.Millisecond)

	if mx.isARM {
		defer mx.i2cBus.Close()
	}

loop:
	for {
		select {
		// Shutdown gracefully
		case <-mx.chanStop:
			break loop
		// When the webserver safely received a chunk of data
		case d := <-mx.chanData:
			mx.Mux.Lock()
			if mx.isARM {
				err := mx.display(d)
				if err != nil {
					log.Println(err)
				}
			}
			mx.butMatrixData = d
			mx.broadcast()
			mx.Mux.Unlock()
		// To run some task periodically
		case <-tick.C:
			// log.Println("test from the sensorhat routine")
		}
	}
	mx.wait.Done()
}

// Push expose the data channel
func Push(d *common.MatrixData) (err error) {
	Matrix.chanData <- *d
	return nil
}

// AddObserver ...
func AddObserver(id string, ch chan common.MatrixData) {
	observer := common.Observer{
		ID:       id,
		ChanData: ch,
	}
	Matrix.observers = append(Matrix.observers, observer)
	log.Println("Add a new observer", id)
}

// RemoveObserver ...
func RemoveObserver(id string) {
	if len(Matrix.observers) == 0 {
		return
	}

	index := 0
	for i, observer := range Matrix.observers {
		if observer.ID == id {
			index = i
			break
		}
	}
	Matrix.observers = append(Matrix.observers[:index], Matrix.observers[len(Matrix.observers)-1])
	log.Println("Remove an observer", id)
}

func (mx *SenseHatMatrix) display(d common.MatrixData) (err error) {
	// Map RGB to Raw (linear)
	j := int(0)
	for i := 0; i < 64; i++ {
		j = int(i/8) * 8
		j = j + j
		mx.bufRaw[i+j+1] = d.R[i] / 4
		mx.bufRaw[i+j+9] = d.G[i] / 4
		mx.bufRaw[i+j+17] = d.B[i] / 4
	}

	// Write
	writtenDataNum, err := mx.i2cDev.Write(mx.bufRaw[:])
	if err != nil {
		return err
	} else if writtenDataNum != 193 {
		return err
	}

	return nil
}

func (mx *SenseHatMatrix) display_test() {
	d := common.MatrixData{}
	for i := 0; i < 64; i++ {
		d.R[i] = 3
		d.G[i] = 3
		d.B[i] = 3
	}
	mx.display(d)
	time.Sleep(time.Millisecond * 100)
	for i := 0; i < 64; i++ {
		d.R[i] = 0
		d.G[i] = 0
		d.B[i] = 0
	}
	mx.display(d)
	time.Sleep(time.Millisecond * 100)
}

func (mx *SenseHatMatrix) broadcast() {
	if len(mx.observers) == 0 {
		return
	}

	for _, observer := range mx.observers {
		observer.ChanData <- mx.butMatrixData
	}
}
