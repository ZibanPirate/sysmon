use cli_run::cli_run;

fn main() {
    cli_run(
        "typeshare",
        vec![
            "./",
            "--lang=typescript",
            "--output-file=./bindings/index.ts",
        ],
    );
}
