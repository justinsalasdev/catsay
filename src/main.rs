extern crate colored;
extern crate structopt;

use colored::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    /// What does the cat say??
    message: String,

    #[structopt(short = "d", long = "dead")]
    /// Make the cat appear dead
    dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the cat picture from specified file
    catfile: Option<std::path::PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::from_args();
    let message = options.message;

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shoudn't bark like a dog.")
    }

    let eye = if options.dead { "x" } else { "o" };
    println!("{}", message.bright_yellow().underline());

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)?;
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", &cat_picture);
            Ok(())
        }
        None => {
            println!(" \\");
            println!(" \\");
            println!("  /\\_/\\");
            println!(" ( {eye} {eye} )", eye = eye.red().bold()); // [2]
            println!(" =( I )=");
            Ok(())
        }
    }
}
