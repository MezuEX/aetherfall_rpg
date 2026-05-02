use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Role {
    DPS,
    Tank,
    Support,
    Healer,
}

impl Role {
    pub fn name(&self) -> &str {
        match self {
            Role::DPS => "Penyerang",
            Role::Tank => "Tank",
            Role::Support => "Support",
            Role::Healer => "Penyembuh",
        }
    }
}
