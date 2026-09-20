use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: hexdump-rs <file>");
        return;
    }

    let data = fs::read(&args[1]).expect("failed to read file");

    for (index, chunk) in data.chunks(16).enumerate() {
        print!("{:08x}  ", index * 16);

        let mut ascii = String::new();

        for byte in chunk {
            print!("{byte:02x} ");

            if (0x20..=0x7e).contains(byte) {
                ascii.push(*byte as char);
            } else {
                ascii.push('.');
            }
        }

        // Padding for incomplete rows.
        for _ in chunk.len()..16 {
            print!("   ");
        }

        println!(" |{ascii}|");
    }
}
