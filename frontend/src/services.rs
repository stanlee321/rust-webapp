use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::UseStateHandle;

// Response type for errors
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
}

// User Models
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
    pub email: String,
    pub role: UserRole,
    pub active: bool,
    pub last_login: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    SuperAdmin,
    Admin,
    Viewer,
}

// Camera Models
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Camera {
    pub id: String,
    pub name: String,
    pub ip_address: String,
    pub port: u16,
    pub location: String,
    pub active: bool,
    pub status: CameraStatus,
    pub last_update: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CameraStatus {
    Online,
    Offline,
    Maintenance,
}

// Activity Log Model
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActivityLog {
    pub id: String,
    pub timestamp: String,
    pub user_id: String,
    pub action: String,
    pub target: String,
    pub details: String,
}

// Report Models
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    pub id: String,
    pub name: String,
    pub type_: ReportType,
    pub created_at: String,
    pub created_by: String,
    pub period: String,
    pub format: ReportFormat,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ReportType {
    UsageSummary,
    CameraStatus,
    UserActivity,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ReportFormat {
    PDF,
    CSV,
}

// Settings Model
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub registered_to: String,
    pub server_status: bool,
    pub api_url: String,
    pub license_expiry: String,
    pub theme: String,
    pub email_alerts: bool,
    pub sms_alerts: bool,
    pub refresh_interval: u32,
    pub app_version: String,
}

// Authentication types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

// API Service methods
pub async fn login(username: &str, password: &str) -> Result<LoginResponse, String> {
    let request = LoginRequest {
        username: username.to_string(),
        password: password.to_string(),
    };

    let response = Request::post("/api/auth/login")
        .json(&request)
        .expect("Failed to serialize JSON")
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<LoginResponse>().await {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("Failed to parse response: {}", err)),
                }
            } else {
                Err("Invalid credentials".to_string())
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

// User Service methods
pub async fn get_users() -> Result<Vec<User>, String> {
    let response = Request::get("/api/users")
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Vec<User>>().await {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("Failed to parse response: {}", err)),
                }
            } else {
                Err("Failed to get users".to_string())
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

pub async fn get_user(id: &str) -> Result<User, String> {
    let response = Request::get(&format!("/api/users/{}", id))
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<User>().await {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("Failed to parse response: {}", err)),
                }
            } else {
                Err("User not found".to_string())
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

pub async fn create_user(user: &User) -> Result<User, String> {
    let response = Request::post("/api/users")
        .json(user)
        .expect("Failed to serialize JSON")
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<User>().await {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("Failed to parse response: {}", err)),
                }
            } else {
                Err("Failed to create user".to_string())
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

pub async fn update_user(id: &str, user: &User) -> Result<User, String> {
    let response = Request::put(&format!("/api/users/{}", id))
        .json(user)
        .expect("Failed to serialize JSON")
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<User>().await {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("Failed to parse response: {}", err)),
                }
            } else {
                Err("Failed to update user".to_string())
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

pub async fn delete_user(id: &str) -> Result<(), String> {
    let response = Request::delete(&format!("/api/users/{}", id))
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status() == 204 {
                Ok(())
            } else {
                Err("Failed to delete user".to_string())
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

// Camera Service methods
pub async fn get_cameras() -> Result<Vec<Camera>, String> {
    let response = Request::get("/api/cameras")
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Vec<Camera>>().await {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("Failed to parse response: {}", err)),
                }
            } else {
                Err("Failed to get cameras".to_string())
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

pub async fn get_camera(id: &str) -> Result<Camera, String> {
    let response = Request::get(&format!("/api/cameras/{}", id))
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Camera>().await {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("Failed to parse response: {}", err)),
                }
            } else {
                Err("Camera not found".to_string())
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

// Activity Log Service methods
pub async fn get_logs() -> Result<Vec<ActivityLog>, String> {
    let response = Request::get("/api/logs")
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Vec<ActivityLog>>().await {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("Failed to parse response: {}", err)),
                }
            } else {
                Err("Failed to get logs".to_string())
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

// Report Service methods
pub async fn get_reports() -> Result<Vec<Report>, String> {
    let response = Request::get("/api/reports")
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Vec<Report>>().await {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("Failed to parse response: {}", err)),
                }
            } else {
                Err("Failed to get reports".to_string())
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

pub async fn get_report(id: &str) -> Result<Report, String> {
    let response = Request::get(&format!("/api/reports/{}", id))
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Report>().await {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("Failed to parse response: {}", err)),
                }
            } else {
                Err("Report not found".to_string())
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

// Settings Service methods
pub async fn get_settings() -> Result<Settings, String> {
    let response = Request::get("/api/settings")
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Settings>().await {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("Failed to parse response: {}", err)),
                }
            } else {
                Err("Failed to get settings".to_string())
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

pub async fn update_settings(settings: &Settings) -> Result<(), String> {
    let response = Request::put("/api/settings")
        .json(settings)
        .expect("Failed to serialize JSON")
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status() == 200 {
                Ok(())
            } else {
                Err("Failed to update settings".to_string())
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

// Helper function to fetch data and update state
pub fn fetch_data<T, F, S>(
    fetch_fn: F,
    state: UseStateHandle<Option<T>>,
    error_state: UseStateHandle<Option<String>>,
) where
    F: FnOnce() -> S + 'static,
    S: std::future::Future<Output = Result<T, String>> + 'static,
    T: Clone + 'static,
{
    let state = state.clone();
    let error_state = error_state.clone();
    
    // Clear previous error
    error_state.set(None);
    
    spawn_local(async move {
        match fetch_fn().await {
            Ok(data) => {
                state.set(Some(data));
            }
            Err(err) => {
                error_state.set(Some(err));
            }
        }
    });
} 