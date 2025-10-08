# Berlin DanceMode Backend 🦀

> Rust Actix-web API server for the Berlin DanceMode clone platform

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Actix-web](https://img.shields.io/badge/Actix--web-4.4-blue.svg)](https://actix.rs/)
[![Sled](https://img.shields.io/badge/Sled-0.34-green.svg)](https://github.com/spacejam/sled)

## 🚀 Overview

The backend is a high-performance REST API server built with Rust and Actix-web, featuring:

- **Fast & Safe**: Rust's performance with memory safety
- **Async**: Fully asynchronous request handling
- **Embedded Database**: Sled key-value store (no external DB required)
- **JWT Authentication**: Secure token-based authentication
- **CORS Enabled**: Ready for frontend integration
- **Structured Logging**: Comprehensive error tracking

## 🏗️ Architecture

### Project Structure

```
backend/
├── src/
│   ├── handlers/           # Request handlers
│   │   ├── auth.rs        # Authentication (login/register)
│   │   ├── events.rs      # Event management
│   │   ├── venues.rs      # Venue management  
│   │   ├── packages.rs    # Dance packages
│   │   ├── users.rs       # User profiles
│   │   └── registrations.rs # Event registrations
│   ├── models/            # Data structures
│   │   └── mod.rs         # User, Event, Venue, Package models
│   ├── database/          # Database operations
│   │   └── mod.rs         # CRUD operations & sample data
│   ├── routes/            # Route configurations
│   │   └── mod.rs         # API route definitions
│   ├── utils/             # Utilities
│   │   └── auth.rs        # JWT & password utilities
│   └── main.rs            # Application entry point
├── Cargo.toml             # Dependencies & metadata
└── README.md              # This file
```

### Key Components

#### � Authentication Flow
1. **Registration**: User creates account with bcrypt-hashed password
2. **Login**: Credentials validated, JWT token issued
3. **Protected Routes**: Token validated on each request
4. **Token Refresh**: Automatic token renewal

#### 💾 Database Layer
- **Sled**: Embedded key-value database
- **JSON Serialization**: Data stored as JSON strings
- **UUID Keys**: Unique identifiers for all entities
- **Automatic Initialization**: Sample data loaded on startup

## 🚀 Quick Start

### Prerequisites
- **Rust 1.70+**: [Install via rustup](https://rustup.rs/)
- **Cargo**: Comes with Rust installation

### Installation & Running

```bash
# Navigate to backend directory
cd backend

# Install dependencies & compile
cargo build

# Run in development mode with logging
RUST_LOG=debug cargo run

# Or run in release mode for better performance
cargo run --release
```

The server will start at: **http://127.0.0.1:8080**

### Development Commands

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Run tests
cargo test

# Check without building
cargo check

# Build for release
cargo build --release
```

## � Configuration

### Environment Variables

```bash
# Server configuration
export HOST=127.0.0.1          # Server host (default: 127.0.0.1)
export PORT=8080                # Server port (default: 8080)

# Logging
export RUST_LOG=debug           # Log level (debug, info, warn, error)

# JWT Secret (auto-generated if not set)
export JWT_SECRET=your-secret-key
```

### Database Configuration

The Sled database is automatically created at:
```
./data/dancemode.sled
```

No additional configuration required. The database includes:
- **Auto-initialization**: Sample data loaded on first run
- **Persistence**: Data survives server restarts
- **Atomic Operations**: ACID-compliant transactions

## 📡 API Endpoints

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
  "email": "dancer@example.com",
  "password": "securepassword123",
  "first_name": "Jane",
  "last_name": "Doe",
  "phone": "+491234567890",
  "dance_experience": "Intermediate"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
    "user": {
      "id": "uuid-here",
      "email": "dancer@example.com",
      "first_name": "Jane",
      "last_name": "Doe",
      "phone": "+491234567890",
      "dance_experience": "Intermediate",
      "created_at": "2025-10-07T08:47:45.181580Z"
    }
  },
  "message": null,
  "errors": null
}
```

#### Login User
```http
POST /api/auth/login
Content-Type: application/json

{
  "email": "dancer@example.com",
  "password": "securepassword123"
}
```

### Resource Endpoints

#### Events
- `GET /api/events` - List all events
- `GET /api/events/{id}` - Get specific event
- `POST /api/events` - Create event (auth required)

#### Venues
- `GET /api/venues` - List all venues
- `POST /api/venues` - Create venue (auth required)

#### Packages
- `GET /api/packages` - List all dance packages
- `POST /api/packages` - Create package (auth required)

#### Users
- `GET /api/users/profile` - Get user profile (auth required)
- `PUT /api/users/profile` - Update profile (auth required)
- `GET /api/users/registrations` - Get user registrations (auth required)

#### Health Check
- `GET /health` - Server health status

## 📊 Sample Data

The server initializes with sample data for development:

### Events (3 items)
- **Movement & Blues Immersion** - €280, 7-day intensive
- **Blues & Fusion Experience** - €85, Weekend festival  
- **Blues & Fusion Alumni Training** - €180, Advanced workshop

### Venues (2 items)
- **Dance Studio Berlin** - 50 capacity, Mitte location
- **Community Center Kreuzberg** - 80 capacity, Kreuzberg location

### Packages (3 items)
- **Beginner Blues** - €45, 1 day, 20 participants
- **Fusion Experience** - €85, 2 days, 25 participants
- **Blues & Fusion Immersion** - €180, 5 days, 15 participants

## 🔍 Testing

### Manual API Testing

Test endpoints using curl:

```bash
# Test venues endpoint
curl -X GET http://127.0.0.1:8080/api/venues

# Test user registration
curl -X POST http://127.0.0.1:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123",
    "first_name": "Test",
    "last_name": "User",
    "dance_experience": "Beginner"
  }'

# Test login
curl -X POST http://127.0.0.1:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }'
```

### Unit Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_user_registration
```

## 🚀 Deployment

### Production Build
```bash
cargo build --release
```

### Performance Considerations
- **Release Mode**: Always use `--release` for production
- **Database Path**: Ensure `./data/` directory is writable
- **Memory Usage**: Sled database runs in-memory for better performance
- **Concurrent Connections**: Actix-web handles thousands of concurrent connections

## 🐛 Troubleshooting

### Common Issues

#### Port Already in Use
```bash
# Check what's using port 8080
lsof -i :8080

# Kill process
kill -9 <PID>
```

#### Database Permissions
```bash
# Ensure data directory is writable
mkdir -p data
chmod 755 data
```

#### Compilation Errors
```bash
# Update Rust toolchain
rustup update

# Clean build cache
cargo clean
cargo build
```

### Logging

Enable debug logging for troubleshooting:
```bash
RUST_LOG=debug cargo run
```

Log levels available:
- `error` - Only errors
- `warn` - Warnings and errors  
- `info` - General information
- `debug` - Detailed debugging
- `trace` - Very verbose debugging

## 📈 Performance Metrics

### Expected Performance
- **Concurrent Connections**: 1000+ simultaneous connections
- **Response Time**: <10ms for simple queries
- **Memory Usage**: ~50MB base + data size
- **Startup Time**: <2 seconds with sample data

## 🤝 Contributing

### Code Style
- Follow Rust standard formatting: `cargo fmt`
- Use Clippy for linting: `cargo clippy`
- Write tests for new features
- Update documentation for API changes

### Adding New Endpoints

1. **Create Handler**: Add to `src/handlers/`
2. **Define Model**: Update `src/models/mod.rs`
3. **Add Route**: Configure in `src/routes/mod.rs`
4. **Add Tests**: Create test functions
5. **Update Docs**: Document API endpoint

---

**Happy Coding!** 🦀✨