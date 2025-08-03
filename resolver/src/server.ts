import { Server } from "bun";
import { deployEscrows } from "./mocks/cosmos_src_flow";

interface WebSocketMessage {
  type: string;
  data: any;
  timestamp: number;
}

interface Client {
  id: string;
  ws: any; // Using any to avoid type conflicts with Bun's WebSocket
  connectedAt: number;
}

class WebSocketServer {
  private server: Server | null = null;
  private clients: Map<string, Client> = new Map();
  private port: number;

  constructor(port: number = 3000) {
    this.port = port;
  }

  start() {
    this.server = Bun.serve({
      port: this.port,
      fetch: (req, server) => {
        const url = new URL(req.url);
        
        // Handle WebSocket upgrade
        if (server.upgrade(req)) {
          return; // Return if upgrade was successful
        }

        // Handle HTTP requests
        return new Response("WebSocket server is running", {
          status: 200,
          headers: {
            "Content-Type": "text/plain",
          },
        });
      },
      websocket: {
        open: (ws) => {
          const clientId = this.generateClientId();
          const client: Client = {
            id: clientId,
            ws,
            connectedAt: Date.now(),
          };
          
          this.clients.set(clientId, client);
          console.log(`Client ${clientId} connected. Total clients: ${this.clients.size}`);
          
          // Send welcome message
          this.sendMessage(ws, {
            type: "connection",
            data: { clientId, message: "Connected to WebSocket server" },
            timestamp: Date.now(),
          });
        },
        message: (ws, message) => {
          this.handleMessage(ws, message);
        },
        close: (ws, code, reason) => {
          const clientId = this.findClientId(ws);
          if (clientId) {
            this.clients.delete(clientId);
            console.log(`Client ${clientId} disconnected. Total clients: ${this.clients.size}`);
          }
        },
      },
    });

    console.log(`WebSocket server started on port ${this.port}`);
    console.log(`HTTP server also available on http://localhost:${this.port}`);
  }

  private generateClientId(): string {
    return `client_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private findClientId(ws: any): string | null {
    for (const [clientId, client] of this.clients.entries()) {
      if (client.ws === ws) {
        return clientId;
      }
    }
    return null;
  }

  private async handleMessage(ws: any, message: string | Buffer) {
    try {
      let parsedMessage: WebSocketMessage;
      
      if (typeof message === "string") {
        parsedMessage = JSON.parse(message);
      } else {
        parsedMessage = JSON.parse(message.toString());
      }

      const clientId = this.findClientId(ws);
      console.log(`Received message from ${clientId}:`, parsedMessage);

      // Process different message types
      switch (parsedMessage.type) {
        case "ping":
          this.sendMessage(ws, {
            type: "pong",
            data: { timestamp: Date.now() },
            timestamp: Date.now(),
          });
          break;
        
        case "order" : {
          //call cosmos
         await deployEscrows();

          this.sendMessage(ws, {
            type: "order_status",
            data: { timestamp: Date.now() },
            timestamp: Date.now(),
          });

          




        }
        break

        case "broadcast":
          // Broadcast message to all connected clients
          this.broadcastMessage({
            type: "broadcast",
            data: parsedMessage.data,
            timestamp: Date.now(),
          });
          break;

        case "private":
          // Send private message to specific client
          if (parsedMessage.data.targetClientId) {
            const targetClient = this.clients.get(parsedMessage.data.targetClientId);
            if (targetClient) {
              this.sendMessage(targetClient.ws, {
                type: "private",
                data: {
                  from: clientId,
                  message: parsedMessage.data.message,
                },
                timestamp: Date.now(),
              });
            }
          }
          break;

        case "get_clients":
          // Send list of connected clients
          this.sendMessage(ws, {
            type: "clients_list",
            data: {
              clients: Array.from(this.clients.keys()),
              total: this.clients.size,
            },
            timestamp: Date.now(),
          });
          break;

        default:
          // Echo back the message for unknown types
          this.sendMessage(ws, {
            type: "echo",
            data: parsedMessage,
            timestamp: Date.now(),
          });
          break;
      }
    } catch (error) {
      console.error("Error processing message:", error);
      this.sendMessage(ws, {
        type: "error",
        data: { message: "Invalid message format" },
        timestamp: Date.now(),
      });
    }
  }

  private sendMessage(ws: any, message: WebSocketMessage) {
    if (ws.readyState === 1) { // WebSocket.OPEN
      ws.send(JSON.stringify(message));
    }
  }

  private broadcastMessage(message: WebSocketMessage) {
    this.clients.forEach((client) => {
      this.sendMessage(client.ws, message);
    });
  }

  // Public methods for external use
  public broadcast(data: any, type: string = "broadcast") {
    this.broadcastMessage({
      type,
      data,
      timestamp: Date.now(),
    });
  }

  public sendToClient(clientId: string, data: any, type: string = "message") {
    const client = this.clients.get(clientId);
    if (client) {
      this.sendMessage(client.ws, {
        type,
        data,
        timestamp: Date.now(),
      });
    }
  }

  public getConnectedClients(): string[] {
    return Array.from(this.clients.keys());
  }

  public getClientCount(): number {
    return this.clients.size;
  }

  public stop() {
    if (this.server) {
      this.server.stop();
      console.log("WebSocket server stopped");
    }
  }
}

// Create and start the server
const wsServer = new WebSocketServer(3000);
wsServer.start();

// Handle graceful shutdown
process.on("SIGINT", () => {
  console.log("Shutting down WebSocket server...");
  wsServer.stop();
  process.exit(0);
});

process.on("SIGTERM", () => {
  console.log("Shutting down WebSocket server...");
  wsServer.stop();
  process.exit(0);
});

export default wsServer;
