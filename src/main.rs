use clap::Parser;
use colored::Colorize;

use color::Color;
mod color;

/// ccolor is a simple program to convert hexadecimal colors to other formats.
#[derive(Parser)]
#[command(author = "Kamil Konefke", version)]
pub struct Cli {
    /// convert color to other formats
    #[arg(short, long, value_name = "HEX")]
    convert: Option<String>,
}

fn main() {
    let args = Cli::parse();

    let color = Color::from_hex(&args.convert.expect("err"));

    // RGB output
    print!(" R: {}", color.get_red().to_string().red());
    print!(" G: {}", color.get_green().to_string().green());
    print!(" B: {}", color.get_blue().to_string().blue());

    println!(" ");
    // normalize output
    print!(" R: {}", (color.get_red() as f32 / 255.0).to_string().red());
    print!(" G: {}", (color.get_green()  as f32 / 255.0).to_string().green());
    print!(" B: {}", (color.get_blue()  as f32 / 255.0).to_string().blue());
}