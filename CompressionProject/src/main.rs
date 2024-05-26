extern crate flate2;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, copy, BufReader, BufWriter};

fn main() -> io::Result<()> {
    let input_file = File::open("input.pdf")?;
    let mut input = BufReader::new(input_file);

    let output_file = File::create("output.pdf.gz")?;
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    copy(&mut input, &mut encoder)?;

    encoder.finish()?;

    let compressed_file = File::open("output.pdf.gz")?;
    let mut decoder = GzDecoder::new(BufReader::new(compressed_file));

    let decompressed_file = File::create("decompressed_output.pdf")?;
    let mut output = BufWriter::new(decompressed_file);

    copy(&mut decoder, &mut output)?;

    Ok(())
}
