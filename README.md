## Getting Started

First, install prerequisites by following [this guide](https://tauri.app/v1/guides/getting-started/prerequisites).

Then install the project dependencies:
```
npm install
```

### Starting the development environment
```
npm run tauri dev
```

### Debugging the rust backend in VS Code
1. Press <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>B</kbd> to start the NextJS development server (this runs `npm run dev` which is configured as the default build task)
2. Start debugging using the 'Tauri Development Debug' configuration

### Building the installer
```
npm run tauri build
```
