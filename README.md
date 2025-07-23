# System Monitor

Cross-platform Widget showing download/upload speed at the corner of your screen, with minimal overhead.

**example:**

|                           macOS                           |                          Windows                           |
| :-------------------------------------------------------: | :--------------------------------------------------------: |
| ![alt text](desktop/assets/jpeg/sysmon-sample-macos.jpeg) | ![alt text](desktop/assets/jpeg/sysmon-sample-windows.png) |

## Download

| OS      | Processor     | Download                                                                                                                                                                                                                                                                                                                                                                      |
| ------- | ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| MacOS   | Apple Silicon | - [Installer](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_0.0.32_aarch64.dmg)<br>- [Standalone](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_aarch64.app.tar.gz)                                                                                                                                           |
| MacOS   | Intel         | - [Installer](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_0.0.32_x64.dmg)<br>- [Standalone](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_x64.app.tar.gz)                                                                                                                                                   |
| Windows | Arm           | - [Installer](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_0.0.32_arm64-setup.exe)<br>- [NSIS Installer](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_0.0.32_arm64-setup.nsis.zip)<br>- [MSI Installer](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_0.0.32_x64_en-US.msi) |
| Windows | x86           | - [Installer](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_0.0.32_x64-setup.exe)<br>- [NSIS Installer](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_0.0.32_x64-setup.nsis.zip)<br>- [MSI Installer](https://github.com/ZibanPirate/sysmon/releases/latest/download/System.Monitor_0.0.32_x64_en-US.msi)     |
| Linux   | Arm           | N/A                                                                                                                                                                                                                                                                                                                                                                           |
| Linux   | x86           | N/A                                                                                                                                                                                                                                                                                                                                                                           |

## Contributing

Contributions are welcome, please read [`CONTRIBUTING.md`](./CONTRIBUTING.md) to get started.

here's a rough roadmap of the project

### Checklist

- [x] gether Network speed (`./lib-cpp` and `./lib-swift`)
- [x] show network speed graph on an always-on-top widget, and on tray icon text
- [x] Auto-update
- [ ] improve rust code quality: no-unwrap, tests, clippy on CI checks
- [ ] improve lib-cpp and lib-swift code quality, formatting, tests, lint, CI checks
- [ ] Telemetry data on user consent
- [ ] Linux support
- [ ] Apple Store support
- [ ] more widgets
  - [ ] CPU
  - [ ] RAM
  - [ ] Storage
- [ ] optimize Widgets for low CPU consumption

#### todos

- todo-zm: bug: settings window flash before loading in Windows
- todo-zm: bug: tray menue can be hidden in Windows via settings (should we detect it?)
- todo-zm: bug: auto-start sometimes not working on Windows
- todo-zm: bug: network widget flashes a scroll bar real quick when ran on windows in debug mode
- todo-zm: have a landing page for https://sysmon.zak-man.com
- todo-zm: change the updater endpoint to sysmon.zak-man.com/...
- todo-zm: quit app on SIGINT and SIGTERM in debug mode
- todo-zm: add CI checks for PRs with format, lint, build, and tests
- todo-zm: figure out how to show numbers in tray icon on windows (via icon?)
- todo-zm: sign Windows app https://v2.tauri.app/distribute/sign/windows/#azure-key-vault
