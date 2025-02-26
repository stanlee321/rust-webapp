use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use once_cell::sync::Lazy;

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

// Global state for mock data
pub static MOCK_DATA: Lazy<Arc<Mutex<MockData>>> = Lazy::new(|| {
    Arc::new(Mutex::new(MockData::new()))
});

pub struct MockData {
    users: HashMap<String, User>,
    cameras: HashMap<String, Camera>,
    activity_logs: Vec<ActivityLog>,
    reports: Vec<Report>,
    settings: Settings,
}

impl MockData {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        users.insert("1".to_string(), User {
            id: "1".to_string(),
            username: "admin".to_string(),
            name: "Admin User".to_string(),
            email: "admin@example.com".to_string(),
            role: UserRole::SuperAdmin,
            active: true,
            last_login: "2025-02-25T14:30:00Z".to_string(),
        });
        users.insert("2".to_string(), User {
            id: "2".to_string(),
            username: "jdoe".to_string(),
            name: "John Doe".to_string(),
            email: "jdoe@example.com".to_string(),
            role: UserRole::Admin,
            active: true,
            last_login: "2025-02-24T10:15:00Z".to_string(),
        });
        users.insert("3".to_string(), User {
            id: "3".to_string(),
            username: "asmith".to_string(),
            name: "Alice Smith".to_string(),
            email: "asmith@example.com".to_string(),
            role: UserRole::Viewer,
            active: true,
            last_login: "2025-02-23T09:45:00Z".to_string(),
        });

        let mut cameras = HashMap::new();
        cameras.insert("101".to_string(), Camera {
            id: "101".to_string(),
            name: "Front Gate".to_string(),
            ip_address: "192.168.1.101".to_string(),
            port: 554,
            location: "Main Entrance".to_string(),
            active: true,
            status: CameraStatus::Online,
            last_update: "2025-02-25T14:35:00Z".to_string(),
        });
        cameras.insert("102".to_string(), Camera {
            id: "102".to_string(),
            name: "Side Door".to_string(),
            ip_address: "192.168.1.102".to_string(),
            port: 554,
            location: "East Wing".to_string(),
            active: false,
            status: CameraStatus::Offline,
            last_update: "2025-02-25T10:20:00Z".to_string(),
        });
        cameras.insert("103".to_string(), Camera {
            id: "103".to_string(),
            name: "Parking Lot".to_string(),
            ip_address: "192.168.1.103".to_string(),
            port: 554,
            location: "North Side".to_string(),
            active: true,
            status: CameraStatus::Online,
            last_update: "2025-02-25T14:40:00Z".to_string(),
        });
        cameras.insert("104".to_string(), Camera {
            id: "104".to_string(),
            name: "Reception".to_string(),
            ip_address: "192.168.1.104".to_string(),
            port: 554,
            location: "Main Building".to_string(),
            active: true,
            status: CameraStatus::Maintenance,
            last_update: "2025-02-25T08:15:00Z".to_string(),
        });

        let activity_logs = vec![
            ActivityLog {
                id: "1001".to_string(),
                timestamp: "2025-02-25T14:30:00Z".to_string(),
                user_id: "1".to_string(),
                action: "EDIT_CAMERA".to_string(),
                target: "Camera 101".to_string(),
                details: "Changed name to 'Front Gate'".to_string(),
            },
            ActivityLog {
                id: "1002".to_string(),
                timestamp: "2025-02-25T13:45:00Z".to_string(),
                user_id: "2".to_string(),
                action: "DISABLE_CAMERA".to_string(),
                target: "Camera 102".to_string(),
                details: "Disabled for maintenance".to_string(),
            },
            ActivityLog {
                id: "1003".to_string(),
                timestamp: "2025-02-25T12:30:00Z".to_string(),
                user_id: "1".to_string(),
                action: "GENERATE_REPORT".to_string(),
                target: "UsageSummary".to_string(),
                details: "Generated monthly usage report".to_string(),
            },
            ActivityLog {
                id: "1004".to_string(),
                timestamp: "2025-02-25T11:15:00Z".to_string(),
                user_id: "1".to_string(),
                action: "CREATE_USER".to_string(),
                target: "User 3".to_string(),
                details: "Created user 'asmith'".to_string(),
            },
        ];

        let reports = vec![
            Report {
                id: "2001".to_string(),
                name: "Usage_Summary_Feb_2025".to_string(),
                type_: ReportType::UsageSummary,
                created_at: "2025-02-25T12:30:00Z".to_string(),
                created_by: "1".to_string(),
                period: "February 2025".to_string(),
                format: ReportFormat::PDF,
                url: "/api/reports/2001/download".to_string(),
            },
            Report {
                id: "2002".to_string(),
                name: "Camera_Status_Q1_2025".to_string(),
                type_: ReportType::CameraStatus,
                created_at: "2025-02-20T09:15:00Z".to_string(),
                created_by: "2".to_string(),
                period: "Q1 2025".to_string(),
                format: ReportFormat::CSV,
                url: "/api/reports/2002/download".to_string(),
            },
            Report {
                id: "2003".to_string(),
                name: "User_Activity_Jan_2025".to_string(),
                type_: ReportType::UserActivity,
                created_at: "2025-02-10T14:45:00Z".to_string(),
                created_by: "1".to_string(),
                period: "January 2025".to_string(),
                format: ReportFormat::PDF,
                url: "/api/reports/2003/download".to_string(),
            },
        ];

        let settings = Settings {
            registered_to: "Jane Doe".to_string(),
            server_status: true,
            api_url: "api.example.com".to_string(),
            license_expiry: "2025-12-31".to_string(),
            theme: "light".to_string(),
            email_alerts: true,
            sms_alerts: false,
            refresh_interval: 10,
            app_version: "1.0.0".to_string(),
        };

        MockData {
            users,
            cameras,
            activity_logs,
            reports,
            settings,
        }
    }
}

// Functions to access and manipulate mock data

// Users
pub fn get_users() -> Vec<User> {
    let mock_data = MOCK_DATA.lock().unwrap();
    mock_data.users.values().cloned().collect()
}

pub fn get_user(id: &str) -> Option<User> {
    let mock_data = MOCK_DATA.lock().unwrap();
    mock_data.users.get(id).cloned()
}

pub fn create_user(user: User) -> User {
    let mut mock_data = MOCK_DATA.lock().unwrap();
    mock_data.users.insert(user.id.clone(), user.clone());
    user
}

pub fn update_user(id: &str, user: User) -> Option<User> {
    let mut mock_data = MOCK_DATA.lock().unwrap();
    if mock_data.users.contains_key(id) {
        mock_data.users.insert(id.to_string(), user.clone());
        Some(user)
    } else {
        None
    }
}

pub fn delete_user(id: &str) -> bool {
    let mut mock_data = MOCK_DATA.lock().unwrap();
    mock_data.users.remove(id).is_some()
}

// Cameras
pub fn get_cameras() -> Vec<Camera> {
    let mock_data = MOCK_DATA.lock().unwrap();
    mock_data.cameras.values().cloned().collect()
}

pub fn get_camera(id: &str) -> Option<Camera> {
    let mock_data = MOCK_DATA.lock().unwrap();
    mock_data.cameras.get(id).cloned()
}

pub fn create_camera(camera: Camera) -> Camera {
    let mut mock_data = MOCK_DATA.lock().unwrap();
    mock_data.cameras.insert(camera.id.clone(), camera.clone());
    camera
}

pub fn update_camera(id: &str, camera: Camera) -> Option<Camera> {
    let mut mock_data = MOCK_DATA.lock().unwrap();
    if mock_data.cameras.contains_key(id) {
        mock_data.cameras.insert(id.to_string(), camera.clone());
        Some(camera)
    } else {
        None
    }
}

pub fn delete_camera(id: &str) -> bool {
    let mut mock_data = MOCK_DATA.lock().unwrap();
    mock_data.cameras.remove(id).is_some()
}

// Activity Logs
pub fn get_activity_logs() -> Vec<ActivityLog> {
    let mock_data = MOCK_DATA.lock().unwrap();
    mock_data.activity_logs.clone()
}

pub fn add_activity_log(log: ActivityLog) {
    let mut mock_data = MOCK_DATA.lock().unwrap();
    mock_data.activity_logs.push(log);
}

// Reports
pub fn get_reports() -> Vec<Report> {
    let mock_data = MOCK_DATA.lock().unwrap();
    mock_data.reports.clone()
}

pub fn get_report(id: &str) -> Option<Report> {
    let mock_data = MOCK_DATA.lock().unwrap();
    mock_data.reports.iter().find(|r| r.id == id).cloned()
}

pub fn add_report(report: Report) {
    let mut mock_data = MOCK_DATA.lock().unwrap();
    mock_data.reports.push(report);
}

// Settings
pub fn get_settings() -> Settings {
    let mock_data = MOCK_DATA.lock().unwrap();
    mock_data.settings.clone()
}

pub fn update_settings(settings: Settings) {
    let mut mock_data = MOCK_DATA.lock().unwrap();
    mock_data.settings = settings;
} 