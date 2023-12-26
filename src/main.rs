use clap::Parser;

const ABOUT: &str = "Simple Base64 Encoder and Decoder Utility";

#[derive(Parser)]
#[command(author,version,about=ABOUT)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[arg(short, long)]
    text: String,

    #[arg(short, long, default_value = "false")]
    decode: bool,
}

fn main() {
    let args = Cli::parse();

    if args.decode {
        todo!("impl decoding string")
    } else {
        let content = args.text.as_bytes();

        let output = rbase64::encode(&content);

        println!("Encoded String : {}", output);
    }
}
