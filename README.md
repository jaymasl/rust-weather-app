# Weather App

A full-stack Rust weather application using Yew for the frontend and Rocket for the backend. 
Uses www.weatherapi.com to get current weather data from user input.

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
