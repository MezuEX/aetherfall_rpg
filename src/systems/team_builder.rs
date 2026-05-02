use crate::core::state::GameState;
use crate::data::characters::get_character;
use crate::utils::input::get_usize;
use crate::core::config::MAX_TEAM_SIZE;

pub fn show_team_builder(state: &mut GameState) {
    println!("\n{}═══════════ TEAM BUILDER ═══════════{}", 
        crate::core::config::COLOR_CYAN, crate::core::config::COLOR_RESET);
    println!("Pilih 4 karakter untuk tim aktifmu.");
    println!("Karakter yang sudah di-unlock: {} dari {} tersedia", state.unlocked_characters.len(), 12);
    
    loop {
        println!("\n{}Tim Aktif Saat Ini:{}", crate::core::config::COLOR_GREEN, crate::core::config::COLOR_RESET);
        for (i, name) in state.active_team.iter().enumerate() {
            println!("  {}. {}", i+1, name);
        }
        
        println!("\n{}Karakter Tersedia:{}", crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
        for (i, name) in state.unlocked_characters.iter().enumerate() {
            let in_team = if state.active_team.contains(name) { "✓" } else { " " };
            println!("  {}. [{}] {}", i+1, in_team, name);
        }
        
        println!("\n1. Tambah karakter ke tim");
        println!("2. Hapus karakter dari tim");
        println!("3. Selesai (simpan)");
        println!("0. Kembali ke menu");
        
        let choice = crate::utils::input::get_input("Pilih: ");
        match choice.as_str() {
            "1" => {
                if state.active_team.len() >= MAX_TEAM_SIZE {
                    println!("❌ Tim sudah penuh (maksimal {} karakter)!", MAX_TEAM_SIZE);
                    continue;
                }
                let idx = get_usize("Pilih karakter (nomor): ", state.unlocked_characters.len()) - 1;
                let char_name = state.unlocked_characters[idx].clone();
                if state.active_team.contains(&char_name) {
                    println!("{} sudah ada di tim!", char_name);
                } else {
                    state.active_team.push(char_name);
                    println!("✅ Ditambahkan!");
                }
            }
            "2" => {
                if state.active_team.is_empty() {
                    println!("Tim kosong!");
                    continue;
                }
                let idx = get_usize("Pilih posisi (1-{}): ", state.active_team.len()) - 1;
                let removed = state.active_team.remove(idx);
                println!("✅ {} dihapus dari tim.", removed);
            }
            "3" => {
                if state.active_team.is_empty() {
                    println!("❌ Tim tidak boleh kosong!");
                    continue;
                }
                println!("{}Tim berhasil disimpan!{}", crate::core::config::COLOR_GREEN, crate::core::config::COLOR_RESET);
                break;
            }
            "0" => break,
            _ => println!("Pilihan salah."),
        }
    }
}

pub fn get_active_team_characters(state: &GameState) -> Vec<crate::entity::character::Character> {
    let mut team = vec![];
    for name in &state.active_team {
        if let Some(mut ch) = get_character(name) {
            let (atk_bonus, def_bonus) = state.get_equipment_bonus(name);
            ch.atk += atk_bonus;
            ch.def += def_bonus;
            team.push(ch);
        }
    }
    team
}
