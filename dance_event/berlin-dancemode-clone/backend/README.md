# Berlin DanceMode Backend

Rust-based backend API server using Actix-web framework and Sled database.

## Features

- 🔐 JWT-based authentication
- 📊 Event management
- 👥 User registration and profiles
- 🏢 Venue management
- 📦 Package management
- 💳 Registration and payment processing (placeholder)
- 📝 Sled embedded database for persistence
- 📋 Input validation
- 🔒 Password hashing with bcrypt

## API Endpoints

### Authentication
- `POST /api/auth/register` - User registration
- `POST /api/auth/login` - User login

### Events
- `GET /api/events` - Get all events
- `GET /api/events/{id}` - Get event by ID
- `POST /api/events` - Create new event (admin)

### Venues
- `GET /api/venues` - Get all venues
- `POST /api/venues` - Create new venue (admin)

### Packages
- `GET /api/packages` - Get all packages
- `POST /api/packages` - Create new package (admin)

### Users
- `GET /api/users/profile` - Get user profile (requires auth)
- `PUT /api/users/profile` - Update user profile (requires auth)
- `GET /api/users/registrations` - Get user registrations (requires auth)

### Registrations
- `POST /api/registrations` - Create registration (requires auth)
- `POST /api/registrations/{id}/payment` - Process payment (requires auth)

### Health Check
- `GET /health` - Health check endpoint

## Setup

1. Install Rust (1.70+)
2. Clone the repository
3. Navigate to the backend directory: `cd backend`
4. Run the server: `cargo run`

The server will start on `http://127.0.0.1:8080`

## Environment Variables

- `HOST` - Server host (default: 127.0.0.1)
- `PORT` - Server port (default: 8080)
- `JWT_SECRET` - JWT secret key (default: development key)

## Database

The application uses Sled, an embedded key-value database. The database files are stored in the `./data/` directory.

Sample data is automatically initialized on first run, including:
- Sample venues
- Sample packages
- Sample events

## Development

```bash
# Run in development mode with auto-reload
cargo watch -x run

# Run tests
cargo test

# Check formatting
cargo fmt

# Run clippy for linting
cargo clippy
```

## Production Deployment

1. Set proper environment variables
2. Build the release binary: `cargo build --release`
3. Run the binary: `./target/release/berlin-dancemode-backend`

## Security Notes

- Change the default JWT secret in production
- Consider using a more robust database for production
- Implement proper rate limiting
- Add HTTPS in production
- Implement proper logging and monitoring