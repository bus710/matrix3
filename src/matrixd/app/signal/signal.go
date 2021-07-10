package signal

import (
	"os"
	"os/signal"
	"sync"
	"syscall"
	"time"

	server "github.com/bus710/matrixd/src/matrixd/app/api"
	matrix "github.com/bus710/matrixd/src/matrixd/app/matrix"
	window "github.com/bus710/matrixd/src/matrixd/app/window"
)

// TermSignal ...
type TermSignal struct {
	// Join
	wait *sync.WaitGroup
	// Channels
	sigterm              chan os.Signal
	windowCloseIndicator chan bool
}

var Signal TermSignal

// Init ...
func (sig *TermSignal) Init(wait *sync.WaitGroup, windowCloseIndicator chan bool) {
	// Store join
	sig.wait = wait
	// Initialize channels
	sig.sigterm = make(chan os.Signal, 1)
	sig.windowCloseIndicator = windowCloseIndicator
}

// Run catch the interrupts from keyboard (CTRL+C)
func (sig *TermSignal) Run() {
	// Connect the keyboard signal to the channel.
	signal.Notify(sig.sigterm, syscall.SIGINT, syscall.SIGTERM)

	// Wait for the keyboard interrupt.
	select {
	case <-sig.sigterm:
	case <-sig.windowCloseIndicator:
	}

	// Shutdown webview
	time.Sleep(time.Millisecond * 100)
	window.Shutdown()

	// Shutdown matrix controller
	time.Sleep(time.Millisecond * 100)
	matrix.Matrix.Shutdown()

	// Shutdown webserver
	time.Sleep(time.Millisecond * 100)
	server.Server.Shutdown()

	// Decrease the wait group
	sig.wait.Done()
}
