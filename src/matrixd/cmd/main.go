package main

import (
	"log"
	"sync"

	server "github.com/bus710/matrixd/src/matrixd/app/api"
	matrix "github.com/bus710/matrixd/src/matrixd/app/matrix"
	signal "github.com/bus710/matrixd/src/matrixd/app/signal"
	window "github.com/bus710/matrixd/src/matrixd/app/window"
)

func main() {
	log.Println("Hello!")

	windowCloseIndicator := make(chan bool, 1)
	waitInstance := sync.WaitGroup{}

	matrix.Matrix.Init(&waitInstance)
	server.Server.Init(&waitInstance)
	window.View.Init(&waitInstance, windowCloseIndicator)
	signal.Signal.Init(&waitInstance, windowCloseIndicator)

	waitInstance.Add(1)
	go signal.Signal.Run()
	waitInstance.Add(1)
	go matrix.Matrix.Run()
	waitInstance.Add(1)
	go server.Server.Run()
	waitInstance.Add(1)
	go window.View.Run()

	waitInstance.Wait()

	log.Println("Bye!")
}
