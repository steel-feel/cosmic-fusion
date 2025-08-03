# Cosmic Fusion Resolver

To install dependencies:

```bash
bun install
```

To run:

```bash
bun run index.ts
```

This project was created using `bun init` in bun v1.2.8. [Bun](https://bun.sh) is a fast all-in-one JavaScript runtime.


## Public API Methods:
broadcast(): Send messages to all clients
sendToClient(): Send message to specific client
getConnectedClients(): Get list of client IDs
getClientCount(): Get number of connected clients
stop(): Gracefully stop the server
## Usage
Start the server:
```bash
bun run server
```

### Test with the HTML client:
Open test-websocket.html in your browser
The page will automatically connect to ws://localhost:3000