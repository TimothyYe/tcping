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
