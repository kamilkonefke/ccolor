use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Get color information.
    #[arg(short, long)]
    info: String,
}

pub struct Color {
    r: i32,
    g: i32,
    b: i32,
}

fn main() {
    let args = Args::parse();

    let output = hex_to_rgb(&args.info);

    println!("Hex: #{}", &args.info);

    // RGB output
    print!(" R: {}", output.r.to_string().red());
    print!(" G: {}", output.g.to_string().green());
    print!(" B: {}", output.b.to_string().blue());
}

fn hex_to_rgb(hex: &str) -> Color {

    let color = split_hex(hex);

    Color {
        r: i32::from_str_radix(color[0].as_str(), 16).expect("err"),
        g: i32::from_str_radix(color[1].as_str(), 16).expect("err"),
        b: i32::from_str_radix(color[2].as_str(), 16).expect("err"),
    }
}

fn hex_to_float(hex: &str) -> Color {
    todo!();
}

fn split_hex(hex: &str) -> Vec<String> {
    vec![
        hex.get(0..2).unwrap().to_string(),
        hex.get(2..4).unwrap().to_string(),
        hex.get(4..6).unwrap().to_string(),
    ]
}