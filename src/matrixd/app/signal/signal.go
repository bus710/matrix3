package signal

import (
	"log"
	"os"
	"os/signal"
	"sync"
	"syscall"
	"time"

	server "github.com/bus710/matrixd/src/matrixd/app/api"
	matrix "github.com/bus710/matrixd/src/matrixd/app/matrix"
)

// TermSignal ...
type TermSignal struct {
	// Join
	wait *sync.WaitGroup
	// Channels
	sigterm chan os.Signal
}

var Signal TermSignal

// Init ...
func (sig *TermSignal) Init(wait *sync.WaitGroup) {
	// Store join
	sig.wait = wait
	// Initialize channels
	sig.sigterm = make(chan os.Signal, 1)
}

// Run catch the interrupts from keyboard (CTRL+C)
func (sig *TermSignal) Run() {
	// Connect the keyboard signal to the channel.
	signal.Notify(sig.sigterm, syscall.SIGINT, syscall.SIGTERM)

	// Wait for the keyboard interrupt.
	received := <-sig.sigterm
	log.Println("Received a CTRL+C", received)

	// Shutdown matrix controller
	time.Sleep(time.Millisecond * 100)
	matrix.Matrix.Shutdown()

	// Shutdown webserver
	time.Sleep(time.Millisecond * 100)
	server.Server.Shutdown()

	// Decrease the wait group
	sig.wait.Done()
}
