use ciborium_ll::{Decoder, Header};


pub fn read_float_array<R: std::io::Read>(decoder: &mut Decoder<R>, len: usize) -> Vec<f64> {
    let mut data = Vec::<f64>::new();
    data.reserve(len);
    for _ in 0..len {
        match decoder.pull().unwrap() {
            Header::Float(x) => data.push(x),
            _ => panic!("oof"),
        }
    }
    data
}