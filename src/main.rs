mod args;
mod generator;

use crate::args::Arguments;
use crate::generator::generate_password;
use clap::Parser;

fn main() {
    let args : Arguments = Arguments::parse();
    let password : String = generate_password(&args);

    println!("your password is: {:?}", password);
}
