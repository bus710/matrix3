package common

// WebSocketMessage - can be used for the websocket response to the clients
type WebSocketMessage struct {
	Type string `json:"type"`
	Data string `json:"data"`
}

// MatrixData ...
type MatrixData struct {
	R [64]uint8 `json:"R"`
	G [64]uint8 `json:"G"`
	B [64]uint8 `json:"B"`
}

// Observer ...
type Observer struct {
	ID       string
	ChanData chan MatrixData
}
