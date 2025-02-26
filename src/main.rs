use axum::{
    routing::{get, get_service},
    Router,
    response::Json,
    extract::Path,
};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use std::{net::{SocketAddr, IpAddr}, env};

#[tokio::main]
async fn main() {
    // Get frontend assets path from environment variable or use default
    let frontend_path = env::var("FRONTEND_PATH").unwrap_or_else(|_| "frontend/dist".to_string());
    println!("Using frontend assets from: {}", frontend_path);

    // Create our API routes
    let api_routes = Router::new()
        .route("/hello", get(hello_handler))
        .route("/hello/:name", get(hello_name_handler));

    // Create a combined router for static files and API
    let app = Router::new()
        .nest("/api", api_routes)
        // Serve the Yew app using the configured path
        .nest_service("/", get_service(ServeDir::new(&frontend_path)))
        .fallback_service(get_service(ServeDir::new(&frontend_path)));

    // Try to get host from environment variable, or use default
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let host_addr: IpAddr = host.parse().unwrap_or_else(|_| "127.0.0.1".parse().unwrap());
    
    // Try to get port from environment variable, or use a list of ports to try
    let port = env::var("PORT").ok().and_then(|p| p.parse::<u16>().ok()).unwrap_or(0);
    
    // If port is 0, we'll try a sequence of ports
    let ports_to_try = if port == 0 {
        vec![3000, 3001, 3002, 3003, 8000, 8080]
    } else {
        vec![port]
    };

    // Try each port in the list
    for port in ports_to_try {
        let addr = SocketAddr::new(host_addr, port);
        
        match axum::Server::try_bind(&addr) {
            Ok(server) => {
                println!("Server started on http://{}", addr);
                server.serve(app.into_make_service()).await.unwrap();
                return; // Exit the loop if server starts successfully
            },
            Err(e) => {
                eprintln!("Failed to bind to {}:{}: {}", host, port, e);
                // Continue to the next port
            }
        }
    }
    
    // If we get here, all ports failed
    eprintln!("Failed to bind to any port. Please free up a port or specify a different one using the PORT environment variable.");
    std::process::exit(1);
}

// Simple response type for our API
#[derive(Serialize, Deserialize)]
struct HelloResponse {
    message: String,
}

// API handler for /api/hello
async fn hello_handler() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello, world!".to_string(),
    })
}

// API handler for /api/hello/:name
async fn hello_name_handler(Path(name): Path<String>) -> Json<HelloResponse> {
    Json(HelloResponse {
        message: format!("Hello, {}!", name),
    })
}
