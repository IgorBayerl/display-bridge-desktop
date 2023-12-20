

# Project Name: Display Bridge 

## Description
Display Bridge is a cross-platform desktop application built with Tauri. It is designed to use an iPad as an external display for both Windows and Linux systems. The application captures the screen of your computer, encodes it as a video stream, and then sends this stream to the iPad via a USB-C connection.

## Key Features
- Screen capturing on Windows and Linux.
- Real-time video encoding.
- USB-C connectivity with iPad.
- Cross-platform support using Tauri.

## Getting Started

### Prerequisites
Before you begin, ensure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/)
- [Yarn](https://yarnpkg.com/) (Optional, can use npm instead)

### Installation
1. **Clone the Repository**
   ```bash
   git clone https://github.com/IgorBayerl/display-bridge-desktop.git
   cd display-bridge-desktop
   ```

2. **Install Dependencies**
   Navigate to the project directory and install the required Node dependencies:
   ```bash
   yarn install # or `npm install` if you're not using Yarn
   ```

3. **Run the Tauri Development Server**
   Start the development server with the following command:
   ```bash
   yarn tauri dev # or `npm run tauri dev`
   ```

### Building the Application
To build the application for production, use the following command:
```bash
yarn tauri build # or `npm run tauri build`
```
This will compile your application into an executable file for your platform.

## Contributing
Contributions to [Your Project's Name] are welcome. Please follow these steps to contribute:
1. Fork the repository.
2. Create a new branch for your feature.
3. Make your changes and commit them.
4. Push the branch and create a Pull Request.

## License
This project is licensed under the [License Name] - see the LICENSE file for details.

## Acknowledgments
- Tauri, for the cross-platform toolkit.
- [Any other libraries or tools used in the project]
- [Any other acknowledgments]

