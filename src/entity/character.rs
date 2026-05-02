use crate::entity::element::Element;
use crate::entity::role::Role;
use crate::entity::skill::Skill;
use crate::entity::effect::Effect;
use crate::core::config::{ENERGY_PER_TURN, ULTIMATE_ENERGY_COST};
use crate::systems::buff_system::BuffManager;
use crate::ui::battle_ui::{damage_animation, heal_animation, status_effect_animation};
use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub element: Element,
    pub role: Role,
    pub hp: i32,
    pub max_hp: i32,
    pub atk: i32,
    pub def: i32,
    pub speed: i32,
    pub skills: Vec<Skill>,
    pub ultimate: Skill,
    pub current_energy: i32,
    pub current_effects: Vec<(Effect, u32)>,
    pub toughness: i32,
    pub max_toughness: i32,
    pub regen_per_turn: i32,
    pub is_defending: bool,
}

impl Character {
    pub fn new(name: &str, element: Element, role: Role, hp: i32, atk: i32, def: i32, speed: i32,
               skills: Vec<Skill>, ultimate: Skill, toughness: i32) -> Self {
        Character {
            name: name.to_string(),
            element,
            role,
            hp,
            max_hp: hp,
            atk,
            def,
            speed,
            skills,
            ultimate,
            current_energy: 0,
            current_effects: vec![],
            toughness,
            max_toughness: toughness,
            regen_per_turn: 0,
            is_defending: false,
        }
    }

    pub fn is_alive(&self) -> bool { self.hp > 0 }

    pub fn is_frozen(&self) -> bool {
        self.current_effects.iter().any(|(e, _)| matches!(e, Effect::Freeze))
    }

    pub fn take_damage(&mut self, mut damage: i32) -> i32 {
        if self.is_defending {
            damage = damage / 2;
            self.is_defending = false;
            println!("{}🛡️ {} bertahan, damage berkurang 50%!{}", 
                crate::core::config::COLOR_CYAN, self.name, crate::core::config::COLOR_RESET);
        }
        let actual = damage.max(0).min(self.hp);
        self.hp -= actual;
        if self.hp < 0 { self.hp = 0; }
        actual
    }

    pub fn heal(&mut self, amount: i32) {
        let healed = amount.min(self.max_hp - self.hp);
        self.hp += healed;
        heal_animation(&self.name, healed);
    }

    pub fn get_effective_atk(&self) -> i32 {
        BuffManager::calculate_buffed_atk(self)
    }

    pub fn get_effective_def(&self) -> i32 {
        BuffManager::calculate_debuffed_def(self)
    }

    pub fn basic_attack(&mut self, target: &mut Character, rng: &mut impl Rng) {
        let element_mult = self.element.advantage(target.element);
        let is_crit = rng.gen_bool(0.1);
        let atk = self.get_effective_atk();
        let def = target.get_effective_def();
        let base_damage = (atk - def).max(5);
        let damage = ((base_damage as f32) * element_mult * if is_crit { 1.5 } else { 1.0 }) as i32;
        let final_damage = damage.max(1).min(target.hp);
        
        damage_animation(&target.name, final_damage, is_crit);
        target.take_damage(final_damage);
        
        self.current_energy += ENERGY_PER_TURN;
        if self.current_energy > ULTIMATE_ENERGY_COST {
            self.current_energy = ULTIMATE_ENERGY_COST;
        }

        if element_mult > 1.0 {
            target.toughness -= final_damage / 5;
            if target.toughness <= 0 {
                println!("\n{}💥 BREAK! {} terpental dan kehilangan giliran!{}", 
                    crate::core::config::COLOR_YELLOW, target.name, crate::core::config::COLOR_RESET);
                target.toughness = target.max_toughness;
                target.current_effects.push((Effect::Freeze, 1));
            }
        }
    }

    pub fn use_skill(&mut self, skill: &Skill, target: &mut Character, _rng: &mut impl Rng) {
        println!("\n{}✨ {} menggunakan {}! ✨{}", 
            crate::core::config::COLOR_MAGENTA, self.name, skill.name, crate::core::config::COLOR_RESET);
        
        match &skill.effect {
            Some(Effect::Heal(amt)) => {
                let heal = (*amt as i32) + skill.power;
                target.heal(heal);
            }
            Some(Effect::Burn(dmg)) => {
                let burn_dmg = (*dmg as i32) + skill.power;
                let scaled = (target.max_hp as f32 * 0.08).max(5.0) as i32 + skill.power / 5;
                let final_dmg = burn_dmg.min(scaled).min(target.hp);
                damage_animation(&target.name, final_dmg, false);
                target.take_damage(final_dmg);
                target.current_effects.push((Effect::Burn(final_dmg as u32), 2));
                status_effect_animation(&target.name, "BURN", crate::core::config::COLOR_RED);
            }
            Some(Effect::Freeze) => {
                let dmg = skill.power.min(target.hp);
                damage_animation(&target.name, dmg, false);
                target.take_damage(dmg);
                target.current_effects.push((Effect::Freeze, 1));
                status_effect_animation(&target.name, "FREEZE", crate::core::config::COLOR_CYAN);
            }
            Some(Effect::BuffAtk(bonus)) => {
                let bonus_val = (*bonus as i32) + skill.power;
                self.current_effects.push((Effect::BuffAtk(bonus_val as u32), 3));
                println!("{}⚡ ATK {} meningkat +{}!{}", 
                    crate::core::config::COLOR_GREEN, self.name, bonus_val, crate::core::config::COLOR_RESET);
            }
            Some(Effect::DebuffDef(red)) => {
                let red_val = (*red as i32) + skill.power;
                target.current_effects.push((Effect::DebuffDef(red_val as u32), 3));
                println!("{}🛡️ DEF {} menurun -{}!{}", 
                    crate::core::config::COLOR_YELLOW, target.name, red_val, crate::core::config::COLOR_RESET);
            }
            None => {
                let element_mult = self.element.advantage(target.element);
                let atk = self.get_effective_atk();
                let def = target.get_effective_def();
                let base_damage = (atk - def).max(5);
                let damage = ((base_damage as f32) * element_mult * (skill.power as f32 / 100.0)) as i32;
                let final_damage = damage.max(1).min(target.hp);
                damage_animation(&target.name, final_damage, false);
                target.take_damage(final_damage);
            }
        }
        
        self.current_energy -= skill.cost;
        if self.current_energy < 0 {
            self.current_energy = 0;
        }
        self.current_energy += ENERGY_PER_TURN / 2;
        if self.current_energy > ULTIMATE_ENERGY_COST {
            self.current_energy = ULTIMATE_ENERGY_COST;
        }
    }

    pub fn use_ultimate(&mut self, target: &mut Character, rng: &mut impl Rng) {
        println!("\n{}🌟 ULTIMATE: {} 🌟{}", 
            crate::core::config::COLOR_YELLOW, self.ultimate.name, crate::core::config::COLOR_RESET);
        let ultimate_clone = self.ultimate.clone();
        self.use_skill(&ultimate_clone, target, rng);
    }

    pub fn tick_status_effects(&mut self) {
        use crate::systems::status_system::apply_all_status;
        apply_all_status(self);
    }
}
