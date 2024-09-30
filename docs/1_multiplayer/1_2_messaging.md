# Overview

Here we discuss how communication takes place between various components of our game:
- Between game server and UI and vice verse
- Between players in the chat section

# Choice of Protocol

First we need to choose a suitable protocol for our multi-player needs.

| Protocol            | Use Case                               | Pros                                                                                                                                       | Cons                                                                                                             | Stream Support                    |
|---------------------|----------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------|-----------------------------------|
| **WebSocket**        | Real-time bidirectional communication  | Full-duplex communication, low latency, efficient for chat/gaming apps                                                                    | Requires more management (e.g., handling connection state), not as performant with poor network conditions       | Full-duplex streaming             |
| **SSE**              | Server-to-client updates               | Simple to implement, server-sent messages, native browser support                                                                          | Client-to-server communication requires separate requests, only unidirectional from server                       | Server-to-client streaming        |
| **TCP**              | General-purpose low-level communication | Reliable, ordered, connection-oriented, efficient for basic bi-directional data flow                                                       | No native support for Web-based apps, lacks application-layer protocol features like multiplexing                | Full-duplex streaming             |
| **QUIC**             | Low-latency, real-time communication   | Multiplexing without head-of-line blocking, built-in encryption, better performance over lossy networks                                    | Newer and more complex, less mature tooling and support, especially in some ecosystems                           | Full-duplex streaming             |
| **gRPC**             | RPC-based communication                | Built-in streaming, bi-directional communication, high performance, automatic code generation for various languages                        | Complex setup, requires HTTP/2, needs special client libraries                                                   | Full-duplex streaming             |
| **HTTP/2**           | Improved HTTP performance              | Multiplexing requests, server push, reduced latency, and improved connection reuse                                                          | More complex than HTTP/1.1, requires special configuration in some server environments                           | Server-to-client (server push)    |
| **REST**             | Request-response communication         | Simple, stateless, widely supported, easy caching                                                                                         | No native streaming, higher latency due to repeated connections, not suitable for real-time interactions         | No streaming                      |

Certain parts of the game seem to be more suited with `Http Server` and for other parts requiring `duplex` communication, we will use `WebSocket`.
Here is a table of which features use what (this list is not complete it is just there to give us an idea):

| Component        | Functionality                                                                                     |
|------------------|---------------------------------------------------------------------------------------------------|
| **HTTP Server**   |                                                                                                   |
| Authentication/Authorization | Log in, register, and manage users.                                                        |
| Static Assets     | Serve the HTML, CSS, and JavaScript files for the game client.                                   |
| API Endpoints     | Endpoints to retrieve player stats, game history, available games, etc.                           |
| **WebSocket Server** |                                                                                                 |
| Game Actions      | Handle moves made by players in real time.                                                       |
| Real-Time Notifications | Send notifications about game events (e.g., turns, game end, etc.).                          |
| Chat Messages     | Allow players to send messages to each other during the game.                                     |


# Message Serialization
Here, popular choises are:
- JSON: `serde` package
- MessagePack/Protobuf: for performance

# Final Tech Choice
For now choose `WebSockets` as messaging mechanism and `Protobuf` as serialization method.

# Which WebSocket framework in Rust?
Doing a some research on Websocket frameworks, there are several to consider. Here we includes pros and cons;

- `Tokio Tungstenite`: seem to be deemed to memory heavy, allocating memory on each packet
- `actix-web`: look like a good choice, however, WebSocket support maybe using an older version.
- TODO! add more framework reviews here

# Actix-Web WebSocket Basic Impl
# Testing WebSocket
For this we can use `websocat` client.

1. Install `Websocat`
```bash
    # On macOS
    brew install websocat
    
    # On Ubuntu or Debian
    sudo apt-get install websocat
    
    # Or download from GitHub
    curl -LO https://github.com/vi/websocat/releases/download/v1.6.0/websocat_amd64-linux
    chmod +x websocat_amd64-linux
```

2. Establish WebSocket connection
```bash
websocat ws://127.0.0.1:8080/echo 
```

3. Type messages and get the copy of it back
```bash
websocat ws://127.0.0.1:8080/echo
# Type a message
Hello, WebSocket!

# The server will echo back
Hello, WebSocket!
```



# Handling WebSocket Interruptions


# Security

