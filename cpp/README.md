# Media Dashboard - Beautiful C++ Application

A stunning media server dashboard with beautiful terminal UI built in C++17.

## Features

✨ **Beautiful Terminal Interface**
- Colorful ANSI terminal output
- Box-drawing characters for elegant UI
- Interactive menu system
- Smooth user experience

📊 **Dashboard Statistics**
- Media library overview (Movies, TV Shows, Music)
- Active user streams
- Storage usage statistics
- Real-time server metrics

🎬 **Media Management**
- Browse media library
- View recent additions
- Monitor active playback
- User connection tracking

💻 **System Information**
- Server status and uptime
- CPU and RAM usage
- Network statistics
- System specifications

## Building

### Requirements
- C++17 compatible compiler (GCC 7+, MSVC 2017+, Clang 5+)
- CMake 3.15 or higher

### Quick Build
```bash
./build.sh
```

### Manual Build
```bash
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
cmake --build . --config Release
```

## Running

```bash
# Run the dashboard
./build/bin/media-dashboard

# Show version
./build/bin/media-dashboard --version

# Show help
./build/bin/media-dashboard --help
```

## Screenshots

The application features:
- Beautiful ASCII art banner
- Color-coded statistics
- Interactive menu navigation
- Elegant box-drawn interfaces
- Real-time data display

## Architecture

- **Platform**: Cross-platform (Windows/Linux)
- **Language**: C++17
- **Build System**: CMake
- **Architecture**: x64
- **UI**: Terminal-based with ANSI colors

## Project Structure

```
cpp/
├── CMakeLists.txt          # Build configuration
├── build.sh                # Quick build script
├── include/
│   ├── dashboard.h         # Dashboard class interface
│   └── utils.h             # Utility functions
└── src/
    ├── main.cpp            # Application entry point
    ├── dashboard.cpp       # Dashboard implementation
    └── utils.cpp           # Utility implementations
```

## License

Part of the Media-Server-Tools project.
