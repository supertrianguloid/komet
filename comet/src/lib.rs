use ciborium_io::Write as _;
use ciborium_ll::{Decoder, Encoder, Header};
use comet_algorithms;
use wasm_minimal_protocol::*;
use String;
initiate_protocol!();

mod read;

#[wasm_func]
pub fn histogram(input: &[u8]) -> Result<Vec<u8>, String> {

    let mut decoder = Decoder::from(&input[..]);

    match decoder.pull().unwrap() {
        Header::Array(Some(len)) => {
            if len != 2 {
                return Err(String::from("Expected array of 2 elements, got more"));
            }

            let data = match decoder.pull().unwrap() {
                Header::Array(Some(len)) => read::read_float_array(&mut decoder, len),
                _ => return Err(String::from("Bad input"))
            };

            let edges = match decoder.pull().unwrap() {
                Header::Array(Some(len)) => read::read_float_array(&mut decoder, len),
                Header::Positive(num_bins) => {
                    let min = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                    let max = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                    
                    let step = (max - min) / (num_bins as f64);
                    (0..num_bins + 1).map(|x| min + (x as f64) * step).collect()
                },
                _ => return Err(String::from("Bad input"))
            };

            let counts = comet_algorithms::histogram(&data, &edges);

            let mut output = Vec::<u8>::new();
            let mut encoder = Encoder::from(&mut output);
            // Write the structure
            encoder.push(Header::Map(Some(2))).unwrap();

            encoder.text("counts", None).unwrap();
            encoder.push(Header::Array(Some(counts.len()))).unwrap();
            for count in counts {
                encoder.push(Header::Positive(count)).unwrap();
            }
            
            encoder.text("edges", None).unwrap();
            encoder.push(Header::Array(Some(edges.len()))).unwrap();
            for edge in edges {
                encoder.push(Header::Float(edge)).unwrap();
            }
            encoder.flush().unwrap();
            return Ok(output);
        }
        _ => return Err(String::from("Expected an array of inputs"))
    };
}
