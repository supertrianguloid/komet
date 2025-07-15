#[derive(Debug, PartialEq)]
pub struct BoxplotStats {
    pub mean: f64,
    pub median: f64,
    pub q1: f64,
    pub q3: f64,
    pub min: f64,
    pub max: f64,
    pub outliers: Vec<f64>,
    pub whisker_low: f64,
    pub whisker_high: f64,
}

/// Computes the value at a rational index into an array by performing linear
/// interpolation. If the index is an integer, the exact value at the index is
/// returned.
fn interpolate(values: &[f64], index: f64) -> f64 {
    assert!(values.len() > 0);
    if index < 0. {
        values[0]
    } else if index >= values.len() as f64 - 1. {
        *values.last().unwrap()
    } else {
        let lower = index.floor() as usize;
        let upper = lower + 1;
        let t = index - (lower as f64);

        values[lower] * (1. - t) + values[upper] * t
    }
}

fn percentile(values: &[f64], q: f64) -> f64 {
    interpolate(values, q / 100. * (values.len() as f64 - 1.))
}

pub fn boxplot(values: &[f64], whiskers: f64) -> BoxplotStats {
    let mut sorted_values = values.to_vec();
    sorted_values.sort_by(f64::total_cmp);

    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let median = percentile(&sorted_values, 50.);
    let q1 = percentile(&sorted_values, 25.);
    let q3 = percentile(&sorted_values, 75.);

    let iqr = q3 - q1;
    let whisker_low = sorted_values
        .iter()
        .find(|&&x| x >= q1 - iqr * whiskers)
        .unwrap_or(&q1)
        .min(q1);
    let whisker_high = sorted_values
        .iter()
        .rev()
        .find(|&&x| x <= q3 + iqr * whiskers)
        .unwrap_or(&q3)
        .max(q3);

    BoxplotStats {
        mean: mean,
        median: median,
        q1: q1,
        q3: q3,
        min: sorted_values[0],
        max: sorted_values.last().unwrap().clone(),
        whisker_low: whisker_low,
        whisker_high: whisker_high,
        outliers: values
            .iter()
            .filter(|&&x| x < whisker_low || x > whisker_high)
            .cloned()
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            boxplot(&[1., 2., 3., 4.], 1.5),
            BoxplotStats {
                mean: 2.5,
                median: 2.5,
                q1: 1.75,
                q3: 3.25,
                min: 1.,
                max: 4.,
                whisker_low: 1.,
                whisker_high: 4.,
                outliers: vec![],
            }
        );

        assert_eq!(
            boxplot(&[1., 2., 3., 4., -2., 100.], 1.5),
            BoxplotStats {
                mean: 18.0,
                median: 2.5,
                q1: 1.25,
                q3: 3.75,
                min: -2.,
                max: 100.,
                whisker_low: -2.,
                whisker_high: 4.,
                outliers: vec![100.],
            }
        );
    }
}
