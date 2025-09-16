use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GamePrediction {
    pub id: String,
    pub game_id: String,
    pub home_score_distribution: ProbabilityDistribution,
    pub away_score_distribution: ProbabilityDistribution,
    pub spread_prediction: f64,
    pub total_prediction: f64,
    pub confidence_interval: ConfidenceInterval,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProbabilityDistribution {
    pub mean: f64,
    pub std_dev: f64,
    pub samples: Vec<f64>,
    pub percentiles: HashMap<u8, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfidenceInterval {
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub confidence_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McmcParameters {
    pub num_samples: usize,
    pub burn_in: usize,
    pub chains: usize,
    pub convergence_threshold: f64,
    pub step_size: f64,
    pub target_acceptance_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McmcDiagnostics {
    pub r_hat: f64, // Gelman-Rubin statistic
    pub effective_sample_size: f64,
    pub acceptance_rate: f64,
    pub converged: bool,
    pub chains_analyzed: usize,
    pub total_samples: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameWithPrediction {
    pub game_id: String,
    pub home_team_name: String,
    pub away_team_name: String,
    pub game_time: DateTime<Utc>,
    pub prediction: Option<GamePrediction>,
    pub confidence_score: f64,
    pub last_updated: DateTime<Utc>,
}

impl GamePrediction {
    pub fn new(
        game_id: String,
        home_score_distribution: ProbabilityDistribution,
        away_score_distribution: ProbabilityDistribution,
    ) -> Self {
        let spread_prediction = home_score_distribution.mean - away_score_distribution.mean;
        let total_prediction = home_score_distribution.mean + away_score_distribution.mean;
        
        // Calculate confidence interval for the spread
        let spread_variance = home_score_distribution.variance() + away_score_distribution.variance();
        let spread_std_dev = spread_variance.sqrt();
        let confidence_interval = ConfidenceInterval {
            lower_bound: spread_prediction - 1.96 * spread_std_dev,
            upper_bound: spread_prediction + 1.96 * spread_std_dev,
            confidence_level: 0.95,
        };

        Self {
            id: Uuid::new_v4().to_string(),
            game_id,
            home_score_distribution,
            away_score_distribution,
            spread_prediction,
            total_prediction,
            confidence_interval,
            generated_at: Utc::now(),
        }
    }

    pub fn home_win_probability(&self) -> f64 {
        // Simple approximation: probability that home score > away score
        // In a more sophisticated implementation, this would use the full distributions
        if self.spread_prediction > 0.0 {
            0.5 + (self.spread_prediction / 14.0).min(0.45) // Cap at 95%
        } else {
            0.5 - (self.spread_prediction.abs() / 14.0).min(0.45) // Cap at 5%
        }
    }

    pub fn away_win_probability(&self) -> f64 {
        1.0 - self.home_win_probability()
    }

    pub fn is_high_confidence(&self, threshold: f64) -> bool {
        let interval_width = self.confidence_interval.upper_bound - self.confidence_interval.lower_bound;
        interval_width < threshold
    }

    pub fn get_prediction_summary(&self) -> String {
        format!(
            "Predicted spread: {:.1}, Total: {:.1}, Home win probability: {:.1}%",
            self.spread_prediction,
            self.total_prediction,
            self.home_win_probability() * 100.0
        )
    }
}

impl ProbabilityDistribution {
    pub fn new(samples: Vec<f64>) -> Self {
        let mean = samples.iter().sum::<f64>() / samples.len() as f64;
        let variance = samples
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / samples.len() as f64;
        let std_dev = variance.sqrt();

        let mut percentiles = HashMap::new();
        let mut sorted_samples = samples.clone();
        sorted_samples.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for &p in &[5, 10, 25, 50, 75, 90, 95] {
            let percentile_value = if p == 50 && sorted_samples.len() % 2 == 0 {
                // For median with even number of samples, take average of middle two
                let mid1 = sorted_samples.len() / 2 - 1;
                let mid2 = sorted_samples.len() / 2;
                (sorted_samples[mid1] + sorted_samples[mid2]) / 2.0
            } else {
                let index = ((p as f64 / 100.0) * (sorted_samples.len() - 1) as f64).round() as usize;
                sorted_samples[index]
            };
            percentiles.insert(p, percentile_value);
        }

        Self {
            mean,
            std_dev,
            samples,
            percentiles,
        }
    }

    pub fn variance(&self) -> f64 {
        self.std_dev.powi(2)
    }

    pub fn get_percentile(&self, percentile: u8) -> Option<f64> {
        self.percentiles.get(&percentile).copied()
    }

    pub fn probability_above(&self, threshold: f64) -> f64 {
        self.samples
            .iter()
            .filter(|&&x| x > threshold)
            .count() as f64 / self.samples.len() as f64
    }

    pub fn probability_below(&self, threshold: f64) -> f64 {
        self.samples
            .iter()
            .filter(|&&x| x < threshold)
            .count() as f64 / self.samples.len() as f64
    }

    pub fn probability_between(&self, lower: f64, upper: f64) -> f64 {
        self.samples
            .iter()
            .filter(|&&x| x >= lower && x <= upper)
            .count() as f64 / self.samples.len() as f64
    }
}

impl ConfidenceInterval {
    pub fn new(lower_bound: f64, upper_bound: f64, confidence_level: f64) -> Self {
        Self {
            lower_bound,
            upper_bound,
            confidence_level,
        }
    }

    pub fn width(&self) -> f64 {
        self.upper_bound - self.lower_bound
    }

    pub fn contains(&self, value: f64) -> bool {
        value >= self.lower_bound && value <= self.upper_bound
    }

    pub fn midpoint(&self) -> f64 {
        (self.lower_bound + self.upper_bound) / 2.0
    }
}

impl Default for McmcParameters {
    fn default() -> Self {
        Self {
            num_samples: 10000,
            burn_in: 2000,
            chains: 4,
            convergence_threshold: 1.1, // R-hat threshold
            step_size: 0.1,
            target_acceptance_rate: 0.44,
        }
    }
}

impl McmcParameters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_samples(mut self, num_samples: usize) -> Self {
        self.num_samples = num_samples;
        self
    }

    pub fn with_burn_in(mut self, burn_in: usize) -> Self {
        self.burn_in = burn_in;
        self
    }

    pub fn with_chains(mut self, chains: usize) -> Self {
        self.chains = chains;
        self
    }

    pub fn total_samples(&self) -> usize {
        self.num_samples * self.chains
    }

    pub fn effective_samples(&self) -> usize {
        (self.num_samples - self.burn_in) * self.chains
    }
}

impl McmcDiagnostics {
    pub fn new(
        r_hat: f64,
        effective_sample_size: f64,
        acceptance_rate: f64,
        chains_analyzed: usize,
        total_samples: usize,
    ) -> Self {
        let converged = r_hat < 1.1 && effective_sample_size > 400.0;
        
        Self {
            r_hat,
            effective_sample_size,
            acceptance_rate,
            converged,
            chains_analyzed,
            total_samples,
        }
    }

    pub fn is_converged(&self) -> bool {
        self.converged
    }

    pub fn needs_more_samples(&self) -> bool {
        self.effective_sample_size < 400.0
    }

    pub fn acceptance_rate_ok(&self) -> bool {
        self.acceptance_rate >= 0.2 && self.acceptance_rate <= 0.7
    }

    pub fn get_diagnostics_summary(&self) -> String {
        format!(
            "R-hat: {:.3}, ESS: {:.0}, Acceptance: {:.1}%, Converged: {}",
            self.r_hat,
            self.effective_sample_size,
            self.acceptance_rate * 100.0,
            self.converged
        )
    }
}

impl GameWithPrediction {
    pub fn new(
        game_id: String,
        home_team_name: String,
        away_team_name: String,
        game_time: DateTime<Utc>,
    ) -> Self {
        Self {
            game_id,
            home_team_name,
            away_team_name,
            game_time,
            prediction: None,
            confidence_score: 0.0,
            last_updated: Utc::now(),
        }
    }

    pub fn with_prediction(mut self, prediction: GamePrediction) -> Self {
        // Calculate confidence score based on prediction interval width
        let interval_width = prediction.confidence_interval.width();
        self.confidence_score = (20.0 - interval_width).max(0.0) / 20.0; // Normalize to 0-1
        self.prediction = Some(prediction);
        self.last_updated = Utc::now();
        self
    }

    pub fn has_prediction(&self) -> bool {
        self.prediction.is_some()
    }

    pub fn is_prediction_stale(&self, max_age_hours: i64) -> bool {
        let age = Utc::now() - self.last_updated;
        age > chrono::Duration::hours(max_age_hours)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_samples() -> Vec<f64> {
        vec![18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0]
    }

    #[test]
    fn test_probability_distribution_creation() {
        let samples = create_test_samples();
        let dist = ProbabilityDistribution::new(samples.clone());

        assert_eq!(dist.mean, 22.5);
        assert!((dist.std_dev - 2.872).abs() < 0.01);
        assert_eq!(dist.samples, samples);
        assert!(dist.percentiles.contains_key(&50));
    }

    #[test]
    fn test_probability_distribution_percentiles() {
        let samples = create_test_samples();
        let dist = ProbabilityDistribution::new(samples);

        assert_eq!(dist.get_percentile(50), Some(22.5)); // Median (average of 22.0 and 23.0)
        assert_eq!(dist.get_percentile(5), Some(18.0));  // 5th percentile
        assert_eq!(dist.get_percentile(95), Some(27.0)); // 95th percentile
    }

    #[test]
    fn test_probability_distribution_probability_calculations() {
        let samples = create_test_samples();
        let dist = ProbabilityDistribution::new(samples);

        assert_eq!(dist.probability_above(25.0), 0.2); // 2 out of 10 samples > 25 (26.0, 27.0)
        assert_eq!(dist.probability_below(20.0), 0.2); // 2 out of 10 samples < 20 (18.0, 19.0)
        assert_eq!(dist.probability_between(20.0, 25.0), 0.6); // 6 out of 10 samples between 20-25 (20.0, 21.0, 22.0, 23.0, 24.0, 25.0)
    }

    #[test]
    fn test_game_prediction_creation() {
        let home_samples = vec![22.0, 23.0, 24.0, 25.0, 26.0];
        let away_samples = vec![18.0, 19.0, 20.0, 21.0, 22.0];
        
        let home_dist = ProbabilityDistribution::new(home_samples);
        let away_dist = ProbabilityDistribution::new(away_samples);
        
        let prediction = GamePrediction::new("game-1".to_string(), home_dist, away_dist);

        assert_eq!(prediction.game_id, "game-1");
        assert_eq!(prediction.spread_prediction, 4.0); // 24.0 - 20.0
        assert_eq!(prediction.total_prediction, 44.0); // 24.0 + 20.0
        assert!(prediction.home_win_probability() > 0.5);
        assert!(!prediction.id.is_empty());
    }

    #[test]
    fn test_game_prediction_win_probabilities() {
        let home_samples = vec![28.0, 29.0, 30.0, 31.0, 32.0]; // Higher scoring
        let away_samples = vec![18.0, 19.0, 20.0, 21.0, 22.0]; // Lower scoring
        
        let home_dist = ProbabilityDistribution::new(home_samples);
        let away_dist = ProbabilityDistribution::new(away_samples);
        
        let prediction = GamePrediction::new("game-1".to_string(), home_dist, away_dist);

        let home_prob = prediction.home_win_probability();
        let away_prob = prediction.away_win_probability();

        assert!(home_prob > 0.7); // Home team heavily favored
        assert!(away_prob < 0.3); // Away team underdog
        assert!((home_prob + away_prob - 1.0).abs() < 0.001); // Should sum to 1.0
    }

    #[test]
    fn test_confidence_interval() {
        let ci = ConfidenceInterval::new(-2.0, 6.0, 0.95);

        assert_eq!(ci.width(), 8.0);
        assert_eq!(ci.midpoint(), 2.0);
        assert!(ci.contains(0.0));
        assert!(ci.contains(5.0));
        assert!(!ci.contains(-3.0));
        assert!(!ci.contains(7.0));
    }

    #[test]
    fn test_mcmc_parameters() {
        let params = McmcParameters::new()
            .with_samples(5000)
            .with_burn_in(1000)
            .with_chains(2);

        assert_eq!(params.num_samples, 5000);
        assert_eq!(params.burn_in, 1000);
        assert_eq!(params.chains, 2);
        assert_eq!(params.total_samples(), 10000);
        assert_eq!(params.effective_samples(), 8000);
    }

    #[test]
    fn test_mcmc_diagnostics() {
        let diagnostics = McmcDiagnostics::new(1.05, 500.0, 0.44, 4, 10000);

        assert!(diagnostics.is_converged());
        assert!(!diagnostics.needs_more_samples());
        assert!(diagnostics.acceptance_rate_ok());

        let summary = diagnostics.get_diagnostics_summary();
        assert!(summary.contains("R-hat: 1.050"));
        assert!(summary.contains("ESS: 500"));
        assert!(summary.contains("Converged: true"));
    }

    #[test]
    fn test_mcmc_diagnostics_not_converged() {
        let diagnostics = McmcDiagnostics::new(1.5, 200.0, 0.1, 4, 10000);

        assert!(!diagnostics.is_converged());
        assert!(diagnostics.needs_more_samples());
        assert!(!diagnostics.acceptance_rate_ok());
    }

    #[test]
    fn test_game_with_prediction() {
        let game_time = Utc::now() + chrono::Duration::hours(2);
        let mut game_with_pred = GameWithPrediction::new(
            "game-1".to_string(),
            "Chiefs".to_string(),
            "Bills".to_string(),
            game_time,
        );

        assert!(!game_with_pred.has_prediction());
        assert_eq!(game_with_pred.confidence_score, 0.0);

        let home_dist = ProbabilityDistribution::new(vec![22.0, 23.0, 24.0, 25.0, 26.0]);
        let away_dist = ProbabilityDistribution::new(vec![18.0, 19.0, 20.0, 21.0, 22.0]);
        let prediction = GamePrediction::new("game-1".to_string(), home_dist, away_dist);

        game_with_pred = game_with_pred.with_prediction(prediction);

        assert!(game_with_pred.has_prediction());
        assert!(game_with_pred.confidence_score > 0.0);
        assert!(!game_with_pred.is_prediction_stale(24));
    }

    #[test]
    fn test_prediction_staleness() {
        let game_time = Utc::now() + chrono::Duration::hours(2);
        let mut game_with_pred = GameWithPrediction::new(
            "game-1".to_string(),
            "Chiefs".to_string(),
            "Bills".to_string(),
            game_time,
        );

        // Manually set last_updated to past
        game_with_pred.last_updated = Utc::now() - chrono::Duration::hours(25);

        assert!(game_with_pred.is_prediction_stale(24));
        assert!(!game_with_pred.is_prediction_stale(48));
    }

    #[test]
    fn test_serialization() {
        let home_dist = ProbabilityDistribution::new(vec![22.0, 23.0, 24.0, 25.0, 26.0]);
        let away_dist = ProbabilityDistribution::new(vec![18.0, 19.0, 20.0, 21.0, 22.0]);
        let prediction = GamePrediction::new("game-1".to_string(), home_dist, away_dist);

        let serialized = serde_json::to_string(&prediction).expect("Failed to serialize");
        let deserialized: GamePrediction = serde_json::from_str(&serialized).expect("Failed to deserialize");

        assert_eq!(prediction, deserialized);
    }

    #[test]
    fn test_high_confidence_prediction() {
        let home_samples = vec![24.0, 24.1, 24.2, 24.3, 24.4]; // Very tight distribution
        let away_samples = vec![20.0, 20.1, 20.2, 20.3, 20.4]; // Very tight distribution
        
        let home_dist = ProbabilityDistribution::new(home_samples);
        let away_dist = ProbabilityDistribution::new(away_samples);
        
        let prediction = GamePrediction::new("game-1".to_string(), home_dist, away_dist);

        assert!(prediction.is_high_confidence(2.0)); // Narrow confidence interval
        
        let summary = prediction.get_prediction_summary();
        assert!(summary.contains("Predicted spread"));
        assert!(summary.contains("Total"));
        assert!(summary.contains("Home win probability"));
    }
}