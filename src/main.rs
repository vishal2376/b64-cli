use clap::Parser;

const ABOUT: &str = "Simple Utility to encode or decode string";

#[derive(Parser)]
#[command(author,version,about=ABOUT)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[arg(short, long, required = false)]
    /// Encode or Decode Text
    text: String,

    #[arg(short, long)]
    /// Decode Text
    decode: bool,
}

fn main() {
    let args = Cli::parse();
    let content = args.text;

    if args.decode {
        let binary_string = rbase64::decode(&content).unwrap();

        let output = String::from_utf8_lossy(&binary_string);
        println!("Decoded String : {}", output);
    } else {
        let output = rbase64::encode(&content.as_bytes());

        println!("Encoded String : {}", output);
    }
}
