package api

import (
	"context"
	"log"
	"math/rand"
	"net/http"
	"sync"
	"time"

	"github.com/bus710/matrixd/src/matrixd/app/api/message"
	"github.com/bus710/matrixd/src/matrixd/app/common"
	"github.com/bus710/matrixd/src/matrixd/app/matrix"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

// WebServer handles http requests
type WebServer struct {
	// Join
	wait *sync.WaitGroup
	// Server
	instance *echo.Echo
	// Websocket
	receivedItemWS *common.WebSocketMessage
	responseItemWS *common.WebSocketMessage
}

var Server WebServer

// Init ...
func (wserver *WebServer) Init(wait *sync.WaitGroup) {
	// Store join
	wserver.wait = wait
	// Assign server instance
	wserver.instance = echo.New()
}

// Shutdown ...
func (wserver *WebServer) Shutdown() error {
	ctx, cancel := context.WithTimeout(
		context.Background(), time.Millisecond*300)
	defer cancel()

	err := wserver.instance.Shutdown(ctx)
	if err != nil {
		log.Println(err)
		return err
	}
	return nil
}

// Run ...
func (wserver *WebServer) Run() (err error) {
	// Middleware
	wserver.instance.Use(middleware.Logger())
	wserver.instance.Use(middleware.Recover())
	wserver.instance.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		AllowOrigins: []string{"*"},
		AllowHeaders: []string{echo.HeaderOrigin, echo.HeaderContentType, echo.HeaderAccept},
	}))
	// Route
	wserver.instance.Static("/", "../public")
	wserver.instance.POST("/v1/ping", pingHandler)
	wserver.instance.POST("/v1/matrix", matrixHandler)
	wserver.instance.POST("/v1/random", randomHandler)
	wserver.instance.GET("/v1/ws", message.WebSocketHandler)
	// Serve
	err = wserver.instance.Start(":8000")
	if err != http.ErrServerClosed {
		log.Fatal(err)
	}

	wserver.wait.Done()
	return nil
}

func pingHandler(c echo.Context) error {
	log.Println("ping")
	return c.String(http.StatusOK, "pong")
}

func matrixHandler(c echo.Context) error {
	log.Println("matrix")
	d := new(common.MatrixData)
	err := c.Bind(d)
	if err != nil {
		return c.String(http.StatusNotFound, "matrix")
	}
	matrix.Push(d)
	return c.String(http.StatusOK, "matrix")
}

func randomHandler(c echo.Context) error {
	log.Println("random")
	d := new(common.MatrixData)

	for i := 0; i < 64; i++ {
		d.R[i] = uint8(rand.Intn(64))
		d.G[i] = uint8(rand.Intn(64))
		d.B[i] = uint8(rand.Intn(64))
	}

	matrix.Push(d)
	return c.String(http.StatusOK, "random")
}
