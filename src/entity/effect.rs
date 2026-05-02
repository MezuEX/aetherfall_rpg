use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effect {
    Burn(u32),
    Freeze,
    Heal(u32),
    BuffAtk(u32),
    DebuffDef(u32),
}
