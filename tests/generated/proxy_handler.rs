//! This file has been auto-generated, please do not modify manually
//! To regenerate this file re-run `cargo xtask generate tests` from the project root

use std::fs;
use xshell::{cmd, Shell};

#[test]
fn proxy_handler() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let wasi_file = "./tests/rundir/proxy_handler.component.wasm";
    let _ = fs::remove_dir_all("./tests/rundir/proxy_handler");

    let cmd = cmd!(sh, "node ./src/jco.js run  --jco-dir ./tests/rundir/proxy_handler --jco-import ./tests/virtualenvs/server-api-proxy.js {wasi_file} hello this '' 'is an argument' 'with 🚩 emoji'");

    cmd.run()?;
    Ok(())
}