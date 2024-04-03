use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]

pub struct Args {
    #[arg(short, long)]
    pub hex: String,
}

fn main() {
    let args = Args::parse();

    println!("Hex: #{}", args.hex);
    println!("R: {}, G: {}, B: {}", hex_to_rgb(args.hex.clone())[0], hex_to_rgb(args.hex.clone())[1], hex_to_rgb(args.hex)[2]); // Yup... I know...
}

fn hex_to_dec(hex: &str) -> i32 {
    i32::from_str_radix(hex, 16).expect("err")
}

fn hex_to_rgb(hex: String) -> [i32; 3] {

    let r_h = hex.split_at(2);
    let g_h = r_h.1.split_at(2);
    let b_h = g_h.1.split_at(2);

    let rgb_array = [
        hex_to_dec(r_h.0),
        hex_to_dec(g_h.0),
        hex_to_dec(b_h.0),
    ];

    rgb_array
}

fn hex_to_frgb(hex: String) -> String {
    todo!()
}