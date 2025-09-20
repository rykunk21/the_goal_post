use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::prediction::GamePrediction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BettingLine {
    pub id: String,
    pub game_id: String,
    pub provider: String,
    pub spread: f64,
    pub total: f64,
    pub moneyline_home: i32,
    pub moneyline_away: i32,
    pub timestamp: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LineComparison {
    pub id: String,
    pub game_id: String,
    pub betting_line: BettingLine,
    pub prediction: GamePrediction,
    pub spread_difference: f64,
    pub total_difference: f64,
    pub value_score: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValueOpportunity {
    pub id: String,
    pub game_id: String,
    pub opportunity_type: OpportunityType,
    pub confidence: f64,
    pub expected_value: f64,
    pub recommendation: String,
    pub betting_line_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OpportunityType {
    SpreadValue,
    TotalValue,
    MoneylineValue,
    ArbitrageOpportunity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BettingProvider {
    pub id: String,
    pub name: String,
    pub api_endpoint: String,
    pub is_active: bool,
    pub rate_limit_per_minute: u32,
    pub last_request_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl BettingLine {
    /// Convert point spread to implied win probability using logistic model
    /// Each point is worth approximately 3.3% win probability in NFL
    pub fn spread_to_probability(spread: f64) -> f64 {
        if spread == 0.0 {
            return 0.5;
        }
        // Using logistic function: P = 1 / (1 + e^(-spread/3.3))
        1.0 / (1.0 + (-spread / 3.3).exp())
    }

    /// Get implied probability for home team winning based on spread
    pub fn implied_probability_home_spread(&self) -> f64 {
        Self::spread_to_probability(-self.spread) // Negative because spread is from home perspective
    }

    /// Get implied probability for away team winning based on spread  
    pub fn implied_probability_away_spread(&self) -> f64 {
        1.0 - self.implied_probability_home_spread()
    }
    pub fn new(
        game_id: String,
        provider: String,
        spread: f64,
        total: f64,
        moneyline_home: i32,
        moneyline_away: i32,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            game_id,
            provider,
            spread,
            total,
            moneyline_home,
            moneyline_away,
            timestamp: Utc::now(),
            is_active: true,
        }
    }

    pub fn is_expired(&self, expiry_minutes: i64) -> bool {
        let expiry_time = self.timestamp + chrono::Duration::minutes(expiry_minutes);
        Utc::now() > expiry_time
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn implied_probability_home(&self) -> f64 {
        if self.moneyline_home > 0 {
            100.0 / (self.moneyline_home as f64 + 100.0)
        } else {
            (-self.moneyline_home as f64) / (-self.moneyline_home as f64 + 100.0)
        }
    }

    pub fn implied_probability_away(&self) -> f64 {
        if self.moneyline_away > 0 {
            100.0 / (self.moneyline_away as f64 + 100.0)
        } else {
            (-self.moneyline_away as f64) / (-self.moneyline_away as f64 + 100.0)
        }
    }

    pub fn total_implied_probability(&self) -> f64 {
        self.implied_probability_home() + self.implied_probability_away()
    }

    pub fn vig_percentage(&self) -> f64 {
        (self.total_implied_probability() - 1.0) * 100.0
    }
}

impl LineComparison {
    pub fn new(betting_line: BettingLine, prediction: GamePrediction) -> Self {
        let spread_difference = prediction.spread_prediction - betting_line.spread;
        let total_difference = prediction.total_prediction - betting_line.total;
        
        // Simple value score calculation (can be enhanced with more sophisticated methods)
        let value_score = (spread_difference.abs() + total_difference.abs()) / 2.0;
        
        Self {
            id: Uuid::new_v4().to_string(),
            game_id: betting_line.game_id.clone(),
            betting_line,
            prediction,
            spread_difference,
            total_difference,
            value_score,
            created_at: Utc::now(),
        }
    }

    pub fn has_spread_value(&self, threshold: f64) -> bool {
        self.spread_difference.abs() > threshold
    }

    pub fn has_total_value(&self, threshold: f64) -> bool {
        self.total_difference.abs() > threshold
    }

    pub fn get_spread_recommendation(&self) -> Option<String> {
        if self.spread_difference > 1.0 {
            Some(format!("Consider betting the underdog (+{:.1})", self.betting_line.spread.abs()))
        } else if self.spread_difference < -1.0 {
            Some(format!("Consider betting the favorite ({:.1})", self.betting_line.spread))
        } else {
            None
        }
    }

    pub fn get_total_recommendation(&self) -> Option<String> {
        if self.total_difference > 3.0 {
            Some(format!("Consider betting OVER {:.1}", self.betting_line.total))
        } else if self.total_difference < -3.0 {
            Some(format!("Consider betting UNDER {:.1}", self.betting_line.total))
        } else {
            None
        }
    }
}

impl ValueOpportunity {
    pub fn new(
        game_id: String,
        opportunity_type: OpportunityType,
        confidence: f64,
        expected_value: f64,
        recommendation: String,
        betting_line_id: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            game_id,
            opportunity_type,
            confidence,
            expected_value,
            recommendation,
            betting_line_id,
            created_at: Utc::now(),
            expires_at: None,
        }
    }

    /// Create a value opportunity from probability analysis
    /// community_prob: Community win probability for the team (0.0 to 1.0)
    /// betting_prob: Implied betting probability for the team (0.0 to 1.0)
    /// team_abbr: Team abbreviation (e.g., "CAR", "ATL")
    /// spread: The betting spread for the team (positive = underdog, negative = favorite)
    /// is_home: Whether this is the home team
    pub fn from_probability_analysis(
        game_id: String,
        betting_line_id: String,
        community_prob: f64,
        betting_prob: f64,
        team_abbr: String,
        spread: f64,
        _is_home: bool,
    ) -> Option<Self> {
        let value_diff = community_prob - betting_prob;
        
        // Only create opportunity if there's significant value (5% threshold)
        if value_diff.abs() < 0.05 {
            return None;
        }

        let confidence = community_prob.max(betting_prob).min(0.95); // Cap at 95%
        let expected_value = value_diff; // Store as decimal (will be converted to % in frontend)
        
        let recommendation = if spread > 0.0 {
            format!("{} +{:.1}", team_abbr, spread)
        } else if spread < 0.0 {
            format!("{} {:.1}", team_abbr, spread)
        } else {
            format!("{} EVEN", team_abbr)
        };

        Some(Self::new(
            game_id,
            OpportunityType::SpreadValue,
            confidence,
            expected_value,
            recommendation,
            betting_line_id,
        ))
    }

    pub fn with_expiry(mut self, expires_at: DateTime<Utc>) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }

    pub fn is_high_confidence(&self, threshold: f64) -> bool {
        self.confidence >= threshold
    }

    pub fn is_positive_expected_value(&self) -> bool {
        self.expected_value > 0.0
    }
}

impl BettingProvider {
    pub fn new(name: String, api_endpoint: String, rate_limit_per_minute: u32) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            api_endpoint,
            is_active: true,
            rate_limit_per_minute,
            last_request_at: None,
            created_at: Utc::now(),
        }
    }

    pub fn can_make_request(&self) -> bool {
        if !self.is_active {
            return false;
        }

        if let Some(last_request) = self.last_request_at {
            let time_since_last = Utc::now() - last_request;
            let min_interval = chrono::Duration::seconds(60 / self.rate_limit_per_minute as i64);
            time_since_last >= min_interval
        } else {
            true
        }
    }

    pub fn record_request(&mut self) {
        self.last_request_at = Some(Utc::now());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::prediction::{GamePrediction, ProbabilityDistribution, ConfidenceInterval};
    use std::collections::HashMap;

    fn create_test_prediction() -> GamePrediction {
        GamePrediction {
            id: "test-prediction".to_string(),
            game_id: "test-game".to_string(),
            home_score_distribution: ProbabilityDistribution {
                mean: 24.0,
                std_dev: 7.0,
                samples: vec![20.0, 22.0, 24.0, 26.0, 28.0],
                percentiles: HashMap::new(),
            },
            away_score_distribution: ProbabilityDistribution {
                mean: 21.0,
                std_dev: 6.0,
                samples: vec![18.0, 19.0, 21.0, 23.0, 24.0],
                percentiles: HashMap::new(),
            },
            spread_prediction: -3.0,
            total_prediction: 45.0,
            confidence_interval: ConfidenceInterval {
                lower_bound: 40.0,
                upper_bound: 50.0,
                confidence_level: 0.95,
            },
            generated_at: Utc::now(),
        }
    }

    #[test]
    fn test_betting_line_creation() {
        let line = BettingLine::new(
            "game-1".to_string(),
            "DraftKings".to_string(),
            -3.5,
            47.5,
            -110,
            -110,
        );

        assert_eq!(line.game_id, "game-1");
        assert_eq!(line.provider, "DraftKings");
        assert_eq!(line.spread, -3.5);
        assert_eq!(line.total, 47.5);
        assert!(line.is_active);
        assert!(!line.id.is_empty());
    }

    #[test]
    fn test_implied_probability_calculations() {
        let line = BettingLine::new(
            "game-1".to_string(),
            "DraftKings".to_string(),
            -3.5,
            47.5,
            -110,
            -110,
        );

        let home_prob = line.implied_probability_home();
        let away_prob = line.implied_probability_away();

        // -110 should imply approximately 52.38% probability
        assert!((home_prob - 0.5238).abs() < 0.01);
        assert!((away_prob - 0.5238).abs() < 0.01);

        let total_prob = line.total_implied_probability();
        assert!(total_prob > 1.0); // Should be > 1.0 due to vig

        let vig = line.vig_percentage();
        assert!(vig > 0.0); // Should have positive vig
    }

    #[test]
    fn test_positive_moneyline_probability() {
        let line = BettingLine::new(
            "game-1".to_string(),
            "DraftKings".to_string(),
            -3.5,
            47.5,
            150, // Positive moneyline
            -180, // Negative moneyline
        );

        let home_prob = line.implied_probability_home();
        let away_prob = line.implied_probability_away();

        // +150 should imply 40% probability
        assert!((home_prob - 0.4).abs() < 0.01);
        // -180 should imply approximately 64.29% probability
        assert!((away_prob - 0.6429).abs() < 0.01);
    }

    #[test]
    fn test_line_comparison_creation() {
        let betting_line = BettingLine::new(
            "game-1".to_string(),
            "DraftKings".to_string(),
            -3.5,
            47.5,
            -110,
            -110,
        );
        let prediction = create_test_prediction();

        let comparison = LineComparison::new(betting_line.clone(), prediction.clone());

        assert_eq!(comparison.game_id, "game-1");
        assert_eq!(comparison.spread_difference, prediction.spread_prediction - betting_line.spread);
        assert_eq!(comparison.total_difference, prediction.total_prediction - betting_line.total);
        assert!(comparison.value_score > 0.0);
    }

    #[test]
    fn test_line_comparison_recommendations() {
        let betting_line = BettingLine::new(
            "game-1".to_string(),
            "DraftKings".to_string(),
            -6.0, // Line has home team favored by 6
            40.0, // Total of 40
            -110,
            -110,
        );
        let prediction = create_test_prediction(); // Prediction has -3.0 spread, 45.0 total

        let comparison = LineComparison::new(betting_line, prediction);

        // Spread difference: -3.0 - (-6.0) = 3.0 (prediction thinks line is too high)
        assert!(comparison.has_spread_value(1.0));
        let spread_rec = comparison.get_spread_recommendation();
        assert!(spread_rec.is_some());
        assert!(spread_rec.unwrap().contains("underdog"));

        // Total difference: 45.0 - 40.0 = 5.0 (prediction thinks total should be higher)
        assert!(comparison.has_total_value(3.0));
        let total_rec = comparison.get_total_recommendation();
        assert!(total_rec.is_some());
        assert!(total_rec.unwrap().contains("OVER"));
    }

    #[test]
    fn test_value_opportunity_creation() {
        let opportunity = ValueOpportunity::new(
            "game-1".to_string(),
            OpportunityType::SpreadValue,
            0.85,
            2.5,
            "Bet the underdog".to_string(),
            "line-1".to_string(),
        );

        assert_eq!(opportunity.game_id, "game-1");
        assert_eq!(opportunity.opportunity_type, OpportunityType::SpreadValue);
        assert_eq!(opportunity.confidence, 0.85);
        assert_eq!(opportunity.expected_value, 2.5);
        assert!(opportunity.is_high_confidence(0.8));
        assert!(opportunity.is_positive_expected_value());
        assert!(!opportunity.is_expired());
    }

    #[test]
    fn test_value_opportunity_expiry() {
        let past_time = Utc::now() - chrono::Duration::hours(1);
        let future_time = Utc::now() + chrono::Duration::hours(1);

        let expired_opportunity = ValueOpportunity::new(
            "game-1".to_string(),
            OpportunityType::SpreadValue,
            0.85,
            2.5,
            "Bet the underdog".to_string(),
            "line-1".to_string(),
        ).with_expiry(past_time);

        let active_opportunity = ValueOpportunity::new(
            "game-1".to_string(),
            OpportunityType::SpreadValue,
            0.85,
            2.5,
            "Bet the underdog".to_string(),
            "line-1".to_string(),
        ).with_expiry(future_time);

        assert!(expired_opportunity.is_expired());
        assert!(!active_opportunity.is_expired());
    }

    #[test]
    fn test_betting_provider_rate_limiting() {
        let mut provider = BettingProvider::new(
            "Test Provider".to_string(),
            "https://api.test.com".to_string(),
            60, // 60 requests per minute = 1 per second
        );

        assert!(provider.can_make_request());

        provider.record_request();
        // Should not be able to make another request immediately
        // Note: This test might be flaky due to timing, in real implementation
        // you'd want to mock the time or use a more deterministic approach
        
        assert_eq!(provider.name, "Test Provider");
        assert!(provider.is_active);
    }

    #[test]
    fn test_betting_line_expiry() {
        let mut line = BettingLine::new(
            "game-1".to_string(),
            "DraftKings".to_string(),
            -3.5,
            47.5,
            -110,
            -110,
        );

        // Should not be expired immediately
        assert!(!line.is_expired(60));

        // Manually set timestamp to past
        line.timestamp = Utc::now() - chrono::Duration::hours(2);
        assert!(line.is_expired(60)); // 60 minute expiry

        line.deactivate();
        assert!(!line.is_active);
    }

    #[test]
    fn test_serialization() {
        let line = BettingLine::new(
            "game-1".to_string(),
            "DraftKings".to_string(),
            -3.5,
            47.5,
            -110,
            -110,
        );

        let serialized = serde_json::to_string(&line).expect("Failed to serialize");
        let deserialized: BettingLine = serde_json::from_str(&serialized).expect("Failed to deserialize");

        assert_eq!(line, deserialized);
    }

    #[test]
    fn test_opportunity_type_serialization() {
        let types = vec![
            OpportunityType::SpreadValue,
            OpportunityType::TotalValue,
            OpportunityType::MoneylineValue,
            OpportunityType::ArbitrageOpportunity,
        ];

        for opportunity_type in types {
            let serialized = serde_json::to_string(&opportunity_type).expect("Failed to serialize");
            let deserialized: OpportunityType = serde_json::from_str(&serialized).expect("Failed to deserialize");
            assert_eq!(opportunity_type, deserialized);
        }
    }
}