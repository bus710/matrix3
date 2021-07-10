package window

import (
	"sync"

	"github.com/webview/webview"
)

type WebView struct {
	// Join
	wait *sync.WaitGroup
	view webview.WebView
	// Channels
	windowCloseIndicator chan bool
}

var View WebView

func (wv *WebView) Init(wait *sync.WaitGroup, windowCloseIndicator chan bool) {
	wv.wait = wait
	wv.windowCloseIndicator = windowCloseIndicator
}

func (wv *WebView) Run() {
	debug := true
	wv.view = webview.New(debug)
	defer wv.view.Destroy()
	wv.view.SetSize(500, 695, webview.HintFixed)
	wv.view.Navigate("http://localhost:8000")
	wv.view.Run()

	//
	wv.wait.Done()
	//
	wv.windowCloseIndicator <- true
}

func Shutdown() {
	View.view.Terminate()
}
