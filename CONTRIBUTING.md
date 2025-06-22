Inside `./scripts` you should find everything you need to start hacking locally.

make sure to have a `./scripts/.env` file with `CLI_RUN_CWD="path-to-repo-root"`

> **Note**
> on windows:
> 1 - prefix the commands bellow with `RUST_BACKTRACE=1 cargo +nightly -Zscript <./command.rs>`
> 2 - set `RUST_BACKTRACE=1` and set correct path for `HOME=C:\Users\<you>` in `./scripts/.env`

**One time setup**

set `CLI_RUN_CWD="path/to/cloned/repo/root"` in `./scripts/.env`
then

```sh
./scripts/setup.rs
```

**Develop locally**

Run app in dev mode:

```sh
./scripts/tauri.rs dev
```

Build and bundle the app:

```sh
./scripts/tauri.rs build
```

The rest of the scripts are self-explanatory.
