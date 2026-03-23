//extern crate rand;

use rand::RngExt;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 | 2 | 3 => {
            println!(
                "Pass in a filename, length, and type of generation (options: all, alt, rand, vis)"
            );
            Ok(())
        }
        _ => {
            let filename = &args[1];
            let num: usize = args[2].parse().unwrap();
            let ty = &args[3];
            println!("filename: {:?}", filename);
            println!("len: {:?}", num);
            println!("type: {:?}", ty);

            let file = File::create(filename)?;
            let mut writer = BufWriter::new(file);

            if ty == "all" {
                for _ in 0..num {
                    writer.write(b"0")?;
                }
                writer.flush()?;
            } else if ty == "alt" {
                for i in 0..num {
                    if i % 2 == 0 {
                        writer.write(b"0")?;
                    } else {
                        writer.write(b"1")?;
                    }
                }
                writer.flush()?;
            } else if ty == "rand" {
                for _ in 0..num {
                    let r = rand::rng().random_range(..2u8) & 1;
                    writer.write(&[r])?;
                }
                writer.flush()?;
            } else if ty == "vis" {
                for i in 0..num {
                    if i % 4 == 0 {
                        writer.write(b"0")?;
                        writer.write(b"0")?;
                    } else if i % 4 == 1 {
                        writer.write(b"0")?;
                        writer.write(b"1")?;
                    } else if i % 4 == 2 {
                        writer.write(b"1")?;
                        writer.write(b"0")?;
                    } else {
                        writer.write(b"1")?;
                        writer.write(b"1")?;
                    }
                }
            } else {
                panic!("nonexistent type: {:?}", ty);
            }

            Ok(())
        }
    }
}
