//! This file has been auto-generated, please do not modify manually
//! To regenerate this file re-run `cargo xtask generate tests` from the project root

use std::fs;
// use tempdir::TempDir;
// use xshell::{cmd, Shell};

#[test]
fn api_read_only() -> anyhow::Result<()> {
    // let sh = Shell::new()?;
    // let file_name = "api_read_only";
    // let tempdir = TempDir::new("{file_name}")?;
    // let wasi_file = test_utils::compile(&sh, &tempdir, &file_name)?;
    let _ = fs::remove_dir_all("./tests/rundir/api_read_only");
    // cmd!(sh, "node ./src/jco.js run  --jco-dir ./tests/rundir/api_read_only --jco-import ./tests/virtualenvs/base.js {wasi_file} hello this '' 'is an argument' 'with 🚩 emoji'").run()?;
    panic!("skipped"); // Ok(())
}
