# Rust HTTPX App

A simple Rust web application with an HTTP server that provides both a web UI and REST API endpoints.

## Features

- Web UI showing "Hello World" with a modern interface
- REST API endpoints:
  - GET `/api/hello`: Returns a "Hello, world!" message
  - GET `/api/hello/:name`: Returns a personalized greeting with the provided name

## Requirements

- Rust (latest stable version)
- Cargo (included with Rust)

## Setup

1. Clone this repository
2. Navigate to the project directory
3. Run the application:

```bash
cargo run
```

The server will start on http://127.0.0.1:3000

## Project Structure

- `src/main.rs`: Main application code with server implementation
- `static/`: Directory for static web files (created automatically on first run)
- `Cargo.toml`: Project configuration and dependencies

## API Endpoints

### GET `/api/hello`

Returns a JSON response with a "Hello, world!" message.

Example response:
```json
{
  "message": "Hello, world!"
}
```

### GET `/api/hello/:name`

Returns a JSON response with a personalized greeting.

Example request: `/api/hello/John`
Example response:
```json
{
  "message": "Hello, John!"
}
```

## Web UI

The web UI is available at the root URL (http://127.0.0.1:3000) and provides a simple interface to test the API endpoints. 