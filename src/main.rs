extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    message: String
}

fn main() {
    let options = Options::from_args();
    let message = options.message;
    println!("{}",message);
}
