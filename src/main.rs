mod args;

fn main() {
    let args = args::parse();

    println!("Args: {:?}", args);
}
