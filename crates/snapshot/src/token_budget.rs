use serde::{Deserialize, Serialize};

/// Snapshot markdown formatter token budget — three discrete presets per
/// arch §Established Decisions [Snapshot Curation Default] + design-system
/// §Decisions Log 2026-05-02. Variant names mirror
/// `ui-bridge::contract::SnapshotPreset` (chunk #38) so the IPC-side
/// preset selector and the algorithmic-substrate budget map by name when
/// `snapshot.generate` IPC wiring lands at a later chunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TokenBudget {
    Conservative,
    #[default]
    Balanced,
    Detailed,
}

impl TokenBudget {
    pub const fn as_token_count(self) -> usize {
        match self {
            Self::Conservative => 10_000,
            Self::Balanced => 25_000,
            Self::Detailed => 50_000,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Conservative => "conservative",
            Self::Balanced => "balanced",
            Self::Detailed => "detailed",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_token_count_returns_locked_constants() {
        assert_eq!(TokenBudget::Conservative.as_token_count(), 10_000);
        assert_eq!(TokenBudget::Balanced.as_token_count(), 25_000);
        assert_eq!(TokenBudget::Detailed.as_token_count(), 50_000);
    }

    #[test]
    fn default_is_balanced() {
        assert_eq!(TokenBudget::default(), TokenBudget::Balanced);
    }

    #[test]
    fn serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&TokenBudget::Conservative).expect("serializes"),
            "\"conservative\""
        );
        assert_eq!(
            serde_json::to_string(&TokenBudget::Balanced).expect("serializes"),
            "\"balanced\""
        );
        assert_eq!(
            serde_json::to_string(&TokenBudget::Detailed).expect("serializes"),
            "\"detailed\""
        );
    }

    #[test]
    fn round_trips_through_serde() {
        for budget in [
            TokenBudget::Conservative,
            TokenBudget::Balanced,
            TokenBudget::Detailed,
        ] {
            let json = serde_json::to_string(&budget).expect("serializes");
            let parsed: TokenBudget = serde_json::from_str(&json).expect("parses");
            assert_eq!(parsed, budget);
        }
    }

    #[test]
    fn label_matches_serde_repr() {
        assert_eq!(TokenBudget::Conservative.label(), "conservative");
        assert_eq!(TokenBudget::Balanced.label(), "balanced");
        assert_eq!(TokenBudget::Detailed.label(), "detailed");
    }
}
