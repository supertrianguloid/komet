
pub fn histogram(
    data: &[f64],
    edges: &[f64]
) -> Vec<u64> {
    let mut counts = vec![0u64; edges.len() - 1];
    for &elem in data {
        let prev = edges[0];
        for (i, &edge) in edges[1..].iter().enumerate() {
            if prev <= elem && elem < edge {
                counts[i] += 1;
                break;
            }
        }
    }

    counts
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(histogram(&vec![2.,4.,4.], &vec![1.,3.,5.]), vec![1,2]);
    }

}