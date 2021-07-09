import { writable } from 'svelte/store';

let dots = [];
for (let i = 0; i < 64; i++) {
    dots.push({ selected: false, background_color: "#000000ff", font_color: "#ffffffff" });
}

export const stored_color = writable({ sliderR: 0, sliderG: 0, sliderB: 0, alpha: 0 });
export const stored_dots = writable(dots);
export const stored_select = writable("");


// https://svelte.dev/repl/29a5bdfb981f479fb387298aef1190a0?version=3.22.2
export const stored_message = writable("");
const socket = new WebSocket("ws://" + location.hostname + ":8000/v1/ws");

// Connection opened
socket.addEventListener('open', function (event) {
    console.log("It's opened");
});

// Connection closed
socket.addEventListener('close', function (event) {
    console.log("It's closed");
});

// Listen for messages
socket.addEventListener('message', function (event) {
    stored_message.set(event.data);
    stored_message.set("");
});

const sendMessage = (message) => {
    if (socket.readyState <= 1) {
        socket.send(message);
    }
}

export default sendMessage
