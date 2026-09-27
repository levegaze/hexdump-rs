use std::env;
use std::fs::File;
use std::io::{self, Read};

const BUFFER_SIZE: usize = 4096;
const BYTES_PER_ROW: usize = 16;

fn dump<R: Read>(reader: &mut R) {
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut offset = 0usize;

    loop {
        let bytes_read = reader.read(&mut buffer).expect("failed to read input");

        if bytes_read == 0 {
            break;
        }

        let data = &buffer[..bytes_read];

        for chunk in data.chunks(BYTES_PER_ROW) {
            print!("{offset:08x}  ");

            let mut ascii = String::new();

            for byte in chunk {
                print!("{byte:02x} ");

                if (0x20..=0x7e).contains(byte) {
                    ascii.push(*byte as char);
                } else {
                    ascii.push('.');
                }
            }

            for _ in chunk.len()..BYTES_PER_ROW {
                print!("   ");
            }

            println!(" |{ascii}|");

            offset += chunk.len();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("usage: hexdump-rs [file]");
        return;
    }

    if args.len() == 2 {
        let mut file = File::open(&args[1]).expect("failed to open file");

        dump(&mut file);
    } else {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();

        dump(&mut stdin);
    }
}
