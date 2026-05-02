use crate::entity::character::Character;
use crate::entity::element::Element;
use crate::entity::role::Role;
use crate::entity::skill::Skill;
use crate::entity::effect::Effect;

pub fn get_enemy_team_chapter1() -> Vec<Character> {
    let skills = vec![Skill::new("Serangan Gelap", 100, 0, None)];
    let ultimate = Skill::new("Null Void", 150, 100, None);
    let void_knight = Character::new("Void Knight", Element::Dark, Role::DPS, 130, 32, 20, 75, skills.clone(), ultimate.clone(), 100);
    let shade = Character::new("Shade", Element::Dark, Role::DPS, 70, 28, 12, 85, skills.clone(), ultimate.clone(), 60);
    let wraith = Character::new("Wraith", Element::Dark, Role::Support, 100, 25, 15, 80, skills.clone(), ultimate.clone(), 80);
    let null_mage = Character::new("Null Mage", Element::Dark, Role::Healer, 110, 22, 18, 65, skills, ultimate, 90);
    vec![void_knight, shade, wraith, null_mage]
}

pub fn get_enemy_team_chapter2() -> Vec<Character> {
    let skills = vec![Skill::new("Tebasan Bayangan", 120, 0, None)];
    let ultimate = Skill::new("Void Storm", 180, 100, None);
    let shadow_guard = Character::new("Shadow Guard", Element::Dark, Role::Tank, 150, 28, 25, 70, skills.clone(), ultimate.clone(), 120);
    let dark_priest = Character::new("Dark Priest", Element::Dark, Role::Healer, 120, 20, 15, 80, skills, ultimate, 90);
    vec![shadow_guard, dark_priest]
}

pub fn get_enemy_team_chapter3() -> Vec<Character> {
    let skills = vec![Skill::new("Pukulan Gila", 140, 0, None)];
    let ultimate = Skill::new("Rage", 220, 100, None);
    let chaos_beast = Character::new("Chaos Beast", Element::Fire, Role::DPS, 200, 45, 20, 90, skills, ultimate, 150);
    vec![chaos_beast]
}

pub fn get_enemy_team_chapter4() -> Vec<Character> {
    let skills = vec![Skill::new("Frost Blast", 130, 0, Some(Effect::Freeze))];
    let ultimate = Skill::new("Glacier", 200, 100, Some(Effect::Freeze));
    let ancient_wyrm = Character::new("Ancient Wyrm", Element::Water, Role::DPS, 250, 50, 30, 85, skills, ultimate, 180);
    vec![ancient_wyrm]
}

pub fn get_final_boss() -> Character {
    let skills = vec![
        Skill::new("Null Wave", 160, 0, Some(Effect::DebuffDef(20))),
        Skill::new("Void Erosion", 120, 0, Some(Effect::Burn(15))),
    ];
    let ultimate = Skill::new("Absolute Zero", 400, 100, None);
    Character::new("Null Sovereign", Element::Dark, Role::DPS, 500, 60, 40, 100, skills, ultimate, 250)
}
