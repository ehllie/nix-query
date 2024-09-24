pub use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long)]
    pub index: bool,
}
