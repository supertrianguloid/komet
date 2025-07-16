use ciborium_ll::{Decoder, Header};
use rustfft::num_complex::Complex64;

pub fn read_float_array<R: std::io::Read>(decoder: &mut Decoder<R>, len: usize) -> Vec<f64> {
    let mut values = Vec::<f64>::with_capacity(len);
    for _ in 0..len {
        match decoder.pull().unwrap() {
            Header::Float(x) => values.push(x),
            _ => panic!("oof"),
        }
    }
    values
}

pub fn read_complex_array<R: std::io::Read>(
    decoder: &mut Decoder<R>,
    len: usize,
) -> Result<Vec<Complex64>, String> {
    let mut values = vec![Complex64 { re: 0.0, im: 0.0 }; len];

    for value in values.iter_mut() {
        *value = match decoder.pull().unwrap() {
            Header::Array(Some(2)) => {
                let re = match decoder.pull().unwrap() {
                    Header::Float(x) => x,
                    _ => return Err(String::from("Bad input")),
                };

                let im = match decoder.pull().unwrap() {
                    Header::Float(x) => x,
                    _ => return Err(String::from("Bad input")),
                };
                Complex64 { re, im }
            }
            _ => return Err(String::from("Bad input")),
        };
    }
    Ok(values)
}
