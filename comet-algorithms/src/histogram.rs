pub fn histogram(data: &[f64], edges: &[f64]) -> Vec<u64> {
    let mut counts = vec![0u64; edges.len() - 1];
    for &elem in data {
        if elem < edges[0] { continue; }

        for (i, &edge) in edges[1..].iter().enumerate() {
            if elem < edge || (elem == edge && i == data.len() - 2) {
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
    fn test_histogram() {
        assert_eq!(histogram(&vec![2., 4., 4.], &vec![1., 3., 5.]), vec![1, 2]);
    }

    #[test]
    fn test_upper_edge() {
        assert_eq!(histogram(&vec![0., 2., 4.], &vec![0., 2., 4.]), vec![1, 2]);
    }
}
