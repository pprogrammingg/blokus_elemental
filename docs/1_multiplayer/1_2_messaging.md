# Overview

Here we discuss how communication takes place between various components of our game:
- Between game server and UI and viceverse
- Between players in the chat section

# Game <-> UI
| Protocol            | Use Case                               | Pros                                                                                                                                       | Cons                                                                                                             | Stream Support                    |
|---------------------|----------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------|-----------------------------------|
| **WebSocket**        | Real-time bidirectional communication  | Full-duplex communication, low latency, efficient for chat/gaming apps                                                                    | Requires more management (e.g., handling connection state), not as performant with poor network conditions       | Full-duplex streaming             |
| **SSE**              | Server-to-client updates               | Simple to implement, server-sent messages, native browser support                                                                          | Client-to-server communication requires separate requests, only unidirectional from server                       | Server-to-client streaming        |
| **TCP**              | General-purpose low-level communication | Reliable, ordered, connection-oriented, efficient for basic bi-directional data flow                                                       | No native support for Web-based apps, lacks application-layer protocol features like multiplexing                | Full-duplex streaming             |
| **QUIC**             | Low-latency, real-time communication   | Multiplexing without head-of-line blocking, built-in encryption, better performance over lossy networks                                    | Newer and more complex, less mature tooling and support, especially in some ecosystems                           | Full-duplex streaming             |
| **gRPC**             | RPC-based communication                | Built-in streaming, bi-directional communication, high performance, automatic code generation for various languages                        | Complex setup, requires HTTP/2, needs special client libraries                                                   | Full-duplex streaming             |
| **HTTP/2**           | Improved HTTP performance              | Multiplexing requests, server push, reduced latency, and improved connection reuse                                                          | More complex than HTTP/1.1, requires special configuration in some server environments                           | Server-to-client (server push)    |
| **REST**             | Request-response communication         | Simple, stateless, widely supported, easy caching                                                                                         | No native streaming, higher latency due to repeated connections, not suitable for real-time interactions         | No streaming                      |


# Player to Player Chat
- Websocket is generally recommendedu7

# Message Serialization
Here, popular choises are:
- JSON: `serde` package
- MessagePack/Protobuf: for performance

# Authentication and Security
This relates to [Users, Sessions and Auth](../user_auth_session/6_users_sessions_auth.md). 
For simplicity we need message to be from authenticated users.

