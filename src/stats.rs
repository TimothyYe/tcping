pub struct PingStats {
    pub total_packages: i32,
    pub received_packages: i32,
    pub avg_latency: f64,
    pub max_latency: f64,
    pub min_latency: f64,
    pub stddev_latency: f64,
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

    pub fn stddev(&self) -> f64 {
        let n = self.latencies.len() as f64;
        if n < 2.0 {
            return 0.0;
        }
        let mut mean = 0.0;
        let mut m2 = 0.0;
        for (i, &x) in self.latencies.iter().enumerate() {
            let delta = x - mean;
            mean += delta / (i as f64 + 1.0);
            m2 += delta * (x - mean);
        }
        (m2 / (n - 1.0)).sqrt()
    }

    pub fn get_result(&self) -> PingStats {
        let sum: f64 = self.latencies.iter().sum();
        let count = self.latencies.len() as f64;
        let avg = sum / count;
        let max = self
            .latencies
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let min = self.latencies.iter().cloned().fold(f64::INFINITY, f64::min);

        PingStats {
            total_packages: count as i32 + self.loss,
            received_packages: count as i32,
            avg_latency: avg,
            max_latency: max,
            min_latency: min,
            stddev_latency: self.stddev(),
        }
    }
}
