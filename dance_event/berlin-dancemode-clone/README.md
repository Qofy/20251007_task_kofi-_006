# Berlin DanceMode Clone 🕺💃

> A comprehensive full-stack dance event management platform inspired by [Berlin DanceMode](https://www.berlindancemode.com/)

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Svelte](https://img.shields.io/badge/Svelte-4.2+-red.svg)](https://svelte.dev/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🎯 Project Overview

Berlin DanceMode Clone is a modern, full-stack web application that recreates the functionality of the popular Berlin dance community platform. It allows dancers to:

- 🕺 Browse and register for dance events
- 💳 Purchase different dance packages
- 🏢 Discover dance venues
- 👤 Create and manage user accounts
- 📅 Schedule and manage dance events

## 🏗️ Tech Stack

### Backend Architecture
- **Language**: Rust 1.70+
- **Framework**: Actix-web 4.4
- **Database**: Sled 0.34 (embedded key-value store)
- **Authentication**: JWT with bcrypt password hashing
- **API**: RESTful JSON API with CORS support

### Frontend Architecture
- **Framework**: SvelteJS 4.2+ (JavaScript, no TypeScript)
- **Build Tool**: Vite 5.0+
- **Styling**: CSS Modules with custom properties
- **HTTP Client**: Axios with interceptors
- **State Management**: Svelte stores

## 📁 Project Structure

```
berlin-dancemode-clone/
├── 📁 backend/                 # Rust Actix-web API server
│   ├── 📁 src/
│   │   ├── 📁 handlers/        # API route handlers
│   │   ├── 📁 models/          # Data models
│   │   ├── � database/        # Database operations
│   │   ├── 📁 routes/          # Route configurations
│   │   ├── 📁 utils/           # Utility functions
│   │   └── 📄 main.rs          # Application entry point
│   ├── 📄 Cargo.toml          # Rust dependencies
│   └── � README.md           # Backend documentation
├── 📁 frontend/                # SvelteJS application
│   ├── 📁 src/
│   │   ├── 📁 lib/             # Shared components & utilities
│   │   ├── 📁 routes/          # Page components
│   │   └── � app.html         # HTML template
│   ├── 📄 package.json        # Node.js dependencies
│   └── 📄 README.md           # Frontend documentation
├── 📄 README.md               # Main project documentation
└── � .gitignore              # Git ignore rules
```

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Node.js 18+ ([Install Node.js](https://nodejs.org/))
- Git ([Install Git](https://git-scm.com/))

### 1. Clone the Repository
```bash
git clone <repository-url>
cd berlin-dancemode-clone
```

### 2. Start the Backend Server
```bash
cd backend
cargo run
# Server starts at http://127.0.0.1:8080
```

### 3. Start the Frontend Development Server
```bash
# In a new terminal
cd frontend
npm install
npm run dev
# Application starts at http://127.0.0.1:5173
```

### 4. Access the Application
- **Frontend**: http://127.0.0.1:5173
- **Backend API**: http://127.0.0.1:8080
- **API Documentation**: See [API Reference](#-api-reference) below

## ✨ Features

### 🔐 Authentication System
- User registration with validation
- Secure login with JWT tokens
- Password hashing with bcrypt
- Automatic token management
- Session persistence

### 🎭 Event Management
- Browse upcoming dance events
- Event details (dates, venues, pricing)
- Participant tracking
- Event types (Workshop, Festival, Intensive)

### 🏢 Venue System
- Venue listings with capacity
- Location information
- Venue-event associations

### 💰 Dance Packages
- Multiple package tiers
- Duration and participant limits
- Pricing information
- Package descriptions

### 🎨 User Interface
- Responsive design for all devices
- Clean, modern aesthetics
- Mobile-friendly navigation
- Accessibility considerations

## 🔧 API Reference

### Base URL
```
http://127.0.0.1:8080
```

### Authentication Endpoints

#### Register User
```http
POST /api/auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword",
  "first_name": "John",
  "last_name": "Doe",
  "phone": "+491234567890",
  "dance_experience": "Beginner"
}
```

#### Login User
```http
POST /api/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword"
}
```

### Resource Endpoints

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| GET | `/api/events` | List all events | ❌ |
| GET | `/api/events/{id}` | Get event by ID | ❌ |
| POST | `/api/events` | Create new event | ✅ |
| GET | `/api/venues` | List all venues | ❌ |
| POST | `/api/venues` | Create new venue | ✅ |
| GET | `/api/packages` | List all packages | ❌ |
| POST | `/api/packages` | Create new package | ✅ |
| GET | `/api/users/profile` | Get user profile | ✅ |
| PUT | `/api/users/profile` | Update profile | ✅ |

### Sample Data

The application initializes with sample data including:
- 🎭 3 Dance events (Blues & Fusion themes)
- 🏢 2 Venues (Berlin locations)
- 💰 3 Dance packages (Beginner to Advanced)

## 🛠️ Development

### Environment Setup

#### Backend Environment Variables
```bash
# Optional - defaults provided
export HOST=127.0.0.1
export PORT=8080
export RUST_LOG=debug
```

#### Frontend Environment
```bash
# No environment variables required
# API base URL is configured in src/lib/api.js
```

### Database
- Uses **Sled** embedded database
- Data stored in `./backend/data/dancemode.sled`
- No external database setup required
- Automatic sample data initialization

### Code Quality
```bash
# Backend
cd backend
cargo fmt              # Format code
cargo clippy           # Lint code
cargo test             # Run tests

# Frontend
cd frontend
npm run lint           # Lint code
npm run format         # Format code
npm run check          # Type checking
```

## 📚 Documentation

- [Backend README](./backend/README.md) - Detailed backend documentation
- [Frontend README](./frontend/README.md) - Detailed frontend documentation
- [API Documentation](#-api-reference) - Complete API reference

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by [Berlin DanceMode](https://www.berlindancemode.com/)
- Built with ❤️ for the Berlin dance community
- Thanks to the Rust and Svelte communities

---

**Happy Dancing!** 🕺💃