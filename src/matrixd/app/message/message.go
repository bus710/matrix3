package message

// WebSocketMessage - can be used for the websocket response to the clients
type WebSocketMessage struct {
	Type string `json:"type"`
	Data string `json:"data"`
}

// Matrix data
type MatrixData [][]int
