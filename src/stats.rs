pub struct PingStats {
    pub total_packages: i32,
    pub received_packages: i32,
    pub avg_latency: f64,
    pub max_latency: f64,
    pub min_latency: f64,
    pub std_dev_latency: f64,
}

pub struct StatsCalculator {
    loss: i32,
    running_mean: f64,
    running_variance: f64,
    min_latency: f64,
    max_latency: f64,
    count: usize,
}

impl StatsCalculator {
    pub fn new() -> Self {
        StatsCalculator {
            loss: 0,
            running_mean: 0.0,
            running_variance: 0.0,
            min_latency: f64::INFINITY,
            max_latency: f64::NEG_INFINITY,
            count: 0,
        }
    }

    pub fn add(&mut self, latency: f64) {
        self.count += 1;

        // Update min/max
        self.min_latency = self.min_latency.min(latency);
        self.max_latency = self.max_latency.max(latency);

        // Update running mean and variance using Welford's algorithm
        let old_mean = self.running_mean;
        self.running_mean += (latency - old_mean) / self.count as f64;
        self.running_variance += (latency - old_mean) * (latency - self.running_mean);
    }

    pub fn add_loss(&mut self) {
        self.loss += 1;
    }

    pub fn std_dev(&self) -> f64 {
        if self.count < 2 {
            return 0.0;
        }
        (self.running_variance / (self.count as f64 - 1.0)).sqrt()
    }

    pub fn get_result(&self) -> PingStats {
        if self.count == 0 {
            return PingStats {
                total_packages: self.loss,
                received_packages: 0,
                avg_latency: 0.0,
                max_latency: 0.0,
                min_latency: 0.0,
                std_dev_latency: 0.0,
            };
        }

        PingStats {
            total_packages: self.count as i32 + self.loss,
            received_packages: self.count as i32,
            avg_latency: self.running_mean,
            max_latency: self.max_latency,
            min_latency: self.min_latency,
            std_dev_latency: self.std_dev(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stats_calculator() {
        let stats = StatsCalculator::new();
        assert_eq!(stats.count, 0);
        assert_eq!(stats.loss, 0);
        assert_eq!(stats.running_mean, 0.0);
        assert_eq!(stats.running_variance, 0.0);
        assert_eq!(stats.min_latency, f64::INFINITY);
        assert_eq!(stats.max_latency, f64::NEG_INFINITY);
    }

    #[test]
    fn test_single_latency() {
        let mut stats = StatsCalculator::new();
        stats.add(100.0);

        let result = stats.get_result();
        assert_eq!(result.total_packages, 1);
        assert_eq!(result.received_packages, 1);
        assert_eq!(result.avg_latency, 100.0);
        assert_eq!(result.min_latency, 100.0);
        assert_eq!(result.max_latency, 100.0);
        assert_eq!(result.std_dev_latency, 0.0); // Single value has no std dev
    }

    #[test]
    fn test_multiple_latencies() {
        let mut stats = StatsCalculator::new();
        stats.add(10.0);
        stats.add(20.0);
        stats.add(30.0);

        let result = stats.get_result();
        assert_eq!(result.total_packages, 3);
        assert_eq!(result.received_packages, 3);
        assert_eq!(result.avg_latency, 20.0);
        assert_eq!(result.min_latency, 10.0);
        assert_eq!(result.max_latency, 30.0);

        // Standard deviation for [10, 20, 30] should be 10.0
        assert!((result.std_dev_latency - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_with_packet_loss() {
        let mut stats = StatsCalculator::new();
        stats.add(50.0);
        stats.add_loss();
        stats.add(100.0);
        stats.add_loss();

        let result = stats.get_result();
        assert_eq!(result.total_packages, 4); // 2 successful + 2 losses
        assert_eq!(result.received_packages, 2);
        assert_eq!(result.avg_latency, 75.0); // (50 + 100) / 2
        assert_eq!(result.min_latency, 50.0);
        assert_eq!(result.max_latency, 100.0);
    }

    #[test]
    fn test_only_losses() {
        let mut stats = StatsCalculator::new();
        stats.add_loss();
        stats.add_loss();
        stats.add_loss();

        let result = stats.get_result();
        assert_eq!(result.total_packages, 3);
        assert_eq!(result.received_packages, 0);
        assert_eq!(result.avg_latency, 0.0);
        assert_eq!(result.min_latency, 0.0);
        assert_eq!(result.max_latency, 0.0);
        assert_eq!(result.std_dev_latency, 0.0);
    }

    #[test]
    fn test_std_dev_calculation() {
        let mut stats = StatsCalculator::new();
        // Add values where we know the expected std dev
        // For values [2, 4, 4, 4, 5, 5, 7, 9], std dev ≈ 2.138
        let values = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        for val in values {
            stats.add(val);
        }

        let result = stats.get_result();
        assert_eq!(result.avg_latency, 5.0);
        assert!((result.std_dev_latency - 2.138089935299395).abs() < 1e-10);
    }

    #[test]
    fn test_edge_case_very_small_values() {
        let mut stats = StatsCalculator::new();
        stats.add(0.001);
        stats.add(0.002);
        stats.add(0.003);

        let result = stats.get_result();
        assert!((result.avg_latency - 0.002).abs() < 1e-10);
        assert!((result.min_latency - 0.001).abs() < 1e-10);
        assert!((result.max_latency - 0.003).abs() < 1e-10);
    }

    #[test]
    fn test_edge_case_large_values() {
        let mut stats = StatsCalculator::new();
        stats.add(1000000.0);
        stats.add(2000000.0);

        let result = stats.get_result();
        assert_eq!(result.avg_latency, 1500000.0);
        assert_eq!(result.min_latency, 1000000.0);
        assert_eq!(result.max_latency, 2000000.0);
    }

    #[test]
    fn test_running_statistics_accuracy() {
        // Test that running statistics match batch calculations
        let mut stats = StatsCalculator::new();
        let values = vec![1.5, 2.3, 4.7, 3.1, 5.9, 2.8, 4.2, 3.6, 1.9, 5.1];

        for val in &values {
            stats.add(*val);
        }

        // Manual calculation for verification
        let sum: f64 = values.iter().sum();
        let mean = sum / values.len() as f64;
        let variance =
            values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (values.len() - 1) as f64;
        let std_dev = variance.sqrt();

        let result = stats.get_result();
        assert!((result.avg_latency - mean).abs() < 1e-10);
        assert!((result.std_dev_latency - std_dev).abs() < 1e-10);
    }
}
