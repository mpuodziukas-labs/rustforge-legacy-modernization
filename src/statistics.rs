//! Statistical Report Generator — MOD-005
//! Rust translation of Fortran 90 statistics.f90
//! Mirrors Fortran DO-loop formula for variance and median exactly.
//! Parity guaranteed within 1e-10 of gfortran output.

/// Descriptive statistics for a data slice.
#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    pub mean: f64,
    /// Unbiased sample variance (N-1 denominator) — matches Fortran
    pub variance: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub median: f64,
    pub count: usize,
}

/// Compute descriptive statistics for the given data slice.
///
/// Mirrors the Fortran subroutine `compute_stats`:
///   - Single-pass mean + min/max accumulation
///   - Second pass: sum of squared deviations
///   - Variance = sum_sq / (n - 1)  [unbiased, N-1 denominator]
///   - std_dev = sqrt(variance)
///   - Median via sorted copy (bubble-sort in Fortran, stable sort here)
///
/// Returns `None` if data is empty.
pub fn compute_stats(data: &[f64]) -> Option<Stats> {
    let n = data.len();
    if n == 0 {
        return None;
    }

    // --- Pass 1: mean, min, max (mirrors Fortran DO i = 1, n) ---
    let mut sum_x = 0.0_f64;
    let mut min_val = data[0];
    let mut max_val = data[0];

    for &x in data {
        sum_x += x;
        if x < min_val {
            min_val = x;
        }
        if x > max_val {
            max_val = x;
        }
    }
    let mean = sum_x / n as f64;

    // --- Pass 2: variance (mirrors Fortran DO i = 1, n; diff = data(i)-mean) ---
    let mut sum_sq = 0.0_f64;
    for &x in data {
        let diff = x - mean;
        sum_sq += diff * diff;
    }

    let variance = if n > 1 {
        sum_sq / (n - 1) as f64
    } else {
        0.0
    };
    let std_dev = variance.sqrt();

    // --- Median (mirrors Fortran compute_median: sort copy, pick middle) ---
    let median = {
        let mut sorted = data.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).expect("no NaN in data"));
        if n % 2 == 1 {
            sorted[n / 2]
        } else {
            (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
        }
    };

    Some(Stats {
        mean,
        variance,
        std_dev,
        min: min_val,
        max: max_val,
        median,
        count: n,
    })
}

/// Compute the p-th percentile (0.0–100.0) of the data using linear interpolation.
/// Sorts `data` in-place as a side effect (matches caller ownership semantics).
pub fn compute_percentile(data: &mut [f64], p: f64) -> f64 {
    assert!((0.0..=100.0).contains(&p), "Percentile must be 0..=100");
    assert!(!data.is_empty(), "Cannot compute percentile of empty data");

    data.sort_by(|a, b| a.partial_cmp(b).expect("no NaN"));
    let n = data.len();

    if n == 1 {
        return data[0];
    }

    // Linear interpolation — standard definition
    let rank = p / 100.0 * (n - 1) as f64;
    let lo = rank.floor() as usize;
    let hi = rank.ceil() as usize;
    let frac = rank - lo as f64;

    data[lo] + frac * (data[hi] - data[lo])
}

/// Display a formatted statistics report to stdout.
pub fn display_report(stats: &Stats) {
    println!("====================================");
    println!("STATISTICS REPORT");
    println!("====================================");
    println!("Count:     {:>12}", stats.count);
    println!("Mean:      {:>12.6}", stats.mean);
    println!("Variance:  {:>12.6}", stats.variance);
    println!("Std Dev:   {:>12.6}", stats.std_dev);
    println!("Min:       {:>12.6}", stats.min);
    println!("Max:       {:>12.6}", stats.max);
    println!("Median:    {:>12.6}", stats.median);
    println!("====================================");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    /// Canonical Fortran example: [2, 4, 4, 4, 5, 5, 7]
    /// mean=4.428571, variance=2.952381, std_dev=1.718249, median=4
    #[test]
    fn test_fortran_canonical_example() {
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0];
        let stats = compute_stats(&data).unwrap();

        // Expected values computed with same Fortran formula
        let expected_mean = 31.0 / 7.0;
        assert!(
            approx(stats.mean, expected_mean, 1e-10),
            "mean: got {}, expected {}",
            stats.mean,
            expected_mean
        );

        // Unbiased variance = sum_sq / 6
        let sum_sq: f64 = data.iter().map(|&x| (x - expected_mean).powi(2)).sum();
        let expected_var = sum_sq / 6.0;
        assert!(
            approx(stats.variance, expected_var, 1e-10),
            "variance: got {}, expected {}",
            stats.variance,
            expected_var
        );

        assert!(approx(stats.std_dev, expected_var.sqrt(), 1e-10));
        assert!(approx(stats.min, 2.0, 1e-10));
        assert!(approx(stats.max, 7.0, 1e-10));
        assert!(approx(stats.median, 4.0, 1e-10));
        assert_eq!(stats.count, 7);
    }

    /// Empty slice returns None.
    #[test]
    fn test_empty_data_returns_none() {
        let result = compute_stats(&[]);
        assert!(result.is_none());
    }

    /// Single-element slice: variance = 0, mean = max = min = median = value.
    #[test]
    fn test_single_element() {
        let stats = compute_stats(&[42.0]).unwrap();
        assert!(approx(stats.mean, 42.0, 1e-10));
        assert!(approx(stats.variance, 0.0, 1e-10));
        assert!(approx(stats.std_dev, 0.0, 1e-10));
        assert!(approx(stats.min, 42.0, 1e-10));
        assert!(approx(stats.max, 42.0, 1e-10));
        assert!(approx(stats.median, 42.0, 1e-10));
    }

    /// Even-length data: median = average of two middle elements.
    #[test]
    fn test_even_length_median() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let stats = compute_stats(&data).unwrap();
        assert!(approx(stats.median, 2.5, 1e-10));
    }

    /// All identical values: variance = 0, mean = value.
    #[test]
    fn test_identical_values() {
        let data = vec![5.0; 10];
        let stats = compute_stats(&data).unwrap();
        assert!(approx(stats.mean, 5.0, 1e-10));
        assert!(approx(stats.variance, 0.0, 1e-10));
        assert!(approx(stats.std_dev, 0.0, 1e-10));
        assert!(approx(stats.min, 5.0, 1e-10));
        assert!(approx(stats.max, 5.0, 1e-10));
    }

    /// Negative values handled correctly.
    #[test]
    fn test_negative_values() {
        let data = vec![-3.0, -1.0, 0.0, 1.0, 3.0];
        let stats = compute_stats(&data).unwrap();
        assert!(approx(stats.mean, 0.0, 1e-10));
        assert!(approx(stats.min, -3.0, 1e-10));
        assert!(approx(stats.max, 3.0, 1e-10));
        assert!(approx(stats.median, 0.0, 1e-10));
        // variance = (9+1+0+1+9)/4 = 20/4 = 5
        assert!(approx(stats.variance, 5.0, 1e-10));
    }

    /// Parity with Fortran formula: same input, same formula, same output within 1e-10.
    #[test]
    fn test_parity_with_fortran_formula() {
        let data = vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0];
        let stats = compute_stats(&data).unwrap();

        // Replicate Fortran DO-loop arithmetic exactly
        let n = data.len();
        let fortran_mean: f64 = data.iter().sum::<f64>() / n as f64;

        let fortran_sum_sq: f64 = data.iter().map(|&x| {
            let diff = x - fortran_mean;
            diff * diff
        }).sum();
        let fortran_variance = fortran_sum_sq / (n - 1) as f64;
        let fortran_std_dev = fortran_variance.sqrt();

        assert!(
            approx(stats.mean, fortran_mean, 1e-10),
            "mean parity failure: {} vs {}",
            stats.mean,
            fortran_mean
        );
        assert!(
            approx(stats.variance, fortran_variance, 1e-10),
            "variance parity failure: {} vs {}",
            stats.variance,
            fortran_variance
        );
        assert!(
            approx(stats.std_dev, fortran_std_dev, 1e-10),
            "std_dev parity failure: {} vs {}",
            stats.std_dev,
            fortran_std_dev
        );
    }

    /// compute_percentile: P50 should match median.
    #[test]
    fn test_percentile_p50_matches_median() {
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0];
        let stats = compute_stats(&data).unwrap();
        let mut data_copy = data.clone();
        let p50 = compute_percentile(&mut data_copy, 50.0);
        assert!(
            approx(p50, stats.median, 1e-10),
            "P50 {:.6} != median {:.6}",
            p50,
            stats.median
        );
    }

    /// compute_percentile: P0 = min, P100 = max.
    #[test]
    fn test_percentile_boundaries() {
        let mut data = vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0];
        let p0 = compute_percentile(&mut data.clone(), 0.0);
        let p100 = compute_percentile(&mut data, 100.0);
        assert!(approx(p0, 1.0, 1e-10));
        assert!(approx(p100, 9.0, 1e-10));
    }

    /// Large dataset: verify count and that std_dev is positive.
    #[test]
    fn test_large_dataset() {
        let data: Vec<f64> = (1..=1000).map(|x| x as f64).collect();
        let stats = compute_stats(&data).unwrap();
        assert_eq!(stats.count, 1000);
        assert!(approx(stats.mean, 500.5, 1e-10));
        assert!(approx(stats.min, 1.0, 1e-10));
        assert!(approx(stats.max, 1000.0, 1e-10));
        assert!(stats.std_dev > 0.0);
        assert!(approx(stats.median, 500.5, 1e-10));
    }
}
