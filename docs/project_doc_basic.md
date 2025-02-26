Below is the full, final document incorporating all previous details plus an added section for a navigation drawer.

1. Overview

Purpose:
Develop an administrator web application for managing IP cameras and monitoring their data. The system enables IT administrators to configure, monitor, and report on camera activity using a clean, minimalist interface inspired by Material Design principles.

Target Users:
IT administrators responsible for maintaining networked IP cameras.

Functional Requirements:
	•	IP Camera CRUD Operations: Create, read, update, and delete camera configurations.
	•	Real-Time Data Monitoring: Display live camera statuses and streams.
	•	Role-Based Access Control: Enforce permissions so that only authorized users (admins) can access or modify certain screens and actions.
	•	Report Generation: Generate activity logs, uptime summaries, and other analytical reports.
	•	Settings Management: Manage system settings such as server connectivity, license information, and user preferences.
	•	User Registration Info: Display the name of the person who registered the product within the Settings page.

2. Technology Stack

Frontend
	•	Yew (Rust): A component-based framework compiled to WebAssembly for a high-performance UI.
	•	Tailwind CSS: A utility-first CSS framework that supports rapid styling and a Material Design–inspired aesthetic.
	•	Material Design Principles: Used for defining spacing, typography, interactive states, and component layout.

Backend
	•	Rust with Tokio: Asynchronous runtime for high-performance server-side operations.
	•	Serde: JSON serialization/deserialization for API communication.
	•	REST API Endpoints: Endpoints for camera management, user authentication, reports, and settings.

Security
	•	External Authentication API: Manages user credentials and returns tokens (e.g., JWT) that include role information.
	•	Role-Based Access Control (RBAC): Enforced in both the frontend (by hiding/disabling UI elements) and backend (through API access control).

3. Screens & Components

The application is organized into multiple screens with consistent navigation, clear hierarchy, and a unified design language.

A. Login Screen

Purpose:
Authenticate users before granting access to the system.

Components:
	•	Login Form: Fields for username and password.
	•	Action Button: “Login” button that submits credentials.
	•	Error Handling: Popups/modals display error messages if authentication fails.

ASCII Mockup – Login Screen:

+-------------------------------+
|    🔒 Admin Portal Login      |
+-------------------------------+
| Username: [__________]        |
| Password: [__________]        |
|                               |
|         [ Login ]             |
+-------------------------------+

B. Home Dashboard

Purpose:
Display an overview of all cameras with their statuses and quick actions to view live streams or detailed data.

Components:
	•	Camera List/Table: Displays camera name, IP address, and status (with icons: ✅ for online, ⚠️ for offline).
	•	Summary Panel: Shows total cameras, count of online/offline cameras.
	•	Navigation and Quick Actions: Buttons for “Add Camera” and “Refresh” data.
	•	Header Integration: Includes a notifications icon (🔔) and user profile icon (👤) (see Header section).

ASCII Mockup – Home Dashboard:

+--------- Dashboard ---------+    Logged in as: Admin 
| Camera          | Status    |    [Add Camera] [Refresh] 
|-----------------------------|
| Lobby Entrance  | Online ✅ |    4 Cameras (3 online, 1 offline)
| Side Door       | Offline ⚠️|
| Warehouse       | Online ✅ |
| Parking Lot     | Online ✅ |
+-----------------------------+

C. IP Camera Management

Purpose:
Provide full CRUD functionality for camera configurations.

Components:
	•	Camera Table: Lists cameras with fields (Name, IP, Port, Active status, Actions such as Edit/Delete).
	•	Active Toggle: Allows enabling/disabling cameras.
	•	Forms for Add/Edit:
	•	Can be inline or appear as a modal popup for focused data entry.
	•	Confirmation Popups: For actions like deleting a camera.

ASCII Mockup – Camera Management Screen:

+---- Camera Management ----+ 
| Name        IP          Active  Actions       |
|-----------------------------------------------|
| LobbyCam    192.168.0.101  [ON]  [Edit] [Del]   |
| SideDoorCam 192.168.0.102  [OFF] [Edit] [Del]   |
| WarehouseCam10.0.5.20      [ON]  [Edit] [Del]   |
| ParkingCam  10.0.5.21      [ON]  [Edit] [Del]   |
+-----------------------------------------------+
| + Add New Camera                        [Save] |
| Name: ______  IP: ______  Port: ___           |
| Location: ______  Active: [Yes] [No]          |
+-----------------------------------------------+

D. Settings

Purpose:
Manage system-wide configurations and view registered product details.

Components:
	•	Registered User Information: Displays “Registered To: [User Name]” (e.g., Jane Doe).
	•	Server Status: Indicates connectivity (e.g., Online ✅).
	•	API Endpoint & License Info: Shows API URL and license details (including expiry).
	•	User Preferences: Toggles/selectors for theme selection (Light/Dark), notifications (Email/SMS), and data refresh intervals.
	•	Modals/Popups: For tasks such as renewing licenses.

ASCII Mockup – Enhanced Settings Screen:

+-------- Settings --------+
| Registered To:  Jane Doe |
|--------------------------|
| Server:         Online ✅|
| API URL:        api.example.com |
| License:        Valid (expires 12/2025) |
| [ Renew License ]        |
|--------------------------|
| Preferences:             |
| Theme: [Light ◉]  [Dark ◯]|
| Email Alerts:   [X]      |
| SMS Alerts:     [ ]      |
| Refresh Interval: 10s    |
|--------------------------|
| App Version:    1.2.3    |
+--------------------------+

E. Live Camera View (Modal)

Purpose:
Display a live stream of a selected camera in a focused view.

Components:
	•	Video Player: Embeds the live feed (via a converted RTSP stream, if necessary).
	•	Camera Details: Shows current camera name, status, and live timestamp.
	•	Controls: Buttons for Pause/Play, Snapshot, and Fullscreen.
	•	Modal Display: Uses a popup/modal to keep the underlying page intact.

ASCII Mockup – Live Camera View:

+--- Live View: LobbyCam ---+
| [Stream Online]  (00:02:15)  |
| *************************   |
| *   Live Video Feed   *   |
| *   (stream frames)   *   |
| *************************   |
| Controls: [Pause] [Snapshot] [Fullscreen] |
+---------------------------+

F. Reports

Purpose:
Generate and download system reports (activity logs, uptime summaries, error logs).

Components:
	•	Report Selector: Dropdown to choose report type.
	•	Date Range Picker: Fields for selecting the reporting period.
	•	Format Options: Choose output format (e.g., CSV or PDF).
	•	Action Buttons: “Generate Report” and “Download Last” report.

ASCII Mockup – Reports Screen:

+------ Reports -------+
| Report Type: [Activity Log v]      |
| Period: From [2025-02-01]            |
|         To   [2025-02-25]            |
| Format: (•) CSV   ( ) PDF            |
|                                      |
| [Generate Report] [Download Last]    |
+-----------------------+

4. Global Components

A. Header

Purpose:
Provide a consistent navigation bar across the application.

Contents:
	•	Logo/Brand Name: Positioned on the left.
	•	Navigation Menu: Links to Dashboard, Cameras, Reports, and Settings.
	•	Notifications Icon (🔔): Shows a dropdown of recent alerts when clicked.
	•	User Icon (👤): Displays a dropdown with at least two options:
	•	Settings: Navigates to the Settings page.
	•	Logout: Ends the current session.

ASCII Mockup – Header:

+---------------------------------------------------------------------------------+
|  [Logo]   Dashboard   Cameras   Reports   Settings                   [🔔] [👤]  |
+---------------------------------------------------------------------------------+

User Dropdown Example:

+-------------------------+
|  • Settings             |
|  • Logout               |
+-------------------------+

B. Navigation Drawer

Purpose:
Offer an alternative, collapsible side navigation panel that aligns with Material Design patterns, particularly useful on larger screens or for mobile devices where the drawer can be toggled.

Contents:
	•	Menu Items: Same as header navigation (Dashboard, Cameras, Reports, Settings).
	•	User Info: Optionally display a mini profile (e.g., user’s avatar and name) at the top of the drawer.
	•	Responsive Behavior:
	•	Visible by default on larger screens.
	•	Collapsible or accessible via a hamburger icon on smaller devices.

ASCII Mockup – Navigation Drawer:

+-----------------------+
| [👤] Jane Doe         |
|-----------------------|
| > Dashboard           |
| > Cameras             |
| > Reports             |
| > Settings            |
+-----------------------+

C. Footer

Purpose:
Display application metadata and supplementary links.

Contents:
	•	Copyright
	•	App Version
	•	Links: e.g., Support, Privacy

ASCII Mockup – Footer:

+---------------------------------------------------------------------------------+
| © 2025 CameraAdmin Inc.   |   App Version: 1.2.3   |   Support   |   Privacy    |
+---------------------------------------------------------------------------------+

5. UI/UX Guidelines & Alignment

The overall design adheres to Material Design principles and a minimalist approach.

Color Palette
	•	Primary Colors:
	•	Use a primary blue or teal for headers, buttons, and active states.
	•	Accent Colors:
	•	Use green for “online” statuses and red/orange for errors or offline states.
	•	Background Colors:
	•	Light neutrals (white or light gray) to ensure content clarity.
	•	Text Colors:
	•	Dark, high-contrast text (near-black) on light backgrounds.

Borders & Shadows
	•	Borders:
	•	Minimal use; 1px solid borders in subtle gray for section delineation.
	•	Rounded Corners:
	•	Components such as cards, buttons, and modals use a border-radius of 4–8px.
	•	Shadows:
	•	Apply subtle box-shadows (e.g., box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.1)) for elevation effects on modals and cards.

Typography & Icons
	•	Typography:
	•	Use a clean sans-serif font (e.g., Roboto) with hierarchy indicated by different font weights.
	•	Icons:
	•	Employ Material icons for consistency; standard sizes are 24px–32px with appropriate padding.

Spacing & Grid
	•	Grid System:
	•	Follow an 8dp grid system to maintain consistent spacing.
	•	Padding/Margins:
	•	Consistent and generous spacing to avoid clutter.
	•	Interactive States:
	•	Clear focus and hover states (ripple effects, color changes) per Material Design standards.

Global Alignment
	•	Responsive Design:
	•	Header and drawer collapse gracefully on smaller screens (using a hamburger menu for the drawer).
	•	Footer remains unobtrusive and fixed at the bottom.
	•	Consistency:
	•	All UI components adhere to a unified style guide, ensuring a seamless user experience.

6. Development Guidelines

Component Architecture (with Modals, Notifications & Drawer)
	•	Reusable Components:
	•	Modal Component: For confirmations, form popups, and error alerts.
	•	Notification Component: To handle real-time notifications accessible via the header’s bell icon.
	•	Header, Navigation Drawer & Footer Components: Maintain global layout consistency.
	•	State Management:
	•	Global state includes user session details, notifications, and theme preferences.
	•	Routing & Access Control:
	•	Protect routes using role-based checks on both the client and server sides.

API Integration & Mock Data
	•	API Endpoints:
	•	Document endpoints such as /api/cameras, /api/login, /api/settings, etc.
	•	Mock Data:
	•	Use mock data structures during development to simulate API responses.
	•	Error Handling:
	•	Implement popups and toast notifications for API errors and user feedback.

Code Organization & Documentation
	•	Modular Structure:
	•	Separate components into modules: components, services (API calls), models (data structures), and routes.
	•	Version Control & CI:
	•	Use Git with clear commit messages and integrate continuous testing (for frontend WASM builds and Rust backend tests).
	•	Style Guides:
	•	Maintain inline documentation and a project README that details setup, build processes, and UI/UX design decisions.

7. Final Notes

This comprehensive document serves as a complete reference for developers and designers working on the administrator web app. It details the system architecture, UI components (with ASCII mockups for Login, Dashboard, Camera Management, Settings, Live View, Reports, Header, Navigation Drawer, and Footer), and UI/UX guidelines (color, typography, spacing, borders, and interactive states). By following these guidelines, the team can ensure a consistent, secure, and user-friendly product that adheres to modern design standards and Material Design principles.

With this final document, development of both the frontend (Yew + Tailwind CSS) and backend (Rust with Tokio & Serde) can commence, providing clear guidance on implementation and design alignment.