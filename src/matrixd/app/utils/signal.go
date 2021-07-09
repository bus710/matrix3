package utils

import (
	"context"
	"log"
	"os"
	"os/signal"
	"sync"
	"syscall"
	"time"

	"github.com/bus710/matrixd/src/matrixd/app/api"
	"github.com/bus710/matrixd/src/matrixd/app/matrix"
)

// Signal - the main struct of this module
type Signal struct {
	// app-wide items
	wait   *sync.WaitGroup
	server *api.WebServer
	matrix *matrix.Matrix

	// channels
	sigterm  chan os.Signal
	chanStop chan bool
}

// init - takes a WG, instance of service, and channels of go routines
// and keeps the assigned params in its struct to access later.
func (sig *Signal) Init(
	wait *sync.WaitGroup,
	serverInstance *api.WebServer,
	senseHatInstance *matrix.Matrix) {

	// To assign instances to the pointers
	sig.wait = wait
	sig.server = serverInstance
	sig.matrix = senseHatInstance

	//
	sig.chanStop = make(chan bool, 1)
	sig.sigterm = make(chan os.Signal, 1)
}

// catcher - a handler to catch the interrupts from keyboard (CTRL+C)
// and gracefully shuts down.
func (sig *Signal) Catcher() {

	// To connect the keyboard signal to the channel.
	signal.Notify(sig.sigterm, syscall.SIGINT, syscall.SIGTERM)

	// The routine waits here for the keyboard interrupt.
	select {
	case received := <-sig.sigterm:
		log.Println()
		log.Println("Received a CTRL+C", received)
		if err := sig.cleanup(); err != nil {
			log.Println(err)
		}
	case <-sig.chanStop:
		log.Println()
		log.Println("Received a signal")
		if err := sig.cleanup(); err != nil {
			log.Println(err)
		}
	}
}

// Running a graceful shutdown.
func (sig *Signal) cleanup() (err error) {

	log.Println("Cleanup - started")

	// To send a signal to the sensorHat's channel
	sig.matrix.ChanStop <- true

	time.Sleep(time.Microsecond * 100)

	// To call the shutdown method of the webserver
	ctx, cancel := context.WithTimeout(
		context.Background(), time.Millisecond*300)

	defer cancel()

	if err := sig.server.Shutdown(ctx); err != nil {
		return err
	}

	// To decrease the wait group
	sig.wait.Done()
	log.Println("Cleanup - done")

	return nil
}
