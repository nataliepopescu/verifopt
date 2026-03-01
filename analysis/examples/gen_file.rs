use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
//use rand::RngExt;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            println!("Pass in a filename and a length");
            Ok(())
        }
        _ => {
            let filename = &args[1];
            let num: usize = args[2].parse().unwrap();
            println!("filename: {:?}", filename);
            println!("len: {:?}", num);

            let file = File::create(filename)?;
            let mut writer = BufWriter::new(file);

            for i in 0..num {
                if i % 2 == 0 {
                    writer.write(b"0")?;
                } else {
                    writer.write(b"1")?;
                }
                //let r = rand::rng().random_range(..2u8);
                //writer.write(&[r])?;
            }
            writer.flush()?;

            Ok(())
        }
    }
}
