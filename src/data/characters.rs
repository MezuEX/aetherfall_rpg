use crate::entity::character::Character;
use crate::entity::element::Element;
use crate::entity::role::Role;
use crate::entity::skill::Skill;
use crate::entity::effect::Effect;

pub fn get_character(name: &str) -> Option<Character> {
    match name {
        "Aiden" => Some(create_aiden()),
        "Lyra" => Some(create_lyra()),
        "Kael" => Some(create_kael()),
        "Mira" => Some(create_mira()),
        "Elara" => Some(create_elara()),
        "Vex" => Some(create_vex()),
        "Ignis" => Some(create_ignis()),
        "Nereid" => Some(create_nereid()),
        "Zeph" => Some(create_zeph()),
        "Terra" => Some(create_terra()),
        "Solis" => Some(create_solis()),
        "Nyx" => Some(create_nyx()),
        "Orion" => Some(create_orion()),
        _ => None,
    }
}

fn create_aiden() -> Character {
    let skills = vec![
        Skill::new("Tebasan Api", 120, 0, None),
        Skill::new("Ledakan", 150, 0, Some(Effect::Burn(10))),
    ];
    let ultimate = Skill::new("Phoenix Strike", 250, 100, Some(Effect::Burn(20)));
    Character::new("Aiden", Element::Fire, Role::DPS, 120, 35, 15, 80, skills, ultimate, 100)
}

fn create_lyra() -> Character {
    let skills = vec![
        Skill::new("Penyembuhan", 80, 0, Some(Effect::Heal(40))),
        Skill::new("Air Mancur", 100, 0, Some(Effect::Heal(60))),
    ];
    let ultimate = Skill::new("Maha Sembuh", 200, 100, Some(Effect::Heal(150)));
    Character::new("Lyra", Element::Water, Role::Healer, 90, 20, 20, 70, skills, ultimate, 80)
}

fn create_kael() -> Character {
    let skills = vec![
        Skill::new("Tamparan Tanah", 110, 0, None),
        Skill::new("Perisai Batu", 0, 0, Some(Effect::BuffAtk(15))),
    ];
    let ultimate = Skill::new("Gempuran Bumi", 200, 100, None);
    Character::new("Kael", Element::Earth, Role::Tank, 150, 25, 30, 60, skills, ultimate, 120)
}

fn create_mira() -> Character {
    let skills = vec![
        Skill::new("Angin Segar", 90, 0, Some(Effect::Heal(30))),
        Skill::new("Tiupan Kekacauan", 130, 0, Some(Effect::DebuffDef(10))),
    ];
    let ultimate = Skill::new("Badai Pemulih", 180, 100, Some(Effect::Heal(80)));
    Character::new("Mira", Element::Wind, Role::Support, 100, 28, 18, 90, skills, ultimate, 90)
}

fn create_elara() -> Character {
    let skills = vec![
        Skill::new("Sinar Suci", 130, 0, None),
        Skill::new("Pukulan Cahaya", 160, 0, Some(Effect::BuffAtk(10))),
    ];
    let ultimate = Skill::new("Judgment Day", 300, 100, None);
    Character::new("Elara", Element::Light, Role::DPS, 140, 38, 22, 85, skills, ultimate, 110)
}

fn create_vex() -> Character {
    let skills = vec![
        Skill::new("Serangan Bayangan", 140, 0, None),
        Skill::new("Racun Kegelapan", 100, 0, Some(Effect::Burn(15))),
    ];
    let ultimate = Skill::new("Void Slash", 280, 100, Some(Effect::DebuffDef(20)));
    Character::new("Vex", Element::Dark, Role::DPS, 110, 40, 18, 95, skills, ultimate, 90)
}

fn create_ignis() -> Character {
    let skills = vec![
        Skill::new("Berkobar", 130, 0, Some(Effect::Burn(10))),
        Skill::new("Serangan Gila", 180, 0, None),
    ];
    let ultimate = Skill::new("Neraka Mengamuk", 350, 100, Some(Effect::Burn(30)));
    Character::new("Ignis", Element::Fire, Role::DPS, 130, 45, 12, 75, skills, ultimate, 100)
}

fn create_nereid() -> Character {
    let skills = vec![
        Skill::new("Pembersihan", 0, 0, Some(Effect::Heal(50))),
        Skill::new("Lembah Air", 100, 0, Some(Effect::DebuffDef(10))),
    ];
    let ultimate = Skill::new("Maha Pemulih", 250, 100, Some(Effect::Heal(200)));
    Character::new("Nereid", Element::Water, Role::Healer, 95, 18, 22, 72, skills, ultimate, 80)
}

fn create_zeph() -> Character {
    let skills = vec![
        Skill::new("Tusukan Angin", 120, 0, None),
        Skill::new("Lompatan Angin", 80, 0, Some(Effect::BuffAtk(15))),
    ];
    let ultimate = Skill::new("Badai Penghancur", 280, 100, None);
    Character::new("Zeph", Element::Wind, Role::DPS, 105, 36, 16, 110, skills, ultimate, 85)
}

fn create_terra() -> Character {
    let skills = vec![
        Skill::new("Pukulan Bumi", 110, 0, None),
        Skill::new("Perisai Karang", 0, 0, Some(Effect::BuffAtk(10))),
    ];
    let ultimate = Skill::new("Gempa Besar", 220, 100, None);
    Character::new("Terra", Element::Earth, Role::Tank, 160, 28, 35, 55, skills, ultimate, 130)
}

fn create_solis() -> Character {
    let skills = vec![
        Skill::new("Berkah Cahaya", 0, 0, Some(Effect::Heal(70))),
        Skill::new("Radiance", 120, 0, None),
    ];
    let ultimate = Skill::new("Penyucian", 200, 100, Some(Effect::Heal(150)));
    Character::new("Solis", Element::Light, Role::Support, 100, 25, 20, 78, skills, ultimate, 90)
}

fn create_nyx() -> Character {
    let skills = vec![
        Skill::new("Kegelapan Menyiksa", 150, 0, Some(Effect::DebuffDef(15))),
        Skill::new("Mimpi Buruk", 100, 0, Some(Effect::Burn(12))),
    ];
    let ultimate = Skill::new("Kiamat", 320, 100, Some(Effect::Burn(25)));
    Character::new("Nyx", Element::Dark, Role::DPS, 115, 42, 14, 88, skills, ultimate, 95)
}

fn create_orion() -> Character {
    let skills = vec![
        Skill::new("Zaman Purba", 130, 0, None),
        Skill::new("Futur", 170, 0, Some(Effect::BuffAtk(10))),
    ];
    let ultimate = Skill::new("Singularitas", 300, 100, Some(Effect::DebuffDef(25)));
    Character::new("Orion", Element::Light, Role::Support, 125, 35, 20, 92, skills, ultimate, 100)
}
