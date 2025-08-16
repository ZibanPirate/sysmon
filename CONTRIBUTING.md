**Prerequisites**

Have these installed:

- Rust
- NodeJS
- On Windows: Have Visual Studio installed with the component:
  - Desktop C++
  - Clang tools for Desktop C++ (make sure to add it to PATH, should be somewhere like "C:\Program Files\Microsoft Visual Studio\20xx\Community\VC\Tools\Llvm\bin")
  - Powershell

Make sure to have a `./scripts/.env` file with content:

```ini
CLI_RUN_CWD="path-to-repo-root"
RUST_BACKTRACE=1
HOME="C:\Users\<username>" # Windows only
```

**Commands**

> On Windows, add a `.ps1` to the commands bellow, eg: `./scripts/setup.rs` will be `./scripts/setup.rs.ps1`

- One time setup:

```sh
./scripts/setup.rs
```

- Run app in dev mode:

```sh
./scripts/tauri.rs dev
```

- Build and bundle the app:

```sh
./scripts/tauri.rs build
```

The rest of the scripts are self-explanatory.
