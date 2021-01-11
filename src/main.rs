use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "Cameron P. Dart <cdart2@illinois.edu>")]
struct Opts {
    filename: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let opts: Opts = Opts::parse();
    let contents = tokio::fs::read(opts.filename).await?;
    println!("{}", String::from_utf8_lossy(&contents));
    Ok(())
}
