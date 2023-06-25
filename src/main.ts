// client implementation
import * as socketIo from "socket.io-client";

const socket = socketIo.connect("http://localhost:3000", {
	reconnectionAttempts: 1,
	autoConnect: false,
});

let timeout: number | undefined;

socket.on("connect", async () => {
  console.log("socket: connected");

	socket.emit("hello");

	timeout = window.setTimeout(() => {
		console.error("socket: timeout");
	}, 5000);
});

socket.on("world", (message: string) => {
  console.log("socket: received 'world' with message '" + message + "'");
  clearTimeout(timeout);
});

socket.on("disconnect", () => {
	console.log("socket: disconnected");
});

socket.open();
