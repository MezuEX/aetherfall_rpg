use crate::core::state::GameState;
use crate::story::dialogue::{print_dialogue, narrator, show_choice, important_dialogue};
use crate::data::enemies::get_enemy_team_chapter2;
use crate::battle::battle::Battle;
use crate::utils::clear::clear_screen;
use crate::systems::team_builder::get_active_team_characters;
use crate::core::save_manager::save_game;
use std::rc::Rc;
use std::cell::RefCell;

pub fn run_chapter2(state: &mut GameState) -> bool {
    if narrator("══════ CHAPTER 2: ECHOES OF LIGHT ══════", state).is_err() { return false; }
    if narrator("Tim melanjutkan perjalanan menuju kota suci Lumina. Elara memimpin jalan.", state).is_err() { return false; }
    if narrator("Namun, bayangan masa lalu mulai menghantui setiap langkah mereka.", state).is_err() { return false; }
    
    if print_dialogue("Elara", "Aku merasakan kehadiran kegelapan di sekitar kita.", state).is_err() { return false; }
    if print_dialogue("Aiden", "Aku tidak takut. Biarkan mereka datang.", state).is_err() { return false; }
    if print_dialogue("Lyra", "Aiden, jangan selalu begitu. Terkadang kedamaian lebih baik dari pertempuran.", state).is_err() { return false; }
    
    // AMBUSH!
    if narrator("Tiba-tiba, sosok-sosok hitam muncul dari bayang-bayang.", state).is_err() { return false; }
    if print_dialogue("Mira", "Mereka terlalu banyak! Ini jebakan!", state).is_err() { return false; }
    
    // BATTLE
    let player_team = get_active_team_characters(state);
    let enemy_team = get_enemy_team_chapter2();
    let state_rc = Rc::new(RefCell::new(state.clone()));
    let mut battle = Battle::new(player_team, enemy_team).with_state(state_rc.clone());
    let win = battle.run();
    
    if let Ok(new_state) = state_rc.try_borrow_mut() {
        *state = new_state.clone();
    }
    
    if !win { return false; }
    
    clear_screen();
    
    if narrator("Selesai bertempur, bayangan bersisa mulai memadat membentuk sosok manusia.", state).is_err() { return false; }
    if print_dialogue("Vex", "Kalian... cukup kuat. Tapi tidak cukup.", state).is_err() { return false; }
    if narrator("Sosok itu adalah seorang wanita dengan rambut hitam pekat dan mata merah menyala.", state).is_err() { return false; }
    if important_dialogue("Vex", "Namaku Vex. Aku dulu salah satu dari mereka. Aku tahu kelemahan Void Sovereign.", state).is_err() { return false; }
    
    // CHOICE 1: RESPON TERHADAP VEX
    let choice = match show_choice(state, "Vex menawarkan bantuan. Bagaimana responmu?", 
        vec![
            "🤝 Terima dia sebagai sekutu (percaya)", 
            "❓ Tanya motifnya lebih dulu",
            "⚔️ Tolak dan anggap dia masih musuh"
        ]) {
        Ok(c) => c,
        Err(_) => return false,
    };
    
    match choice {
        1 => {
            if print_dialogue("Weaver", "Aku menerima bantuanmu, Vex. Tapi jika kau khianati kami...", state).is_err() { return false; }
            if narrator("Vex tersenyum tipis. 'Aku tidak punya alasan untuk mengkhianatimu. Kita sama-sama ingin Sovereign jatuh.'", state).is_err() { return false; }
            state.add_affinity("Vex", "Weaver", 20);
        }
        2 => {
            if important_dialogue("Weaver", "Kenapa kau ingin membantu kami? Apa motifmu?", state).is_err() { return false; }
            if narrator("Vex menunduk. Matanya berkaca-kaca. 'Sovereign... membunuh keluargaku. Aku ingin balas dendam.'", state).is_err() { return false; }
            state.add_affinity("Vex", "Weaver", 15);
        }
        3 => {
            if narrator("Kamu menggeleng. 'Aku tidak percaya mantan musuh.',", state).is_err() { return false; }
            if narrator("Vex tertawa getir. 'Kau akan menyesal. Tapi suatu saat kau akan mencari bantuanku.'", state).is_err() { return false; }
            state.add_affinity("Vex", "Weaver", -10);
            state.add_choice("chapter2", "rejected_vex");
        }
        _ => {}
    }
    
    if state.unlocked_characters.contains(&"Elara".to_string()) && choice != 3 {
        state.unlocked_characters.push("Vex".to_string());
        if narrator("Elara mendekati Vex dengan hati-hati.", state).is_err() { return false; }
        if print_dialogue("Elara", "Aku tahu rasanya kehilangan. Tapi dendam tidak akan memulihkan apa pun.", state).is_err() { return false; }
        
        // CHOICE 2: ELARA VS VEX DEBATE
        let choice2 = match show_choice(state, "Elara dan Vex mulai berdebat. Pihak mana yang kau dukung?", 
            vec![
                "✨ Dukung Elara: Harapan adalah satu-satunya jalan",
                "🌑 Dukung Vex: Dendam bisa menjadi kekuatan",
                "🤝 Coba damaikan mereka berdua"
            ]) {
            Ok(c) => c,
            Err(_) => return false,
        };
        
        match choice2 {
            1 => {
                if narrator("Kamu berdiri di samping Elara. 'Dunia ini masih bisa diselamatkan dengan harapan.'", state).is_err() { return false; }
                state.add_affinity("Elara", "Weaver", 15);
                state.add_affinity("Vex", "Weaver", -5);
            }
            2 => {
                if narrator("Kamu mengangguk pada Vex. 'Terkadang, amarah adalah satu-satunya yang tersisa.'", state).is_err() { return false; }
                state.add_affinity("Vex", "Weaver", 15);
                state.add_affinity("Elara", "Weaver", -5);
            }
            3 => {
                if narrator("Kamu berdiri di antara mereka. 'Cukup! Kita tidak akan kemana-mana jika saling bertengkar.'", state).is_err() { return false; }
                state.add_affinity("Elara", "Weaver", 10);
                state.add_affinity("Vex", "Weaver", 10);
                state.add_affinity("Elara", "Vex", 5);
            }
            _ => {}
        }
    } else if choice == 3 {
        if narrator("Vex menghilang ke dalam bayang-bayang, tetapi matanya masih menatapmu tajam.", state).is_err() { return false; }
        if narrator("Kamu merasa keputusan ini mungkin akan berakibat fatal di kemudian hari.", state).is_err() { return false; }
        state.add_choice("chapter2", "vex_left");
    }
    
    if narrator("Di tengah ketegangan, langit tiba-tiba gelap.", state).is_err() { return false; }
    if important_dialogue("Void Sovereign", "Kalian pikir bisa mengalahkanku dengan bersatu? Tidak ada yang bisa menyelamatkan dunia ini.", state).is_err() { return false; }
    if narrator("Suara itu menggema dari segala arah. Kemudian, tanah berguncang.", state).is_err() { return false; }
    
    state.current_chapter = 3;
    save_game(state, 1);
    true
}
