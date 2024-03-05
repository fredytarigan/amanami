use amanami::{cli::run, errors::ApplicationErrors};

fn main() -> Result<(), ApplicationErrors> {
    if let Err(e) = run() {
        println!("{:?}", e.to_string());
        std::process::exit(1)
    }

    Ok(())
}
