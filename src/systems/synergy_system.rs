use crate::entity::character::Character;
use std::collections::HashMap;

pub fn apply_synergy(team: &mut [Character]) {
    let mut elem_count = HashMap::new();
    let mut role_count = HashMap::new();
    for c in team.iter() {
        *elem_count.entry(c.element).or_insert(0) += 1;
        *role_count.entry(c.role.name().to_string()).or_insert(0) += 1;
    }
    // Bonus elemen: jika 2 elemen sama -> +10 ATK
    for (elem, count) in elem_count {
        if count >= 2 {
            let bonus = 10;
            for c in team.iter_mut() {
                if c.element == elem {
                    c.atk += bonus;
                    println!("Synergy: +{} ATK untuk semua {} karena 2x elemen sama.", bonus, elem.name());
                }
            }
        }
    }
    // Bonus role: jika 2 support -> regen 5 HP tiap turn
    if let Some(&count) = role_count.get("Support") {
        if count >= 2 {
            for c in team.iter_mut() {
                c.regen_per_turn += 5;
            }
            println!("Synergy Support: semua karakter regen 5 HP per turn.");
        }
    }
    // Bonus 2 healer -> regen tambahan
    if let Some(&count) = role_count.get("Penyembuh") {
        if count >= 2 {
            for c in team.iter_mut() {
                c.regen_per_turn += 3;
            }
            println!("Synergy Healer: regen +3 HP per turn.");
        }
    }
}
