mod services;

use chrono::{DateTime, Local, Utc};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::prelude::JsValue;
use web_sys::{HtmlInputElement, HtmlSelectElement, Document, MouseEvent, Window, Event, FocusEvent, SubmitEvent};
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Math;
use crate::services::{
    User, Camera, ActivityLog, Report, Settings, ReportType, ReportFormat, CameraStatus, UserRole,
    get_users, get_cameras, get_logs, get_reports, get_settings, 
    update_camera, create_camera, delete_camera,
    login,
    fetch_data
};
use wasm_bindgen_futures;
use js_sys::Date;

#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    Home,
    Login,
}

impl Default for Route {
    fn default() -> Self {
        Self::Home
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Page {
    Home,
    Users,
    Cameras,
    Reports,
    Logs,
    Settings,
}

impl Page {
    pub fn to_string(&self) -> String {
        match self {
            Page::Home => "Dashboard".to_string(),
            Page::Users => "Users".to_string(),
            Page::Cameras => "Cameras".to_string(),
            Page::Reports => "Reports".to_string(),
            Page::Logs => "Activity Logs".to_string(),
            Page::Settings => "Settings".to_string(),
        }
    }
    
    pub fn icon(&self) -> String {
        match self {
            Page::Home => "fa-home".to_string(),
            Page::Users => "fa-users".to_string(),
            Page::Cameras => "fa-video".to_string(),
            Page::Reports => "fa-chart-bar".to_string(),
            Page::Logs => "fa-history".to_string(),
            Page::Settings => "fa-cog".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Deserialize)]
struct HelloResponse {
    message: String,
}

// Header Component
#[derive(Properties, PartialEq)]
struct HeaderProps {
    is_drawer_open: bool,
    is_dark_mode: bool,
    username: String,
    role: String,
    current_page: Page,
    on_toggle_drawer: Callback<MouseEvent>,
    on_toggle_dark_mode: Callback<MouseEvent>,
    on_logout: Callback<MouseEvent>,
}

#[function_component(Header)]
fn header(props: &HeaderProps) -> Html {
    let user_menu_open = use_state(|| false);
    
    let toggle_user_menu = {
        let user_menu_open = user_menu_open.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            user_menu_open.set(!*user_menu_open);
        })
    };
    
    html! {
        <header class="app-header">
            <div class="header-left">
                <button class="menu-button" onclick={props.on_toggle_drawer.clone()}>
                    {
                        if props.is_drawer_open {
                            "✕"
                        } else {
                            "☰"
                        }
                    }
                </button>
                <div class="app-logo">{"Cam Admin"}</div>
                <h1 class="header-title">{props.current_page.to_string()}</h1>
            </div>
            
            <div class="header-right">
                <button class="icon-button" onclick={props.on_toggle_dark_mode.clone()}>
                    {
                        if props.is_dark_mode {
                            "☀️"
                        } else {
                            "🌙"
                        }
                    }
                </button>
                
                <div class="user-menu-container">
                    <button class="user-button" onclick={toggle_user_menu.clone()}>
                        {"👤 "}
                        {&props.username}
                        {" ▼"}
                    </button>
                    
                    {
                        if *user_menu_open {
                            html! {
                                <div class="user-dropdown">
                                    <button onclick={
                                        let callback = props.on_logout.clone();
                                        Callback::from(move |e: MouseEvent| {
                                            callback.emit(e);
                                        })
                                    }>
                                        {"Logout"}
                                    </button>
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
            </div>
        </header>
    }
}

// Navigation Drawer Component
#[derive(Properties, PartialEq)]
struct NavigationDrawerProps {
    current_page: Page,
    is_open: bool,
    username: String,
    role: String,
    on_nav: Callback<Page>,
}

#[function_component(NavigationDrawer)]
fn navigation_drawer(props: &NavigationDrawerProps) -> Html {
    let drawer_class = if props.is_open { "drawer open" } else { "drawer" };
    
    let pages = vec![
        Page::Home,
        Page::Users,
        Page::Cameras,
        Page::Reports,
        Page::Logs,
        Page::Settings,
    ];
    
    html! {
        <div class={drawer_class}>
            <div class="drawer-header">
                <div class="drawer-user">
                    <div class="user-avatar">{"👤"}</div>
                    <div>
                        <div class="user-name">{&props.username}</div>
                        <div class="user-role">{&props.role}</div>
                    </div>
                </div>
            </div>
            
            <nav class="drawer-nav">
                <ul>
                    {
                        pages.iter().map(|page| {
                            let page_clone = page.clone();
                            let nav_callback = props.on_nav.clone();
                            
                            let class = if *page == props.current_page { "nav-item active" } else { "nav-item" };
                            
                            html! {
                                <li class={class} onclick={
                                    Callback::from(move |_| {
                                        nav_callback.emit(page_clone.clone());
                                    })
                                }>
                                    <span class="nav-icon">{page.icon()}</span>
                                    <span>{page.to_string()}</span>
                                </li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            </nav>
            
            <div class="drawer-footer">
                {"App Version: 1.0.0"}
            </div>
        </div>
    }
}

// Footer Component
#[function_component(Footer)]
fn footer() -> Html {
    html! {
        <footer class="app-footer">
            <div>{"© 2025 Camera Admin Inc."}</div>
            <div class="footer-links">
                <a href="#">{"Help"}</a>
                <a href="#">{"Privacy"}</a>
                <a href="#">{"Terms"}</a>
            </div>
        </footer>
    }
}

fn toggle_drawer() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    
    let class_name = body.class_name();
    if class_name.contains("drawer-open") {
        body.set_class_name(&class_name.replace("drawer-open", "").trim());
    } else {
        body.set_class_name(&format!("{} drawer-open", class_name).trim());
    }
}

#[function_component(App)]
fn app() -> Html {
    let current_page = use_state(|| Page::Home);
    let drawer_open = use_state(|| false);
    let dark_mode = use_state(|| false);
    
    // User state would typically come from authentication service
    let username = use_state(|| Some("Admin".to_string()));
    let role = use_state(|| Some("Admin".to_string()));
    
    // Data states
    let users = use_state(|| None);
    let cameras = use_state(|| None);
    let logs = use_state(|| None);
    let reports = use_state(|| None);
    let settings = use_state(|| None);
    
    // Load data effect
    {
        let users = users.clone();
        let cameras = cameras.clone();
        let logs = logs.clone();
        let reports = reports.clone();
        let settings = settings.clone();
        
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    // In a real app, these would be API calls
                    // For now we'll use our mock service functions
                    match get_users().await {
                        Ok(data) => users.set(Some(data)),
                        Err(e) => log::error!("Failed to load users: {:?}", e),
                    }
                    
                    match get_cameras().await {
                        Ok(data) => cameras.set(Some(data)),
                        Err(e) => log::error!("Failed to load cameras: {:?}", e),
                    }
                    
                    match get_logs().await {
                        Ok(data) => logs.set(Some(data)),
                        Err(e) => log::error!("Failed to load logs: {:?}", e),
                    }
                    
                    match get_reports().await {
                        Ok(data) => reports.set(Some(data)),
                        Err(e) => log::error!("Failed to load reports: {:?}", e),
                    }
                    
                    match get_settings().await {
                        Ok(data) => settings.set(Some(data)),
                        Err(e) => log::error!("Failed to load settings: {:?}", e),
                    }
                });
                || ()
            },
            (),
        );
    }
    
    let toggle_drawer_callback = {
        let drawer_open = drawer_open.clone();
        Callback::from(move |_| {
            toggle_drawer();
            drawer_open.set(!*drawer_open);
        })
    };
    
    let toggle_dark_mode = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |_| {
            let document = window().document().unwrap();
            let body = document.body().unwrap();
            
            let class_name = body.class_name();
            if *dark_mode {
                body.set_class_name(&class_name.replace("dark-theme", "").trim());
                dark_mode.set(false);
            } else {
                body.set_class_name(&format!("{} dark-theme", class_name).trim());
                dark_mode.set(true);
            }
        })
    };
    
    let _handle_logout = {
        Callback::from(move |_: MouseEvent| {
            // In a real app, this would call a logout API
            log::info!("User logged out");
        })
    };
    
    let on_page_change = {
        let current_page = current_page.clone();
        let drawer_open = drawer_open.clone();
        
        Callback::from(move |page: Page| {
            current_page.set(page);
            // On mobile, close the drawer after selection
            let window = web_sys::window().unwrap();
            if window.inner_width().unwrap().as_f64().unwrap() < 768.0 {
                toggle_drawer();
            }
        })
    };
    
    let container_class = if *drawer_open {
        "app-container drawer-open"
    } else {
        "app-container"
    };
    
    // Apply dark theme to document.body if dark mode is active
    if *dark_mode {
        document().body().unwrap().set_class_name("dark-theme");
    } else {
        document().body().unwrap().set_class_name("");
    }
    
    html! {
        <div class={container_class}>
            <header class="app-header">
                <button 
                    class="icon-button menu-button" 
                    onclick={toggle_drawer_callback}
                >
                    <i class="fas fa-bars"></i>
                </button>
                <div class="header-title">
                    {"LucaM Camera Management System"}
                </div>
                <div class="header-actions">
                    <button 
                        class="icon-button theme-toggle" 
                        onclick={toggle_dark_mode.clone()}
                    >
                        <i class={if *dark_mode { "fas fa-sun" } else { "fas fa-moon" }}></i>
                    </button>
                    <div class="user-avatar">
                        {username.as_ref().map_or("G", |name| &name[0..1])}
                    </div>
                </div>
            </header>
            
            <div class="main-container">
                <nav class={if *drawer_open { "navigation-drawer open" } else { "navigation-drawer" }}>
                    <div class="drawer-header">
                        <div class="drawer-user-info">
                            <div class="drawer-user-name">
                                {username.as_ref().map_or("Guest".to_string(), |name| name.clone())}
                            </div>
                            <div class="drawer-user-role">
                                {role.as_ref().map_or("Viewer".to_string(), |r| r.clone())}
                            </div>
                        </div>
                    </div>
                    <div class="nav-list">
                        {
                            [
                                Page::Home,
                                Page::Users,
                                Page::Cameras,
                                Page::Reports,
                                Page::Logs,
                                Page::Settings,
                            ].iter().map(|page| {
                                let is_active = *current_page == *page;
                                let page_clone = page.clone();
                                let onclick = {
                                    let on_page_change = on_page_change.clone();
                                    let page = page.clone();
                                    Callback::from(move |_| {
                                        on_page_change.emit(page.clone());
                                    })
                                };
                                
                                html! {
                                    <div 
                                        class={if is_active { "nav-item active" } else { "nav-item" }}
                                        onclick={onclick}
                                    >
                                        <span class="nav-item-icon">
                                            <i class={format!("fas {}", page_clone.icon())}></i>
                                        </span>
                                        {page_clone.to_string()}
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </nav>
                
                <main class="page-content">
                    {render_page(
                        &*current_page,
                        users.clone(),
                        cameras.clone(),
                        logs.clone(),
                        reports.clone(),
                        settings.clone(),
                    )}
                </main>
            </div>
            
            <footer class="footer">
                <div class="footer-copyright">
                    {"© 2025 LucaM Camera Management System"}
                </div>
                <div class="footer-links">
                    <a href="#">{"Help"}</a>
                    <a href="#">{"Privacy"}</a>
                    <a href="#">{"Terms"}</a>
                </div>
            </footer>
        </div>
    }
}

fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> Document {
    window().document().expect("Failed to get document")
}

fn get_input_value(id: &str) -> String {
    if let Some(element) = document().get_element_by_id(id) {
        // Try to get value from input element
        if let Ok(input) = element.clone().dyn_into::<HtmlInputElement>() {
            return input.value();
        }
        
        // Try to get value from select element
        if let Ok(select) = element.dyn_into::<HtmlSelectElement>() {
            return select.value();
        }
    }
    
    String::new()
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

// Function to render the current page based on selection
fn render_page(
    current_page: &Page,
    users: UseStateHandle<Option<Vec<User>>>,
    cameras: UseStateHandle<Option<Vec<Camera>>>,
    logs: UseStateHandle<Option<Vec<ActivityLog>>>,
    reports: UseStateHandle<Option<Vec<Report>>>,
    settings: UseStateHandle<Option<Settings>>,
) -> Html {
    match current_page {
        Page::Home => {
            let camera_count = cameras.as_ref().map_or(0, |c| c.len());
            let online_camera_count = cameras.as_ref().map_or(0, |c| 
                c.iter().filter(|cam| cam.status == CameraStatus::Online).count()
            );
            let user_count = users.as_ref().map_or(0, |u| u.len());
            
            html! {
                <div class="dashboard-page">
                    <h2>{"Dashboard"}</h2>
                    
                    <div class="dashboard-summary">
                        <div class="summary-card">
                            <div class="card-title">{"Total Cameras"}</div>
                            <div class="card-value">{camera_count}</div>
                            <div class="card-details">{online_camera_count} {" online"}</div>
                        </div>
                        
                        <div class="summary-card">
                            <div class="card-title">{"Registered Users"}</div>
                            <div class="card-value">{user_count}</div>
                            <div class="card-details">{"Active accounts"}</div>
                        </div>
                        
                        <div class="summary-card">
                            <div class="card-title">{"Latest Activity"}</div>
                            <div class="card-value">
                                {logs.as_ref().map_or(0, |l| l.len())}
                            </div>
                            <div class="card-details">{"Recent actions"}</div>
                        </div>
                    </div>
                    
                    <div class="dashboard-content">
                        <div class="widget">
                            <h3>{"Camera Status"}</h3>
                            {
                                if let Some(camera_list) = cameras.as_ref() {
                                    html! {
                                        <table class="dashboard-table">
                                            <thead>
                                                <tr>
                                                    <th>{"Name"}</th>
                                                    <th>{"Status"}</th>
                                                    <th>{"Location"}</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {
                                                    camera_list.iter().take(5).map(|camera| {
                                                        let status_class = match camera.status {
                                                            CameraStatus::Online => "status-online",
                                                            CameraStatus::Offline => "status-offline",
                                                            CameraStatus::Maintenance => "status-maintenance",
                                                        };
                                                        
                                                        let status_text = match camera.status {
                                                            CameraStatus::Online => "Online",
                                                            CameraStatus::Offline => "Offline",
                                                            CameraStatus::Maintenance => "Maintenance",
                                                        };
                                                        
                                                        html! {
                                                            <tr key={camera.id.clone()}>
                                                                <td>{&camera.name}</td>
                                                                <td>
                                                                    <span class={format!("status-indicator {}", status_class)}>
                                                                        {status_text}
                                                                    </span>
                                                                </td>
                                                                <td>{&camera.location}</td>
                                                            </tr>
                                                        }
                                                    }).collect::<Html>()
                                                }
                                            </tbody>
                                        </table>
                                    }
                                } else {
                                    html! { <div class="loading-container">{"Loading cameras..."}</div> }
                                }
                            }
                        </div>
                        
                        <div class="widget">
                            <h3>{"Recent Activity"}</h3>
                            {
                                if let Some(log_list) = logs.as_ref() {
                                    html! {
                                        <table class="dashboard-table">
                                            <thead>
                                                <tr>
                                                    <th>{"Action"}</th>
                                                    <th>{"Target"}</th>
                                                    <th>{"Time"}</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {
                                                    log_list.iter().take(5).map(|log| {
                                                        html! {
                                                            <tr key={log.id.clone()}>
                                                                <td>{&log.action}</td>
                                                                <td>{&log.target}</td>
                                                                <td>{&log.timestamp}</td>
                                                            </tr>
                                                        }
                                                    }).collect::<Html>()
                                                }
                                            </tbody>
                                        </table>
                                    }
                                } else {
                                    html! { <div class="loading-container">{"Loading activity logs..."}</div> }
                                }
                            }
                        </div>
                    </div>
                </div>
            }
        },
        Page::Users => {
            html! {
                <div class="users-page">
                    <div class="page-header">
                        <h2>{"User Management"}</h2>
                        <button class="primary-button">{"Add User"}</button>
                    </div>
                    
                    {
                        if let Some(user_list) = users.as_ref() {
                            html! {
                                <table class="data-table">
                                    <thead>
                                        <tr>
                                            <th>{"Name"}</th>
                                            <th>{"Email"}</th>
                                            <th>{"Role"}</th>
                                            <th>{"Status"}</th>
                                            <th>{"Last Login"}</th>
                                            <th>{"Actions"}</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {
                                            user_list.iter().map(|user| {
                                                let role_text = match user.role {
                                                    UserRole::SuperAdmin => "Super Admin",
                                                    UserRole::Admin => "Admin",
                                                    UserRole::Viewer => "Viewer",
                                                };
                                                
                                                let status_class = if user.active { "status-online" } else { "status-offline" };
                                                let status_text = if user.active { "Active" } else { "Inactive" };
                                                
                                                html! {
                                                    <tr key={user.id.clone()}>
                                                        <td>{&user.name}</td>
                                                        <td>{&user.email}</td>
                                                        <td>{role_text}</td>
                                                        <td><span class={format!("status-indicator {}", status_class)}>{status_text}</span></td>
                                                        <td>{&user.last_login}</td>
                                                        <td>
                                                            <button class="action-button">{"Edit"}</button>
                                                            <button class="action-button danger">{"Delete"}</button>
                                                        </td>
                                                    </tr>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </tbody>
                                </table>
                            }
                        } else {
                            html! { <div class="loading-container">{"Loading users..."}</div> }
                        }
                    }
                </div>
            }
        },
        Page::Cameras => {
            // Simple Camera page implementation without state management challenges
            let cameras_list = mock_cameras();
                
            html! {
                <div class="cameras-page">
                    <div class="page-header">
                        <h2>{"Camera Management"}</h2>
                        <button class="primary-button">{"Add Camera"}</button>
                    </div>
                    
                    <table class="data-table">
                        <thead>
                            <tr>
                                <th>{"Name"}</th>
                                <th>{"IP Address"}</th>
                                <th>{"Location"}</th>
                                <th>{"Status"}</th>
                                <th>{"Last Update"}</th>
                                <th>{"Actions"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {
                                cameras_list.iter().map(|camera| {
                                    let status_class = match camera.status {
                                        CameraStatus::Online => "status-online",
                                        CameraStatus::Offline => "status-offline",
                                        CameraStatus::Maintenance => "status-maintenance",
                                    };
                                    
                                    let status_text = match camera.status {
                                        CameraStatus::Online => "Online",
                                        CameraStatus::Offline => "Offline",
                                        CameraStatus::Maintenance => "Maintenance",
                                    };
                                    
                                    html! {
                                        <tr key={camera.id.clone()}>
                                            <td>{&camera.name}</td>
                                            <td>{format!("{}:{}", camera.ip_address, camera.port)}</td>
                                            <td>{&camera.location}</td>
                                            <td>
                                                <span class={format!("status-indicator {}", status_class)}>
                                                    {status_text}
                                                </span>
                                            </td>
                                            <td>{&camera.last_update}</td>
                                            <td class="action-buttons">
                                                <button class="action-button">{"View"}</button>
                                                <button class="action-button">{"Edit"}</button>
                                                <button class="action-button danger">{"Delete"}</button>
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Html>()
                            }
                        </tbody>
                    </table>
                </div>
            }
        },
        Page::Reports => {
            html! {
                <div class="reports-page">
                    <h2>{"Reports"}</h2>
                    
                    <div class="widget">
                        <h3>{"Generate New Report"}</h3>
                        <div class="form-group">
                            <label for="report-type">{"Report Type"}</label>
                            <select id="report-type">
                                <option value="usage">{"Usage Summary"}</option>
                                <option value="camera-status">{"Camera Status"}</option>
                                <option value="user-activity">{"User Activity"}</option>
                            </select>
                        </div>
                        
                        <div class="form-group">
                            <label>{"Period"}</label>
                            <div>
                                <div class="form-group">
                                    <label for="date-from">{"From"}</label>
                                    <input type="date" id="date-from" />
                                </div>
                                <div class="form-group">
                                    <label for="date-to">{"To"}</label>
                                    <input type="date" id="date-to" />
                                </div>
                            </div>
                        </div>
                        
                        <div class="form-group">
                            <label>{"Format"}</label>
                            <div class="radio-group">
                                <label>
                                    <input type="radio" name="format" value="pdf" checked=true />
                                    {"PDF"}
                                </label>
                                <label>
                                    <input type="radio" name="format" value="csv" />
                                    {"CSV"}
                                </label>
                            </div>
                        </div>
                        
                        <div>
                            <button class="primary-button">{"Generate Report"}</button>
                        </div>
                    </div>
                    
                    <h3>{"Previous Reports"}</h3>
                    {
                        if let Some(report_list) = reports.as_ref() {
                            html! {
                                <table class="data-table">
                                    <thead>
                                        <tr>
                                            <th>{"Name"}</th>
                                            <th>{"Type"}</th>
                                            <th>{"Created"}</th>
                                            <th>{"Period"}</th>
                                            <th>{"Format"}</th>
                                            <th>{"Actions"}</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {
                                            report_list.iter().map(|report| {
                                                let type_text = match report.type_ {
                                                    ReportType::UsageSummary => "Usage Summary",
                                                    ReportType::CameraStatus => "Camera Status",
                                                    ReportType::UserActivity => "User Activity",
                                                };
                                                
                                                let format_text = match report.format {
                                                    ReportFormat::PDF => "PDF",
                                                    ReportFormat::CSV => "CSV",
                                                };
                                                
                                                html! {
                                                    <tr key={report.id.clone()}>
                                                        <td>{&report.name}</td>
                                                        <td>{type_text}</td>
                                                        <td>{&report.created_at}</td>
                                                        <td>{&report.period}</td>
                                                        <td>{format_text}</td>
                                                        <td>
                                                            <button class="action-button">{"Download"}</button>
                                                        </td>
                                                    </tr>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </tbody>
                                </table>
                            }
                        } else {
                            html! { <div class="loading-container">{"Loading reports..."}</div> }
                        }
                    }
                </div>
            }
        },
        Page::Logs => {
            html! {
                <div class="logs-page">
                    <h2>{"Activity Logs"}</h2>
                    
                    {
                        if let Some(log_list) = logs.as_ref() {
                            html! {
                                <table class="data-table">
                                    <thead>
                                        <tr>
                                            <th>{"Timestamp"}</th>
                                            <th>{"User"}</th>
                                            <th>{"Action"}</th>
                                            <th>{"Target"}</th>
                                            <th>{"Details"}</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {
                                            log_list.iter().map(|log| {
                                                html! {
                                                    <tr key={log.id.clone()}>
                                                        <td>{&log.timestamp}</td>
                                                        <td>{&log.user_id}</td>
                                                        <td>{&log.action}</td>
                                                        <td>{&log.target}</td>
                                                        <td>{&log.details}</td>
                                                    </tr>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </tbody>
                                </table>
                            }
                        } else {
                            html! { <div class="loading-container">{"Loading activity logs..."}</div> }
                        }
                    }
                </div>
            }
        },
        Page::Settings => {
            html! {
                <div class="settings-page">
                    <h2>{"Settings"}</h2>
                    
                    {
                        if let Some(app_settings) = settings.as_ref() {
                            html! {
                                <div class="settings-sections">
                                    <div class="settings-section">
                                        <h3>{"System Information"}</h3>
                                        
                                        <div class="settings-item">
                                            <div class="settings-label">{"Registered To"}</div>
                                            <div class="settings-value">{&app_settings.registered_to}</div>
                                        </div>
                                        
                                        <div class="settings-item">
                                            <div class="settings-label">{"Server Status"}</div>
                                            <div class="settings-value">
                                                {
                                                    if app_settings.server_status {
                                                        html! { <span class="status-online">{"Online"}</span> }
                                                    } else {
                                                        html! { <span class="status-offline">{"Offline"}</span> }
                                                    }
                                                }
                                            </div>
                                        </div>
                                        
                                        <div class="settings-item">
                                            <div class="settings-label">{"API URL"}</div>
                                            <div class="settings-value">{&app_settings.api_url}</div>
                                        </div>
                                        
                                        <div class="settings-item">
                                            <div class="settings-label">{"License Expiry"}</div>
                                            <div class="settings-value">{&app_settings.license_expiry}</div>
                                        </div>
                                        
                                        <div class="settings-item">
                                            <div class="settings-label">{"App Version"}</div>
                                            <div class="settings-value">{&app_settings.app_version}</div>
                                        </div>
                                        
                                        <div class="settings-actions">
                                            <button class="primary-button">{"Renew License"}</button>
                                        </div>
                                    </div>
                                    
                                    <div class="settings-section">
                                        <h3>{"Preferences"}</h3>
                                        
                                        <div class="settings-item">
                                            <div class="settings-label">{"Theme"}</div>
                                            <div class="settings-value">
                                                <div class="radio-group">
                                                    <label>
                                                        <input type="radio" name="theme" value="light" 
                                                            checked={app_settings.theme == "light"} />
                                                        {"Light"}
                                                    </label>
                                                    <label>
                                                        <input type="radio" name="theme" value="dark"
                                                            checked={app_settings.theme == "dark"} />
                                                        {"Dark"}
                                                    </label>
                                                </div>
                                            </div>
                                        </div>
                                        
                                        <div class="settings-item">
                                            <div class="settings-label">{"Email Alerts"}</div>
                                            <div class="settings-value">
                                                <label class="switch">
                                                    <input type="checkbox" checked={app_settings.email_alerts} />
                                                    <span class="slider round"></span>
                                                </label>
                                            </div>
                                        </div>
                                        
                                        <div class="settings-item">
                                            <div class="settings-label">{"SMS Alerts"}</div>
                                            <div class="settings-value">
                                                <label class="switch">
                                                    <input type="checkbox" checked={app_settings.sms_alerts} />
                                                    <span class="slider round"></span>
                                                </label>
                                            </div>
                                        </div>
                                        
                                        <div class="settings-item">
                                            <div class="settings-label">{"Refresh Interval"}</div>
                                            <div class="settings-value">
                                                <input type="number" value={app_settings.refresh_interval.to_string()} min="5" max="60" />
                                                <span>{" seconds"}</span>
                                            </div>
                                        </div>
                                        
                                        <div class="settings-actions">
                                            <button class="primary-button">{"Save Changes"}</button>
                                            <button class="secondary-button">{"Reset to Default"}</button>
                                        </div>
                                    </div>
                                </div>
                            }
                        } else {
                            html! { <div class="loading-container">{"Loading settings..."}</div> }
                        }
                    }
                </div>
            }
        }
    }
}

// Modal Component
#[derive(Properties, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub is_open: bool,
    #[prop_or(Callback::noop())]
    pub on_close: Callback<MouseEvent>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Modal)]
fn modal(props: &ModalProps) -> Html {
    if !props.is_open {
        return html! {};
    }

    let onclick = props.on_close.clone();
    
    let modal_onclick = Callback::from(move |e: MouseEvent| {
        e.stop_propagation();
    });
    
    html! {
        <div class="modal-overlay" onclick={onclick.clone()}>
            <div class="modal-container" onclick={modal_onclick}>
                <div class="modal-header">
                    <h3 class="modal-title">{&props.title}</h3>
                    <button class="close-button" onclick={onclick}>{"×"}</button>
                </div>
                <div class="modal-content">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}

// Mock data for development
fn mock_cameras() -> Vec<Camera> {
    vec![
        Camera {
            id: "camera1".to_string(),
            name: "Front Door".to_string(),
            ip_address: "192.168.1.101".to_string(),
            port: 8080,
            location: "Main Entrance".to_string(),
            active: true,
            status: CameraStatus::Online,
            last_update: "2023-01-15 10:30:45".to_string(),
        },
        Camera {
            id: "camera2".to_string(),
            name: "Back Yard".to_string(),
            ip_address: "192.168.1.102".to_string(),
            port: 8080,
            location: "Rear Exit".to_string(),
            active: true,
            status: CameraStatus::Offline,
            last_update: "2023-01-15 09:15:22".to_string(),
        },
        Camera {
            id: "camera3".to_string(),
            name: "Garage".to_string(),
            ip_address: "192.168.1.103".to_string(),
            port: 8080,
            location: "Vehicle Entry".to_string(),
            active: false,
            status: CameraStatus::Maintenance,
            last_update: "2023-01-14 14:45:30".to_string(),
        },
    ]
}

// Mock data for other entities
fn mock_users() -> Vec<User> {
    vec![
        User {
            id: "user1".to_string(),
            username: "admin".to_string(),
            name: "Admin User".to_string(),
            email: "admin@example.com".to_string(),
            role: UserRole::SuperAdmin,
            active: true,
            last_login: "2025-02-25 08:15".to_string(),
        },
        User {
            id: "user2".to_string(),
            username: "jane".to_string(),
            name: "Jane Smith".to_string(),
            email: "jane@example.com".to_string(),
            role: UserRole::Admin,
            active: true,
            last_login: "2025-02-24 14:22".to_string(),
        },
        User {
            id: "user3".to_string(),
            username: "john".to_string(),
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            role: UserRole::Viewer,
            active: true,
            last_login: "2025-02-25 09:03".to_string(),
        },
        User {
            id: "user4".to_string(),
            username: "alice".to_string(),
            name: "Alice Brown".to_string(),
            email: "alice@example.com".to_string(),
            role: UserRole::Viewer,
            active: false,
            last_login: "2025-01-15 10:30".to_string(),
        },
    ]
} 