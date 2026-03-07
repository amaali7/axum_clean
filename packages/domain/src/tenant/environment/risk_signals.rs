#[derive(Debug, Clone)]
pub enum AuthenticationStrength {
    PasswordOnly,
    MultiFactor,
    HardwareKey,
}

#[derive(Debug, Clone)]
pub struct RiskSignals {
    score: u8,
    authentication_strength: AuthenticationStrength,
    recent_transaction_count: u32,
}

impl RiskSignals {
    pub fn new(
        score: u8,
        authentication_strength: AuthenticationStrength,
        recent_transaction_count: u32,
    ) -> Self {
        Self {
            score,
            authentication_strength,
            recent_transaction_count,
        }
    }

    pub fn score(&self) -> u8 {
        self.score
    }

    pub fn authentication_strength(&self) -> AuthenticationStrength {
        self.authentication_strength.clone()
    }

    pub fn recent_transaction_count(&self) -> u32 {
        self.recent_transaction_count
    }
}
