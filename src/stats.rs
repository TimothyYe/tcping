pub struct PingStats {
    pub total_packages: i32,
    pub received_packages: i32,
    pub avg_latency: f64,
    pub max_latency: f64,
    pub min_latency: f64,
    pub std_dev_latency: f64,
}

pub struct StatsCalculator {
    latencies: Vec<f64>,
    loss: i32,
}

impl StatsCalculator {
    pub fn new() -> Self {
        StatsCalculator {
            latencies: Vec::new(),
            loss: 0,
        }
    }

    pub fn add(&mut self, latency: f64) {
        self.latencies.push(latency);
    }

    pub fn add_loss(&mut self) {
        self.loss += 1;
    }

    pub fn std_dev(&self) -> f64 {
        let n = self.latencies.len() as f64;
        if n < 2.0 {
            return 0.0;
        }

        // Use more numerically stable Welford's online algorithm
        let mut mean = self.latencies[0];
        let mut s = 0.0;

        for i in 1..self.latencies.len() {
            let x = self.latencies[i];
            let old_mean = mean;
            mean += (x - mean) / (i as f64 + 1.0);
            s += (x - mean) * (x - old_mean);
        }

        (s / (n - 1.0)).sqrt()
    }

    pub fn get_result(&self) -> PingStats {
        let count = self.latencies.len();

        if count == 0 {
            return PingStats {
                total_packages: self.loss,
                received_packages: 0,
                avg_latency: 0.0,
                max_latency: 0.0,
                min_latency: 0.0,
                std_dev_latency: 0.0,
            };
        }

        let count_f64 = count as f64;
        // Calculate statistics in a single pass
        let mut sum = 0.0;
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        for &latency in &self.latencies {
            sum += latency;
            min = min.min(latency);
            max = max.max(latency);
        }

        let avg = sum / count_f64;

        PingStats {
            total_packages: count as i32 + self.loss,
            received_packages: count as i32,
            avg_latency: avg,
            max_latency: max,
            min_latency: min,
            std_dev_latency: self.std_dev(),
        }
    }
}
