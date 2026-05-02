use crate::entity::character::Character;
use crate::entity::effect::Effect;

pub fn apply_all_status(character: &mut Character) {
    let mut to_remove = vec![];
    for (idx, (eff, dur)) in character.current_effects.iter_mut().enumerate() {
        match eff {
            Effect::Burn(dmg) => {
                let damage = (*dmg as i32).min(character.hp);
                character.hp -= damage;
                if character.hp < 0 { character.hp = 0; }
                println!("{}🔥 {} terbakar: -{} HP{}", crate::core::config::COLOR_RED, character.name, damage, crate::core::config::COLOR_RESET);
            }
            Effect::Freeze => {
                println!("{}❄️ {} dalam keadaan beku!{}", crate::core::config::COLOR_CYAN, character.name, crate::core::config::COLOR_RESET);
            }
            Effect::BuffAtk(b) => {
                println!("{}⚡ {} +{} ATK{}", crate::core::config::COLOR_GREEN, character.name, b, crate::core::config::COLOR_RESET);
            }
            Effect::DebuffDef(b) => {
                println!("{}🛡️ {} -{} DEF{}", crate::core::config::COLOR_YELLOW, character.name, b, crate::core::config::COLOR_RESET);
            }
            Effect::Heal(_) => {}
        }
        *dur -= 1;
        if *dur == 0 {
            to_remove.push(idx);
        }
    }
    for idx in to_remove.into_iter().rev() {
        let (eff, _) = character.current_effects.remove(idx);
        match eff {
            Effect::Burn(_) => println!("{}🔥 Efek burn pada {} hilang.{}", crate::core::config::COLOR_GREEN, character.name, crate::core::config::COLOR_RESET),
            Effect::Freeze => println!("{}❄️ {} tidak beku lagi.{}", crate::core::config::COLOR_GREEN, character.name, crate::core::config::COLOR_RESET),
            Effect::BuffAtk(_) => println!("{}⚡ Buff ATK pada {} habis.{}", crate::core::config::COLOR_YELLOW, character.name, crate::core::config::COLOR_RESET),
            Effect::DebuffDef(_) => println!("{}🛡️ Debuff DEF pada {} hilang.{}", crate::core::config::COLOR_YELLOW, character.name, crate::core::config::COLOR_RESET),
            _ => {}
        }
    }

    if character.regen_per_turn > 0 {
        character.heal(character.regen_per_turn);
        println!("{}💚 {} regenerasi +{} HP dari synergy.{}", 
            crate::core::config::COLOR_GREEN, character.name, character.regen_per_turn, crate::core::config::COLOR_RESET);
    }
}
