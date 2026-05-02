use crate::core::state::GameState;
use crate::core::config::{SAVE_PREFIX, MAX_SAVE_SLOTS};
use crate::utils::input::get_usize;
use crate::battle::battle::BattleSaveData;
use std::fs;
use std::path::Path;

pub fn get_save_filename(slot: usize) -> String {
    format!("{}{}.json", SAVE_PREFIX, slot)
}

pub fn get_battle_save_filename(slot: usize) -> String {
    format!("{}_battle_{}.json", SAVE_PREFIX, slot)
}

pub fn save_game(state: &GameState, slot: usize) -> bool {
    let filename = get_save_filename(slot);
    if let Ok(json) = serde_json::to_string_pretty(state) {
        if fs::write(&filename, json).is_ok() {
            println!("\n{}💾 Game tersimpan pada slot {} (chapter {}){}", 
                crate::core::config::COLOR_GREEN, slot, state.current_chapter, crate::core::config::COLOR_RESET);
            return true;
        }
    }
    println!("{}❌ Gagal menyimpan game!{}", crate::core::config::COLOR_RED, crate::core::config::COLOR_RESET);
    false
}

pub fn save_battle_state(slot: usize, battle_data: &BattleSaveData) -> bool {
    let filename = get_battle_save_filename(slot);
    if let Ok(json) = serde_json::to_string_pretty(battle_data) {
        if fs::write(&filename, json).is_ok() {
            println!("{}⚔️ Battle state tersimpan di slot {}{}", 
                crate::core::config::COLOR_GREEN, slot, crate::core::config::COLOR_RESET);
            return true;
        }
    }
    false
}

pub fn load_game(slot: usize) -> Option<GameState> {
    let filename = get_save_filename(slot);
    if let Ok(data) = fs::read_to_string(&filename) {
        if let Ok(state) = serde_json::from_str::<GameState>(&data) {
            println!("{}✅ Load berhasil dari slot {} (chapter {}){}", 
                crate::core::config::COLOR_GREEN, slot, state.current_chapter, crate::core::config::COLOR_RESET);
            return Some(state);
        }
    }
    println!("{}❌ Tidak ada save file di slot {}{}", crate::core::config::COLOR_RED, slot, crate::core::config::COLOR_RESET);
    None
}

pub fn has_battle_save(slot: usize) -> bool {
    let filename = get_battle_save_filename(slot);
    Path::new(&filename).exists()
}

pub fn get_available_slots() -> Vec<usize> {
    let mut slots = Vec::new();
    for i in 1..=MAX_SAVE_SLOTS {
        let filename = get_save_filename(i);
        if Path::new(&filename).exists() {
            slots.push(i);
        }
    }
    slots
}

pub fn get_empty_slots() -> Vec<usize> {
    let mut slots = Vec::new();
    for i in 1..=MAX_SAVE_SLOTS {
        let filename = get_save_filename(i);
        if !Path::new(&filename).exists() {
            slots.push(i);
        }
    }
    slots
}

pub fn get_save_info(slot: usize) -> Option<(u32, String, bool)> {
    let filename = get_save_filename(slot);
    if let Ok(data) = fs::read_to_string(&filename) {
        if let Ok(state) = serde_json::from_str::<GameState>(&data) {
            let has_battle = has_battle_save(slot);
            return Some((state.current_chapter, state.active_team.join(", "), has_battle));
        }
    }
    None
}

pub fn delete_save(slot: usize) -> bool {
    let filename = get_save_filename(slot);
    let battle_filename = get_battle_save_filename(slot);
    let mut success = true;
    
    if Path::new(&filename).exists() {
        if fs::remove_file(&filename).is_err() {
            success = false;
        }
    }
    if Path::new(&battle_filename).exists() {
        let _ = fs::remove_file(&battle_filename);
    }
    
    if success {
        println!("{}✅ Save slot {} berhasil dihapus!{}", crate::core::config::COLOR_GREEN, slot, crate::core::config::COLOR_RESET);
    } else {
        println!("{}❌ Gagal menghapus save slot {}{}", crate::core::config::COLOR_RED, slot, crate::core::config::COLOR_RESET);
    }
    success
}

pub fn choose_save_slot() -> Option<usize> {
    let empty_slots = get_empty_slots();
    let available_slots = get_available_slots();
    
    println!("\n{}═══════════ PILIH SLOT SAVE ═══════════{}", 
        crate::core::config::COLOR_CYAN, crate::core::config::COLOR_RESET);
    
    if !available_slots.is_empty() {
        println!("\n{}Slot yang sudah ada save:{}", crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
        for &slot in &available_slots {
            if let Some((chapter, team, has_battle)) = get_save_info(slot) {
                let icon = if has_battle { "⚔️" } else { "📖" };
                println!("  {}. {} Slot {} - Chapter {}, Tim: {}", slot, icon, slot, chapter, team);
            }
        }
    }
    
    if !empty_slots.is_empty() {
        println!("\n{}Slot kosong:{}", crate::core::config::COLOR_GREEN, crate::core::config::COLOR_RESET);
        for &slot in &empty_slots {
            println!("  {}. Slot {} (kosong)", slot, slot);
        }
    }
    
    if available_slots.len() >= MAX_SAVE_SLOTS {
        println!("\n{}⚠️  Peringatan: Save slot sudah penuh! (maksimal {}){}", 
            crate::core::config::COLOR_RED, MAX_SAVE_SLOTS, crate::core::config::COLOR_RESET);
        println!("Apakah Anda ingin menghapus salah satu save yang ada?");
        println!("1. Ya, hapus salah satu slot");
        println!("2. Tidak, batal save");
        
        let choice = crate::utils::input::get_input("Pilih [1-2]: ");
        if choice == "1" {
            return choose_delete_slot();
        } else {
            return None;
        }
    }
    
    println!("\nPilih slot [1-{}]:", MAX_SAVE_SLOTS);
    let slot = get_usize("Slot: ", MAX_SAVE_SLOTS);
    Some(slot)
}

pub fn choose_delete_slot() -> Option<usize> {
    let available_slots = get_available_slots();
    if available_slots.is_empty() {
        println!("Tidak ada save file untuk dihapus.");
        return None;
    }
    
    println!("\n{}═══════════ HAPUS SAVE SLOT ═══════════{}", 
        crate::core::config::COLOR_RED, crate::core::config::COLOR_RESET);
    
    for &slot in &available_slots {
        if let Some((chapter, team, _)) = get_save_info(slot) {
            println!("  {}. Slot {} - Chapter {}, Tim: {}", slot, slot, chapter, team);
        }
    }
    println!("  0. Batal");
    
    let choice = get_usize("Pilih slot yang akan dihapus [0-{}]: ", MAX_SAVE_SLOTS);
    if choice == 0 {
        return None;
    }
    
    if delete_save(choice) {
        Some(choice)
    } else {
        None
    }
}

pub fn choose_load_slot() -> Option<usize> {
    let available_slots = get_available_slots();
    if available_slots.is_empty() {
        println!("Tidak ada save file ditemukan.");
        return None;
    }
    
    println!("\n{}═══════════ LOAD GAME ═══════════{}", 
        crate::core::config::COLOR_CYAN, crate::core::config::COLOR_RESET);
    
    for &slot in &available_slots {
        if let Some((chapter, team, has_battle)) = get_save_info(slot) {
            let icon = if has_battle { "⚔️ (lanjut battle)" } else { "📖 (lanjut cerita)" };
            println!("  {}. {} Slot {} - Chapter {}, Tim: {}", slot, icon, slot, chapter, team);
        }
    }
    println!("  0. Kembali");
    
    let choice = get_usize("Pilih slot [0-{}]: ", MAX_SAVE_SLOTS);
    if choice == 0 {
        None
    } else if available_slots.contains(&choice) {
        Some(choice)
    } else {
        println!("Slot {} tidak memiliki save file.", choice);
        None
    }
}
