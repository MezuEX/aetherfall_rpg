use crate::entity::character::Character;
use crate::entity::effect::Effect;

pub struct BuffManager;

impl BuffManager {
    pub fn calculate_buffed_atk(character: &Character) -> i32 {
        let mut bonus = 0;
        for (eff, _) in &character.current_effects {
            if let Effect::BuffAtk(b) = eff {
                bonus += *b as i32;
            }
        }
        character.atk + bonus
    }
    
    pub fn calculate_debuffed_def(character: &Character) -> i32 {
        let mut penalty = 0;
        for (eff, _) in &character.current_effects {
            if let Effect::DebuffDef(b) = eff {
                penalty += *b as i32;
            }
        }
        // Defense minimal 0, tidak bisa negatif
        (character.def - penalty).max(0)
    }
}
