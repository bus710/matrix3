package main

import (
	"log"
	"sync"

	server "github.com/bus710/matrixd/src/matrixd/app/api"
	matrix "github.com/bus710/matrixd/src/matrixd/app/matrix"
	signal "github.com/bus710/matrixd/src/matrixd/app/signal"
)

func main() {
	log.Println("Hello!")

	waitInstance := sync.WaitGroup{}

	matrix.Matrix.Init(&waitInstance)
	server.Server.Init(&waitInstance)
	signal.Signal.Init(&waitInstance)

	waitInstance.Add(1)
	go signal.Signal.Run()
	waitInstance.Add(1)
	go matrix.Matrix.Run()
	waitInstance.Add(1)
	go server.Server.Run()

	waitInstance.Wait()

	log.Println("Bye!")
}
