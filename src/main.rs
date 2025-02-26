use axum::{
    routing::{get, post, put, delete, get_service},
    Router,
    response::Json,
    extract::{Path, State, Query},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use std::{net::{SocketAddr, IpAddr}, env, collections::HashMap};
use std::sync::{Arc, Mutex};

// Import our mock data module
mod mock_data;
use crate::mock_data::{
    User, Camera, ActivityLog, Report, ReportType, ReportFormat, Settings,
    get_users, get_user, create_user, update_user, delete_user,
    get_cameras, get_camera, create_camera, update_camera, delete_camera,
    get_activity_logs, add_activity_log,
    get_reports, get_report, add_report,
    get_settings, update_settings,
};

type AppState = Arc<Mutex<()>>;

#[tokio::main]
async fn main() {
    // Get frontend assets path from environment variable or use default
    let frontend_path = env::var("FRONTEND_PATH").unwrap_or_else(|_| "frontend/dist".to_string());
    println!("Using frontend assets from: {}", frontend_path);

    // Shared state (not used yet but prepared for future)
    let state = Arc::new(Mutex::new(()));

    // Create our API routes
    let api_routes = Router::new()
        // Authentication routes
        .route("/auth/login", post(login_handler))
        // User routes
        .route("/users", get(get_users_handler))
        .route("/users/:id", get(get_user_handler))
        .route("/users", post(create_user_handler))
        .route("/users/:id", put(update_user_handler))
        .route("/users/:id", delete(delete_user_handler))
        // Camera routes
        .route("/cameras", get(get_cameras_handler))
        .route("/cameras/:id", get(get_camera_handler))
        .route("/cameras", post(create_camera_handler))
        .route("/cameras/:id", put(update_camera_handler))
        .route("/cameras/:id", delete(delete_camera_handler))
        // Activity log routes
        .route("/logs", get(get_logs_handler))
        .route("/logs", post(create_log_handler))
        // Report routes
        .route("/reports", get(get_reports_handler))
        .route("/reports/:id", get(get_report_handler))
        .route("/reports", post(create_report_handler))
        // Settings routes
        .route("/settings", get(get_settings_handler))
        .route("/settings", put(update_settings_handler))
        // Legacy routes for backwards compatibility
        .route("/hello", get(hello_handler))
        .route("/hello/:name", get(hello_name_handler))
        .with_state(state);

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

// Authentication handlers
#[derive(Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct LoginResponse {
    token: String,
    user: User,
}

async fn login_handler(Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>, StatusCode> {
    // In a real app, we would validate credentials against a database
    // For this mock implementation, we'll accept any login with a password "password"
    if payload.password != "password" {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    // Find the user by username
    let users = get_users();
    if let Some(user) = users.into_iter().find(|u| u.username == payload.username) {
        // In a real app, we would generate a proper JWT token
        // For now, just use a simple token
        let token = format!("mock-token-{}", user.id);
        
        Ok(Json(LoginResponse {
            token,
            user,
        }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

// User handlers
async fn get_users_handler(_state: State<AppState>) -> Json<Vec<User>> {
    Json(get_users())
}

async fn get_user_handler(Path(id): Path<String>, _state: State<AppState>) -> Result<Json<User>, StatusCode> {
    get_user(&id).map(Json).ok_or(StatusCode::NOT_FOUND)
}

async fn create_user_handler(
    _state: State<AppState>,
    Json(user): Json<User>,
) -> Json<User> {
    // In a real app, we'd generate an ID, but for the mock we'll use the provided one
    Json(create_user(user))
}

async fn update_user_handler(
    Path(id): Path<String>,
    _state: State<AppState>,
    Json(user): Json<User>,
) -> Result<Json<User>, StatusCode> {
    update_user(&id, user).map(Json).ok_or(StatusCode::NOT_FOUND)
}

async fn delete_user_handler(
    Path(id): Path<String>,
    _state: State<AppState>,
) -> StatusCode {
    if delete_user(&id) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// Camera handlers
async fn get_cameras_handler(_state: State<AppState>) -> Json<Vec<Camera>> {
    Json(get_cameras())
}

async fn get_camera_handler(
    Path(id): Path<String>,
    _state: State<AppState>,
) -> Result<Json<Camera>, StatusCode> {
    get_camera(&id).map(Json).ok_or(StatusCode::NOT_FOUND)
}

async fn create_camera_handler(
    _state: State<AppState>,
    Json(camera): Json<Camera>,
) -> Json<Camera> {
    Json(create_camera(camera))
}

async fn update_camera_handler(
    Path(id): Path<String>,
    _state: State<AppState>,
    Json(camera): Json<Camera>,
) -> Result<Json<Camera>, StatusCode> {
    update_camera(&id, camera).map(Json).ok_or(StatusCode::NOT_FOUND)
}

async fn delete_camera_handler(
    Path(id): Path<String>,
    _state: State<AppState>,
) -> StatusCode {
    if delete_camera(&id) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// Activity Log handlers
async fn get_logs_handler(_state: State<AppState>) -> Json<Vec<ActivityLog>> {
    Json(get_activity_logs())
}

async fn create_log_handler(
    _state: State<AppState>,
    Json(log): Json<ActivityLog>,
) -> StatusCode {
    add_activity_log(log);
    StatusCode::CREATED
}

// Report handlers
async fn get_reports_handler(_state: State<AppState>) -> Json<Vec<Report>> {
    Json(get_reports())
}

async fn get_report_handler(
    Path(id): Path<String>,
    _state: State<AppState>,
) -> Result<Json<Report>, StatusCode> {
    get_report(&id).map(Json).ok_or(StatusCode::NOT_FOUND)
}

async fn create_report_handler(
    _state: State<AppState>,
    Json(report): Json<Report>,
) -> StatusCode {
    add_report(report);
    StatusCode::CREATED
}

// Settings handlers
async fn get_settings_handler(_state: State<AppState>) -> Json<Settings> {
    Json(get_settings())
}

async fn update_settings_handler(
    _state: State<AppState>,
    Json(settings): Json<Settings>,
) -> StatusCode {
    update_settings(settings);
    StatusCode::OK
}

// Legacy API handlers that we're keeping for backwards compatibility
#[derive(Serialize, Deserialize)]
struct HelloResponse {
    message: String,
}

async fn hello_handler() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello, world!".to_string(),
    })
}

async fn hello_name_handler(Path(name): Path<String>) -> Json<HelloResponse> {
    Json(HelloResponse {
        message: format!("Hello, {}!", name),
    })
}
