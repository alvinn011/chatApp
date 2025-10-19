# Minimal Chat App

A lightweight real-time chat application built with **Rust (Rocket)** and **SSE** for message streaming.

---

## Features

- Real-time chat using Server-Sent Events (SSE)
- Simple and minimal UI
- Health check endpoint
- Username support

---

## Installation

1. **Clone the repository:**

```bash
git clone https://github.com/alvinn011/chatApp.git
cd chatApp
```

2. **Build and run with Cargo:**

```bash
cargo run --release
```

3. **Open in your browser:**

```
http://127.0.0.1:8001/
```

## Configuration

> **IMPORTANT:** The app ships with a default secret key.  
> **Please change it before running in production!** 

1. **Generate new key:**

```bash
openssl rand -hex 32
```

2. **Change the key:**
You can set the secret key via the 'Rocket.toml' file:

```toml
secret_key = "..."
```

