use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: hexdump-rs <file>");
        return;
    }

    let data = fs::read(&args[1]).expect("failed to read file");

    for (index, byte) in data.iter().enumerate() {
        if index % 16 == 0 {
            if index != 0 {
                println!();
            }

            print!("{index:08x}  ");
        }

        print!("{byte:02x} ");
    }

    println!();
}
