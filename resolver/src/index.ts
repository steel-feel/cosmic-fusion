/**
 * websocketClient.ts
 * * A reusable WebSocket client module designed for Bun's runtime environment.
 * It provides a class-based approach to manage WebSocket connections, handle
 * events, and send messages.
 * * To run this example:
 * 1. Make sure you have Bun installed.
 * 2. Save this file as `websocketClient.ts`.
 * 3. Run it from your terminal: `bun run websocketClient.ts`
 * * Note: You will need a running WebSocket server to connect to.
 * This example assumes a simple echo server on `ws://localhost:8080`.
 */

import WebSocket from "ws"; // Bun provides the 'ws' module built-in, but this import helps with type checking.
import EventEmitter from "events";

// Define a type for the message event payload for better type safety
type WebSocketMessage = string | ArrayBuffer | Buffer;

/**
 * Encapsulates the logic for a WebSocket client connection.
 * It extends EventEmitter to provide a clean event-based API.
 */
export class WebSocketClient extends EventEmitter {
    private ws: WebSocket | null = null;
    private readonly url: string;
    private readonly options: WebSocket.ClientOptions;

    /**
     * Creates an instance of WebSocketClient.
     * @param url The URL of the WebSocket server (e.g., "ws://localhost:8080").
     * @param options Optional WebSocket client options.
     */
    constructor(url: string, options?: WebSocket.ClientOptions) {
        super();
        this.url = url;
        this.options = options || {};
    }

    /**
     * Establishes a connection to the WebSocket server.
     */
    public connect(): void {
        console.log(`Attempting to connect to ${this.url}...`);
        this.ws = new WebSocket(this.url, this.options);

        // Bind event listeners to the WebSocket instance
        this.ws.onopen = (event) => {
            console.log("WebSocket connection established.");
            this.emit("open", event);
            this.sendMessage("Hello")
        };

        this.ws.onmessage = (event) => {
            console.log("Received message:", event.data.toString());
            this.emit("message", event.data);
        };

        this.ws.onclose = (event) => {
            console.log(`WebSocket connection closed with code ${event.code}: ${event.reason}`);
            this.emit("close", event);
            this.ws = null; // Clear the instance on close
        };

        this.ws.onerror = (event) => {
            console.error("WebSocket error:", event.error);
            this.emit("error", event.error);
        };
    }

    /**
     * Disconnects from the WebSocket server if a connection is active.
     */
    public disconnect(): void {
        if (this.ws) {
            console.log("Disconnecting from WebSocket server...");
            this.ws.close();
        } else {
            console.warn("No active WebSocket connection to disconnect.");
        }
    }

    /**
     * Sends a message to the WebSocket server.
     * @param message The message to send. Can be a string, Buffer, or ArrayBuffer.
     */
    public sendMessage(message: WebSocketMessage): void {
        if (this.ws && this.ws.readyState === WebSocket.OPEN) {
            this.ws.send(message);
        } else {
            console.error("Cannot send message: WebSocket is not open.");
        }
    }
}

/**
 * Example usage of the WebSocketClient class.
 * This code will run when the file is executed.
 */
async function main() {
    // Replace with your actual WebSocket server URL
    const wsUrl = "https://echo.websocket.org/"; 

    // Create a new client instance
    const client = new WebSocketClient(wsUrl);

    // // Register custom event handlers
    // client.on("open", () => {
    //     console.log("Custom 'open' handler: Ready to send messages!");
    //     // Send a message after the connection is open
    //     const messageToSend = "Hello from Bun!";
    //     console.log(`Sending message: "${messageToSend}"`);
    //     client.sendMessage(messageToSend);
    // });

    // client.on("message", (data: WebSocketMessage) => {
    //     console.log(`Custom 'message' handler: Echoed message received: ${data.toString()}`);
    //     // Disconnect after receiving the first message
    //     client.disconnect();
    // });

    // client.on("close", (event: WebSocket.CloseEvent) => {
    //     console.log("Custom 'close' handler: Connection has been closed.");
    // });

    // client.on("error", (error: Error) => {
    //     console.error("Custom 'error' handler: An error occurred.");
    // });
    
    // Attempt to connect to the server
    client.connect();
}

// Start the example
main();
