use crate::core::state::GameState;
use crate::story::dialogue::{print_dialogue, narrator, show_choice, important_dialogue};
use crate::data::enemies::get_enemy_team_chapter4;
use crate::battle::battle::Battle;
use crate::utils::clear::clear_screen;
use crate::systems::team_builder::get_active_team_characters;
use crate::core::save_manager::save_game;
use std::rc::Rc;
use std::cell::RefCell;

pub fn run_chapter4(state: &mut GameState) -> bool {
    if narrator("══════ CHAPTER 4: COLLAPSE ══════", state).is_err() { return false; }
    if narrator("Dunia mulai runtuh. Langit terbelah. Tanah retak.", state).is_err() { return false; }
    if narrator("Void Sovereign telah memulai ritual penghancuran elemen.", state).is_err() { return false; }
    
    if print_dialogue("Aiden", "Kita tidak punya banyak waktu! Kita harus segera ke Nexus!", state).is_err() { return false; }
    if print_dialogue("Lyra", "Tapi... jalannya sangat berbahaya. Banyak dari kita mungkin tidak akan selamat.", state).is_err() { return false; }
    
    // CONFLICT IN TEAM
    if narrator("Para karakter mulai berselisih. Ketegangan memuncak.", state).is_err() { return false; }
    if print_dialogue("Elara", "Kita bisa selamat jika kita percaya satu sama lain!", state).is_err() { return false; }
    if print_dialogue("Vex", "Percaya? Kata-kata kosong. Hanya kekuatan yang bisa menyelamatkan kita.", state).is_err() { return false; }
    
    // CHOICE 1: MEDIATE CONFLICT
    let choice = match show_choice(state, "Elara dan Vex bertengkar lagi. Bagaimana kau memediasi?", 
        vec![
            "✨ Pihak Elara: Harapan dan persatuan",
            "🌑 Pihak Vex: Kekuatan dan realita",
            "💔 Mengaku bahwa kau sendiri tidak tahu jawabannya"
        ]) {
        Ok(c) => c,
        Err(_) => return false,
    };
    
    match choice {
        1 => {
            if narrator("Kamu berdiri di samping Elara. 'Tanpa harapan, kita sudah mati sejak awal.'", state).is_err() { return false; }
            state.add_affinity("Elara", "Weaver", 15);
            state.add_affinity("Vex", "Weaver", -10);
        }
        2 => {
            if narrator("Kau menghela napas. 'Vex benar. Dunia ini kejam. Tapi itu bukan alasan untuk menyerah pada kemanusiaan.'", state).is_err() { return false; }
            state.add_affinity("Vex", "Weaver", 15);
            state.add_affinity("Elara", "Weaver", -5);
        }
        3 => {
            if narrator("Kamu menunduk. 'Aku tidak tahu jawabannya... Aku sama bingungnya seperti kalian.'", state).is_err() { return false; }
            if narrator("Semua terdiam. Untuk pertama kalinya, mereka melihat kerentanan dalam dirimu.", state).is_err() { return false; }
            state.add_affinity("Elara", "Weaver", 5);
            state.add_affinity("Vex", "Weaver", 5);
        }
        _ => {}
    }
    
    if narrator("Saat mereka berdebat, bumi berguncang lebih keras.", state).is_err() { return false; }
    if important_dialogue("Void Sovereign", "Cukup! Aku akan mengakhiri ini sekarang.", state).is_err() { return false; }
    
    // BATTLE AGAINST ANCIENT WYRM
    let player_team = get_active_team_characters(state);
    let enemy_team = get_enemy_team_chapter4();
    let state_rc = Rc::new(RefCell::new(state.clone()));
    let mut battle = Battle::new(player_team, enemy_team).with_state(state_rc.clone());
    let win = battle.run();
    
    if let Ok(new_state) = state_rc.try_borrow_mut() {
        *state = new_state.clone();
    }
    
    if !win { return false; }
    
    clear_screen();
    
    if narrator("Wyrm jatuh. Tapi di kejauhan, pintu menuju Void Sovereign terbuka.", state).is_err() { return false; }
    
    // CHOICE 2: FINAL PREPARATION
    let choice2 = match show_choice(state, "Pintu menuju Sovereign terbuka. Apa yang akan kau lakukan?", 
        vec![
            "🏃 Langsung masuk dan hadapi Sovereign sekarang",
            "📝 Rencanakan strategi bersama tim",
            "🙏 Berdoa dan meminta kekuatan dari elemen"
        ]) {
        Ok(c) => c,
        Err(_) => return false,
    };
    
    match choice2 {
        1 => {
            if narrator("Kau berlari tanpa ragu. 'Tidak ada waktu! Kita harus menghentikannya sekarang!'", state).is_err() { return false; }
            state.add_choice("chapter4", "rush");
        }
        2 => {
            if narrator("Kau memanggil tim. 'Kita akan menang jika kita bekerja sama. Ini rencananya...'", state).is_err() { return false; }
            state.add_choice("chapter4", "plan");
            // Bonus synergy untuk battle final
            state.global_flags.insert("got_plan".to_string(), true);
        }
        3 => {
            if narrator("Kau menutup mata dan merasakan energi elemen di sekitarmu.", state).is_err() { return false; }
            if narrator("Cahaya dari berbagai warna mulai menyelimuti tubuhmu dan tim.", state).is_err() { return false; }
            state.add_choice("chapter4", "prayer");
            // Bonus untuk battle final
            state.global_flags.insert("got_blessing".to_string(), true);
        }
        _ => {}
    }
    
    if important_dialogue("Weaver", "Tidak peduli apa yang terjadi di dalam sana... terima kasih untuk segalanya.", state).is_err() { return false; }
    
    if narrator("Mata para karakter berkaca-kaca. Mereka mengangguk sebagai satu kesatuan.", state).is_err() { return false; }
    if important_dialogue("Aiden", "Kita akan menang. Atau mati bersama. Tidak ada yang mundur.", state).is_err() { return false; }
    
    state.current_chapter = 5;
    save_game(state, 1);
    true
}
