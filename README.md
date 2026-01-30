# Finger Monitor

A modern, GPU-accelerated keyboard and mouse activity monitor built with Rust and GPUI. Track your daily input habits, analyze typing speed, and visualize your usage patterns with real-time statistics.

![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)
![GPUI](https://img.shields.io/badge/GUI-GPUI-blue.svg)
![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS-green)
![License](https://img.shields.io/badge/License-MIT-purple.svg)

## ✨ Features

*   **Real-time Dashboard**: Monitor your stats live with a high-performance GPU-rendered UI.
*   **Detailed Statistics**:
    *   **Keystrokes**: Track total keys pressed, WPM (Words Per Minute), and top used keys.
    *   **Mouse Tracking**: Monitor clicks (Left/Right/Middle), movement distance (meters/km), and scroll wheel usage.
    *   **Time Analysis**: Hourly activity charts to understand your peak productivity times.
*   **Visualizations**:
    *   Keyboard Heatmap (Visual representation of key usage).
    *   Top Key Leaderboard.
*   **Data Persistence**: Automatically saves your statistics locally, ensuring no data is lost between sessions.
*   **Modern UI**: Cyberpunk-inspired dark theme with a frameless, draggable, and resizable window.

## 🚀 Installation

### Prerequisites

Ensure you have Rust installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Linux Dependencies

On Ubuntu/Debian based systems, you need to install the following dependencies for GPUI and input monitoring:

```bash
sudo apt install libxdo-dev libx11-dev libxcb1-dev libxcb-render0-dev \
    libxcb-shape0-dev libxcb-xfixes0-dev libwayland-dev \
    libvulkan-dev pkg-config cmake
```

**Important**: To allow the application to listen to global input events without root privileges, add your user to the `input` group:

```bash
sudo usermod -aG input $USER
# You must log out and log back in for the changes to take effect
```

## 🛠️ Build & Run

```bash
# Clone the repository
git clone https://github.com/yourusername/rust-finger.git
cd rust-finger

# Run in development mode
cargo run

# Build for release
cargo build --release
```

## 📊 Data Storage

Your statistics are stored locally in JSON format, keeping your data private:

*   **Linux**: `~/.local/share/rust-finger/stats.json`
*   **macOS**: `~/Library/Application Support/rust-finger/stats.json`

## 🏗️ Project Structure

*   `src/main.rs`: Application entry point.
*   `src/listener.rs`: Global input event listener handling (using `rdev`).
*   `src/stats.rs`: Core statistics data structure and persistence logic.
*   `src/ui/`: GPUI-based user interface components.
    *   `dashboard.rs`: Main window layout and widget composition.
    *   `keyboard_heatmap.rs`: Visual keyboard representation.
    *   `charts.rs`: Graph rendering for hourly stats.

## 🔧 Development

```bash
# Run tests
cargo test

# Check for linting errors
cargo clippy
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📝 License

This project is licensed under the MIT License.
