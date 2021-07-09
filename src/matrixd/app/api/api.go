package api

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
	"strconv"
	"sync"

	"github.com/bus710/matrixd/src/matrixd/app/matrix"
	"github.com/bus710/matrixd/src/matrixd/app/message"
	"golang.org/x/net/websocket"
)

// WebServer - the main struct of this module
type WebServer struct {
	// app-wide items
	wait     *sync.WaitGroup
	senseHat *matrix.Matrix

	// web items
	instance *http.Server

	// data
	receivedItemWS *message.WebSocketMessage
	responseItemWS *message.WebSocketMessage
}

// init - initializes the data and structs
func (wserver *WebServer) Init(
	wait *sync.WaitGroup,
	senseHatInstance *matrix.Matrix) (err error) {

	wserver.wait = wait
	wserver.instance = &http.Server{Addr: ":3000"}
	wserver.senseHat = senseHatInstance
	return nil
}

func (wserver *WebServer) Shutdown(ctx context.Context) error {
	err := wserver.instance.Shutdown(ctx)
	if err != nil {
		log.Println(err)
		return err
	}
	return nil
}

// run - delivers the static web files and serves the REST API (+ websocket)
func (wserver *WebServer) Run() (err error) {
	// WebSocket
	http.Handle("/message", websocket.Handler(wserver.socket))

	// Web Contents
	// The frontend side should be built by webdev build --output build
	// Otherwise, the location will be ../front/build
	http.Handle("/", http.FileServer(http.Dir("../front/build/web")))

	// Server up and running
	log.Println(wserver.instance.ListenAndServe())

	wserver.wait.Done()
	return nil
}

// socket - websocket handler
func (wserver *WebServer) socket(wsocket *websocket.Conn) {

	log.Println(wsocket.Request().RemoteAddr)
	chanData := make(chan string, 1)
	chanResponse := make(chan bool, 1)

	defer wsocket.Close()

	// Processing routine
	go func() {
		for {
			select {
			case data := <-chanData:
				/* for future unmarshling
				https://mholt.github.io/json-to-go/ */
				var dataList message.MatrixData
				// log.Println("Processing routine: " + data)

				if err := json.Unmarshal([]byte(data), &dataList); err != nil {
					log.Println(err)
					chanResponse <- false
				} else {
					if len(dataList) == 64 {
						// log.Println(dataList[0][0])

						for i := 0; i < 64; i++ {
							wserver.senseHat.BufR[i] = byte(dataList[i][0])
							wserver.senseHat.BufG[i] = byte(dataList[i][1])
							wserver.senseHat.BufB[i] = byte(dataList[i][2])
						}

						// To notify the data is ready to the sensorHat routine
						wserver.senseHat.ChanDataReady <- true
						// To notify the data is ready to the client
						chanResponse <- true
					} else {
						chanResponse <- false
					}
				}
			}
		}
	}()

	// Sending routine
	go func() {
		for {
			select {
			case res := <-chanResponse:
				wserver.responseItemWS = &message.WebSocketMessage{
					Type: "response", Data: strconv.FormatBool(res)}
				websocket.JSON.Send(wsocket, wserver.responseItemWS)
			}
		}
	}()

	wserver.receivedItemWS = &message.WebSocketMessage{}
	// Receiving routine
	for {
		// receive a message using the codec
		if err := websocket.JSON.Receive(
			wsocket, &wserver.receivedItemWS); err != nil {
			// log.Println(err)
			break
		} else {
			messageType := wserver.receivedItemWS.Type
			messageData := wserver.receivedItemWS.Data
			log.Println("Received message type:", messageType)
			// log.Println("Received message data:", messageData)
			chanData <- messageData
		}
	}

	log.Println("Websocket closed")
}
