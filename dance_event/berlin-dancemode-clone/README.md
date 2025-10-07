# Berlin DanceMode Clone

A full-stack clone of the Berlin DanceMode website focused on Blues & Fusion dancing events, learning, community and self-care.

## Tech Stack

- **Frontend**: SvelteJS
- **Backend**: Rust with Actix-web
- **Database**: Sled (embedded key-value store)
- **Logging**: File system logging

## Features

- 🎯 Event scheduling and management
- 💃 Dancer registration and profiles
- 💳 Payment processing for dance packages
- 📅 Event calendar
- 🏢 Venue management
- 👥 Community features
- 📱 Responsive design

## Project Structure

```
berlin-dancemode-clone/
├── frontend/           # SvelteJS application
├── backend/           # Rust Actix-web API server
├── shared/            # Shared types and utilities
└── docs/             # Project documentation
```

## Getting Started

### Prerequisites

- Node.js 18+ and npm
- Rust 1.70+
- Git

### Installation

1. Clone the repository
2. Set up the backend: `cd backend && cargo run`
3. Set up the frontend: `cd frontend && npm install && npm run dev`

## Development

See individual README files in `frontend/` and `backend/` directories for detailed setup instructions.

## License

© 2025 DanceMode Berlin Clone - Educational Project