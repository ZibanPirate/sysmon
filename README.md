# System Monitor

Cross-platform Widget showing download/upload speed at the corner of your screen, with minimal overhead.

**example:**

|                         macOS                         |                        Windows                         |
| :---------------------------------------------------: | :----------------------------------------------------: |
| ![alt text](old/assets/jpeg/sysmon-sample-macos.jpeg) | ![alt text](old/assets/jpeg/sysmon-sample-windows.png) |

## Download

| OS      | Processor     | Download                                                                                                                                                                                                                                                                                                                                                                  |
| ------- | ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| MacOS   | Apple Silicon | - [Installer](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_0.0.32_aarch64.dmg)<br>- [Standalone](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_aarch64.app.tar.gz)                                                                                                                                       |
| MacOS   | Intel         | - [Installer](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_0.0.32_x64.dmg)<br>- [Standalone](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_x64.app.tar.gz)                                                                                                                                               |
| Windows | Arm           | N/A                                                                                                                                                                                                                                                                                                                                                                       |
| Windows | x86           | - [Installer](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_0.0.32_x64-setup.exe)<br>- [NSIS Installer](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_0.0.32_x64-setup.nsis.zip)<br>- [MSI Installer](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_0.0.32_x64_en-US.msi) |
| Linux   | Arm           | N/A                                                                                                                                                                                                                                                                                                                                                                       |
| Linux   | x86           | N/A                                                                                                                                                                                                                                                                                                                                                                       |

## Contributing

Contributions are welcome, please read [`CONTRIBUTING.md`](./CONTRIBUTING.md) to get started.

here's a rough roadmap of the project

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
