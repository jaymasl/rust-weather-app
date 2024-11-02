# Weather App

This is a full-stack weather application built entirely in Rust, using Yew (WebAssembly) for the frontend and Rocket for the backend. Users can search for cities to view detailed weather information including temperature (in both °C and °F), conditions, air quality, and location details, with support for dark/light themes. The backend integrates with WeatherAPI.com and stores all search requests in a PostgreSQL database (tracking user IP, user agent, search query, and full weather data), implements rate limiting (5-second delay between requests), and includes comprehensive error handling. The app features a responsive design with loading states, user-friendly error messages, and proper security measures like CORS support and request validation.

## Features
- Real-time weather data display
- Dark/light theme toggle
- Air quality information
- Location-based search
- Rate limiting protection

## Setup
```bash
git clone https://github.com/jaymasl/rust-weather-app.git
cd rust-weather-app
```

Create Secrets.toml in the backend/ directory with your WEATHER_API_KEY set.
```rust
WEATHER_API_KEY = "your API key"
```

Run the build script in the rust-weather-app/ directory
```bash
./build.sh
```

## Requirements
- Rust
- Trunk
- Shuttle

## Structure
- `/frontend` - Yew-based UI
- `/backend` - Rocket server + API handling
- `/shared` - Common types between frontend/backend
