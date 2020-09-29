use clap::{Clap, crate_authors, crate_version, crate_description, AppSettings};

#[derive(Clap, Debug)]
#[clap(version = crate_version!(), author = crate_authors!("\n"), about = crate_description!())]
struct Args {
    #[clap(subcommand)]
    subcmd: SubCommand
}

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(about = "Add a TODO", setting = AppSettings::TrailingVarArg)]
    Add(Add)
}

#[derive(Clap, Debug)]
struct Add {
    #[clap(short, long, about = "The datetime that the TODO is due")]
    due: Option<String>,

    #[clap(multiple = true)]
    message: Vec<String>,
}

fn main() {
    let args = Args::parse();

    println!("Args: {:?}", args);
}