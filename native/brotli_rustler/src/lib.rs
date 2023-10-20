use brotli::{CompressorReader, Decompressor};
use rustler::{Binary, Env, NewBinary};
use std::io::{Read, Write};

#[rustler::nif(name = "compress_native", schedule = "DirtyCpu")]
fn compress<'a>(
    env: Env<'a>,
    input: Binary<'a>,
    compression_level: u32,
    lg_window_size: u32,
) -> Result<Binary<'a>, String> {
    let mut cr = CompressorReader::new(&input[..], 4096, compression_level, lg_window_size);
    let mut buffer = Vec::with_capacity(input.len());
    match cr.read_to_end(&mut buffer) {
        Ok(_) => {
            let mut output = NewBinary::new(env, buffer.len());
            output.as_mut_slice().write_all(&buffer[..]).unwrap();
            Ok(output.into())
        }
        Err(e) => Err(format!("compression error: {:?}", e)),
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn decompress<'a>(env: Env<'a>, input: Binary<'a>) -> Result<Binary<'a>, String> {
    let mut buffer = Vec::with_capacity(input.len());
    let mut dec = Decompressor::new(&input[..], 4096);
    match dec.read_to_end(&mut buffer) {
        Ok(_) => {
            let mut output = NewBinary::new(env, buffer.len());
            output.as_mut_slice().write_all(&buffer[..]).unwrap();
            // let mut binary = OwnedBinary::new(decompressed_bytes.len()).unwrap();
            // let _ = binary.as_mut_slice().write_all(&decompressed_bytes);
            Ok(output.into())
        }
        Err(e) => Err(format!("decompression error: {:?}", e)),
    }
}

rustler::init!("Elixir.BrotliRustler", [compress, decompress]);
