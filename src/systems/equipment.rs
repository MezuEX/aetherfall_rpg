use crate::core::state::{GameState, Equipment, EquipmentSlot};
use crate::utils::input::get_usize;
use rand::Rng;

pub fn show_equipment_menu(state: &mut GameState) {
    println!("\n{}═══════════ EQUIPMENT ═══════════{}", 
        crate::core::config::COLOR_CYAN, crate::core::config::COLOR_RESET);
    
    let characters: Vec<String> = state.active_team.clone();
    if characters.is_empty() {
        println!("Tidak ada karakter di tim aktif.");
        return;
    }
    
    println!("Karakter dalam tim:");
    for (i, name) in characters.iter().enumerate() {
        println!("  {}. {}", i+1, name);
    }
    let char_idx = get_usize("Pilih karakter: ", characters.len()) - 1;
    let char_name = &characters[char_idx];
    
    let equips = state.equipment.entry(char_name.to_string()).or_insert_with(Vec::new);
    println!("\nEquipment {} saat ini:", char_name);
    if equips.is_empty() {
        println!("  (kosong)");
    }
    for e in equips.iter() {
        println!("  {:?}: {} (+{} ATK, +{} DEF) - {}", e.slot, e.name, e.atk_bonus, e.def_bonus, e.description);
    }
    
    println!("\n1. Pasang equipment baru");
    println!("2. Lepas equipment");
    println!("0. Kembali");
    let choice = crate::utils::input::get_input("Pilih: ");
    match choice.as_str() {
        "1" => {
            let dummy_equips = vec![
                Equipment { name: "Pedang Api".to_string(), slot: EquipmentSlot::Weapon, atk_bonus: 15, def_bonus: 0, description: "Meningkatkan damage serangan api.".to_string() },
                Equipment { name: "Perisai Batu".to_string(), slot: EquipmentSlot::Armor, atk_bonus: 0, def_bonus: 20, description: "Meningkatkan pertahanan fisik.".to_string() },
                Equipment { name: "Jimat Cahaya".to_string(), slot: EquipmentSlot::Artifact, atk_bonus: 10, def_bonus: 10, description: "Meningkatkan semua atribut.".to_string() },
                Equipment { name: "Kapak Naga".to_string(), slot: EquipmentSlot::Weapon, atk_bonus: 25, def_bonus: 0, description: "Senjata berat dengan damage tinggi.".to_string() },
                Equipment { name: "Jubah Angin".to_string(), slot: EquipmentSlot::Armor, atk_bonus: 5, def_bonus: 10, description: "Meningkatkan kecepatan.".to_string() },
            ];
            for (i, e) in dummy_equips.iter().enumerate() {
                println!("  {}. {} ({:?}) +{} ATK +{} DEF", i+1, e.name, e.slot, e.atk_bonus, e.def_bonus);
            }
            let equip_idx = get_usize("Pilih equipment: ", dummy_equips.len()) - 1;
            let new_equip = dummy_equips[equip_idx].clone();
            state.equip_item(char_name, new_equip);
        }
        "2" => {
            if equips.is_empty() {
                println!("Tidak ada equipment yang terpasang.");
                return;
            }
            for (i, e) in equips.iter().enumerate() {
                println!("  {}. {} ({:?})", i+1, e.name, e.slot);
            }
            let remove_idx = get_usize("Pilih equipment yang akan dilepas: ", equips.len()) - 1;
            let removed = equips.remove(remove_idx);
            println!("✅ {} dilepas dari {}.", removed.name, char_name);
        }
        _ => {}
    }
}

pub fn random_equipment_drop(rng: &mut impl Rng) -> Option<Equipment> {
    if rng.gen_bool(0.25) {
        let weapons = vec![
            Equipment { name: "Pedang Naga".to_string(), slot: EquipmentSlot::Weapon, atk_bonus: 25, def_bonus: 5, description: "Senjata legendaris dari naga kuno.".to_string() },
            Equipment { name: "Jubah Angin".to_string(), slot: EquipmentSlot::Armor, atk_bonus: 5, def_bonus: 15, description: "Meningkatkan kecepatan dan kelincahan.".to_string() },
            Equipment { name: "Orb Petir".to_string(), slot: EquipmentSlot::Artifact, atk_bonus: 20, def_bonus: 0, description: "Memberikan damage listrik pada serangan.".to_string() },
            Equipment { name: "Perisai Void".to_string(), slot: EquipmentSlot::Armor, atk_bonus: 0, def_bonus: 30, description: "Perisai gelap yang sangat kuat.".to_string() },
            Equipment { name: "Mahkota Solar".to_string(), slot: EquipmentSlot::Artifact, atk_bonus: 15, def_bonus: 15, description: "Meningkatkan semua atribut secara signifikan.".to_string() },
        ];
        Some(weapons[rng.gen_range(0..weapons.len())].clone())
    } else {
        None
    }
}
