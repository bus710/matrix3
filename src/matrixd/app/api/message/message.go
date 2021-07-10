package message

import (
	"encoding/json"
	"fmt"
	"log"
	"math/rand"
	"net/http"
	"time"

	"github.com/bus710/matrixd/src/matrixd/app/common"
	"github.com/bus710/matrixd/src/matrixd/app/matrix"
	"github.com/gorilla/websocket"
	"github.com/labstack/echo/v4"
)

var (
	upgrader = websocket.Upgrader{}
)

func WebSocketHandler(c echo.Context) error {
	upgrader.CheckOrigin = func(r *http.Request) bool { return true }
	ws, err := upgrader.Upgrade(c.Response(), c.Request(), nil)
	if err != nil {
		log.Println("websocket upgrade:", err)
		return err
	}
	defer ws.Close()

	// Register this websocket instance to the observer list
	id := fmt.Sprint(time.Now().Unix()) + fmt.Sprint(rand.Intn(64))
	ch := make(chan common.MatrixData, 1)
	matrix.AddObserver(id, ch)
	defer matrix.RemoveObserver(id)

	tick := time.NewTicker(1000 * time.Millisecond)

	// loop:
	for {
		select {
		case <-tick.C:
			// break loop
		case d := <-ch:
			r, err := json.Marshal(d)
			if err != nil {
				c.Logger().Error(err)
				continue
			}
			err = ws.WriteMessage(websocket.TextMessage, []byte(r))
			if err != nil {
				c.Logger().Error(err)
			}
		}
	}

	return nil
}
