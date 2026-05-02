use crate::entity::effect::Effect;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub power: i32,
    pub cost: i32,
    pub effect: Option<Effect>,
    pub description: String,
}

impl Skill {
    pub fn new(name: &str, power: i32, cost: i32, effect: Option<Effect>, description: &str) -> Self {
        Skill {
            name: name.to_string(),
            power,
            cost,
            effect,
            description: description.to_string(),
        }
    }

    pub fn is_heal(&self) -> bool {
        matches!(self.effect, Some(Effect::Heal(_)))
    }
}
