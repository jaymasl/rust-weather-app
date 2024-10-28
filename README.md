# Weather App

A full-stack Rust weather application using Yew for the frontend and Rocket for the backend. Fetches real-time weather data with customizable location search.

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
