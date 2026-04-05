# Chat Server - Rust (Siftach)

This project is a **learning-focused TCP chat server built in Rust**. It is designed as a **starting point** for a more complex distributed chat system.

---

## Features (Siftach / Initial Stage)

1. TCP server listening on `127.0.0.1:8080`
2. Accepts a single client connection
3. Reads messages from the client and prints to `stdout`
4. Echoes the message back to the client
5. Handles message boundaries using `\n`
6. Supports a simple JSON message protocol with fields:

```json
{ "type": "message", "user": "username", "text": "hello" }
```

---

## Requirements

- Rust >= 1.70
- Cargo
- Dependencies:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## Installation & Running

```bash
git clone <repo-url>
cd chat-server
cargo run
```

---

## Manual Testing

1. Open terminal 1 → run the server:
   ```bash
   cargo run
   ```
2. Open terminal 2 → connect as a client:
   ```bash
   telnet 127.0.0.1 8080
   ```
3. Send a JSON message:
   ```json
   { "type": "message", "user": "zviel", "text": "hello" }
   ```
4. Verify the server prints the message and the client receives the echo back.

---

## File Structure

```text
src/
 ├── main.rs       # Entry point
 ├── connection.rs # TcpStream management
 ├── protocol.rs   # Parsing and message definition
```

---

## Next Steps

- Multi-client support
- Async / Tokio
- Rooms / Channels
- Authentication
- TLS / Security
- Distributed nodes

---

## Rules for Siftach Stage

1. No async yet
2. Only valid JSON messages are accepted
3. Avoid unnecessary cloning
4. Document each module clearly

---

## How This Helps Learning Rust

- Understand ownership and borrowing
- Handle streaming data safely
- Manage TCP connections and message boundaries
- Set up modular architecture for future async and distributed features

