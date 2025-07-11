use ciborium_io::Write as _;
use ciborium_ll::{Decoder, Encoder, Header};
use comet_algorithms;
use rustfft::{FftDirection, FftPlanner};
use wasm_minimal_protocol::*;
use String;
initiate_protocol!();

mod read;

#[wasm_func]
pub fn contour(input: &[u8]) -> Result<Vec<u8>, String> {
    let mut decoder = Decoder::from(&input[..]);

    match decoder.pull().unwrap() {
        Header::Array(Some(len)) => {
            if len != 4 {
                return Err(String::from("Expected array of 4 elements, got more"));
            }

            let x = match decoder.pull().unwrap() {
                Header::Array(Some(len)) => read::read_float_array(&mut decoder, len),
                _ => return Err(String::from("Bad input")),
            };

            let y = match decoder.pull().unwrap() {
                Header::Array(Some(len)) => read::read_float_array(&mut decoder, len),
                _ => return Err(String::from("Bad input")),
            };

            let z = match decoder.pull().unwrap() {
                Header::Array(Some(len)) => read::read_float_array(&mut decoder, len),
                _ => return Err(String::from("Bad input")),
            };

            let levels = match decoder.pull().unwrap() {
                Header::Array(Some(len)) => read::read_float_array(&mut decoder, len),
                _ => return Err(String::from("Bad input")),
            };

            let contours = comet_algorithms::contour(&x, &y, &z, &levels);

            let mut output = Vec::<u8>::new();
            let mut encoder = Encoder::from(&mut output);
            // Write the structure
            encoder.push(Header::Map(Some(1))).unwrap();

            encoder.text("contours", None).unwrap();
            encoder.push(Header::Array(Some(contours.len()))).unwrap();
            for contour in contours {
                encoder.push(Header::Array(Some(contour.len()))).unwrap();
                for contour_line in contour {
                    encoder
                        .push(Header::Array(Some(contour_line.len())))
                        .unwrap();
                    for vertex in contour_line {
                        encoder.push(Header::Array(Some(2))).unwrap();
                        encoder.push(Header::Float(vertex.x)).unwrap();
                        encoder.push(Header::Float(vertex.y)).unwrap();
                    }
                }
            }

            encoder.flush().unwrap();
            return Ok(output);
            // return Ok(b"Hello from wasm!!!".to_vec())
        }
        _ => return Err(String::from("Expected an array of inputs")),
    };
}

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
                _ => return Err(String::from("Bad input")),
            };

            let edges = match decoder.pull().unwrap() {
                Header::Array(Some(len)) => read::read_float_array(&mut decoder, len),
                Header::Positive(num_bins) => {
                    let min = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                    let max = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

                    let step = (max - min) / (num_bins as f64);
                    (0..num_bins + 1).map(|x| min + (x as f64) * step).collect()
                }
                _ => return Err(String::from("Bad input")),
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
        _ => return Err(String::from("Expected an array of inputs")),
    };
}

fn fft_impl(input: &[u8], direction: FftDirection) -> Result<Vec<u8>, String> {
    let mut decoder = Decoder::from(&input[..]);

    let mut data = match decoder.pull().unwrap() {
        Header::Array(Some(len)) => read::read_complex_array(&mut decoder, len).unwrap(),
        _ => return Err(String::from("Expected an array of inputs")),
    };

    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft(data.len(), direction);
    fft.process(&mut data);
    if direction == FftDirection::Inverse {
        let normalization = 1. / (data.len() as f64);
        for elem in data.iter_mut() {
            *elem *= normalization;
        }
    }

    let mut output = Vec::<u8>::new();
    let mut encoder = Encoder::from(&mut output);

    encoder.push(Header::Array(Some(data.len()))).unwrap();
    for elem in data {
        encoder.push(Header::Array(Some(2))).unwrap();
        encoder.push(Header::Float(elem.re)).unwrap();
        encoder.push(Header::Float(elem.im)).unwrap();
    }

    Ok(output)
}

#[wasm_func]
fn fft(input: &[u8]) -> Result<Vec<u8>, String> {
    fft_impl(&input, FftDirection::Forward)
}

#[wasm_func]
fn ifft(input: &[u8]) -> Result<Vec<u8>, String> {
    fft_impl(&input, FftDirection::Inverse)
}
