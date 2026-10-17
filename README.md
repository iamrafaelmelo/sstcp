# SSTCP: Simple TCP Multi-Connection Server

A minimal TCP server in Rust that handles multiple concurrent connections using a **thread-per-connection** model. Built for learning purposes.

## Features

- Accepts multiple simultaneous TCP connections
- Spawns a dedicated OS thread per client
- Reads incoming messages and prints them with the client's address
- Gracefully handles client disconnection
- Custom domain error type with `Display`, `Error`, and bidirectional `From` conversions

## Requirements

- Rust 2024 Edition
- `cargo`

## Run

```bash
cargo run
```

Then connect with any TCP client:

```bash
nc localhost <port>
```

Type a message, press Enter, and watch the server print it. Close the connection with `Ctrl+C` or `exit`.

Open multiple `nc` sessions to see concurrent handling.

## Architecture

```
main thread
│
├── bind + listen
│
└── for each accepted connection:
        thread::spawn(move || handle_client(stream))
```

| Component | Responsibility |
|-----------|---------------|
| `main` | Binds the listener, accepts connections in a loop |
| `handle_client` | Reads bytes in a loop until `Ok(0)` (FIN) |
| `DomainError` | Domain-level error with human-readable messages |
| `DomainErrorKind` | Enum of specific failure modes |

## Key Concepts

- **Thread-per-connection** — one OS thread per client; simple but scales to ~hundreds of connections
- **`move` closures** — transfer ownership of `TcpStream` into spawned threads
- **`Ok(0)` from `Read`** — signals the peer sent a TCP FIN (graceful close)
- **`listener.incoming()`** — iterator that yields one `TcpStream` per accepted connection
- **Error conversion** — `From<DomainError> for io::Error` enables `?` to bridge domain and I/O error spaces
- **Orphan rule** — why you can `impl From<LocalType> for ForeignType` but not `impl ForeignTrait for ForeignType`

## Project Structure

```
├── Cargo.toml
└── src
    ├── main.rs
    └── errors
        └── app.rs
```

## Limitations (by design)

- No TLS
- No connection timeout
- No backpressure handling
- Not production-ready (educational only)
