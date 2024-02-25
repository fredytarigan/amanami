use std::process::ExitCode;

use amanami::cli::run;

fn main() -> anyhow::Result<ExitCode> {
    let _ = run();

    Ok(ExitCode::SUCCESS)
}
