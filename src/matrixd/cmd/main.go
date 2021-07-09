package matrixd

import (
	"log"
	"sync"

	api "github.com/bus710/matrixd/src/matrixd/app/api"
	matrix "github.com/bus710/matrixd/src/matrixd/app/matrix"
	utils "github.com/bus710/matrixd/src/matrixd/app/utils"
)

func main() {
	log.Println("Hello!")
	log.Println("Please open the 3000 port.")

	waitInstance := sync.WaitGroup{}
	senseHatInstance := matrix.Matrix{}
	serverInstance := api.WebServer{}
	signalInstance := utils.Signal{}

	senseHatInstance.Init(&waitInstance)
	serverInstance.Init(&waitInstance, &senseHatInstance)
	signalInstance.Init(&waitInstance, &serverInstance, &senseHatInstance)

	waitInstance.Add(1)
	go signalInstance.Catcher()
	waitInstance.Add(1)
	go senseHatInstance.Run()
	waitInstance.Add(1)
	go serverInstance.Run()

	waitInstance.Wait()

	log.Println()
	log.Println("See you again!")
}
