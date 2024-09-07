pub struct StatsCalculator {
    latencies: Vec<f64>,
}

impl StatsCalculator {
    pub fn new() -> Self {
        StatsCalculator {
            latencies: Vec::new(),
        }
    }

    pub fn add(&mut self, latency: f64) {
        self.latencies.push(latency);
    }

    pub fn get_result(&self) -> (i32, f64, f64, f64) {
        let sum: f64 = self.latencies.iter().sum();
        let count = self.latencies.len() as f64;
        let avg = sum / count;
        let max = self
            .latencies
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let min = self.latencies.iter().cloned().fold(f64::INFINITY, f64::min);
        (count as i32, avg, max, min)
    }
}
