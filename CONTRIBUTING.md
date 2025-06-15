Inside `./scripts` you should find everything you need to start hacking locally.

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
