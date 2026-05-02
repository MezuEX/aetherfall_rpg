use crate::entity::effect::Effect;

#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub power: i32,
    pub cost: i32,
    pub effect: Option<Effect>,
}

impl Skill {
    pub fn new(name: &str, power: i32, cost: i32, effect: Option<Effect>) -> Self {
        Skill {
            name: name.to_string(),
            power,
            cost,
            effect,
        }
    }

    /// Apakah skill ini adalah healing (target ally)
    pub fn is_heal(&self) -> bool {
        matches!(self.effect, Some(Effect::Heal(_)))
    }
}
