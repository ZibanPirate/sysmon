# System Monitor

Cross-platform Widget showing download/upload speed at the corner of your screen, with minimal overhead.

**example:**

|                       macOS                       |                      Windows                       |
| :-----------------------------------------------: | :------------------------------------------------: |
| ![alt text](assets/jpeg/sysmon-sample-macos.jpeg) | ![alt text](assets/jpeg/sysmon-sample-windows.png) |

## Download

| OS      | Processor     | Download                                                                                                                                                                                                                                                                                                                                                                  |
| ------- | ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| MacOS   | Apple Silicon | - [Installer](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_0.0.32_aarch64.dmg)<br>- [Standalone](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_aarch64.app.tar.gz)                                                                                                                                       |
| MacOS   | Intel         | - [Installer](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_0.0.32_x64.dmg)<br>- [Standalone](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_x64.app.tar.gz)                                                                                                                                               |
| Windows | Arm           | N/A                                                                                                                                                                                                                                                                                                                                                                       |
| Windows | x86           | - [Installer](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_0.0.32_x64-setup.exe)<br>- [NSIS Installer](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_0.0.32_x64-setup.nsis.zip)<br>- [MSI Installer](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_0.0.32_x64_en-US.msi) |
| Linux   | Arm           | N/A                                                                                                                                                                                                                                                                                                                                                                       |
| Linux   | x86           | N/A                                                                                                                                                                                                                                                                                                                                                                       |

## Contribution

the project is a mess now, but feel free to fork and modify the code.

once things are a bit clear, I will update this section.

a rough roadmap is compiled into the checklist bellow

### Checklist

- [x] gether Network speed
- [x] show network speed graph on an always-on-top widget
- [x] Auto-update
- [x] persist and apply settings
- [x] Windows support
- [ ] Linux support
- [ ] Submit to Apple Store
- [ ] code quality: no-unwrap, tests
- [ ] Telemetry data on user consent
- [ ] more widgets
  - [ ] CPU
  - [ ] RAM
  - [ ] Storage
- [ ] optimize Widgets for low CPU consumption

### Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
