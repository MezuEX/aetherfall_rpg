use crate::core::state::GameState;
use crate::story::dialogue::{print_dialogue, narrator, show_choice, important_dialogue};
use crate::data::enemies::get_enemy_team_chapter3;
use crate::battle::battle::Battle;
use crate::utils::clear::clear_screen;
use crate::systems::team_builder::get_active_team_characters;
use crate::core::save_manager::save_game;
use std::rc::Rc;
use std::cell::RefCell;

pub fn run_chapter3(state: &mut GameState) -> bool {
    if narrator("══════ CHAPTER 3: FRACTURED TRUTH ══════", state).is_err() { return false; }
    if narrator("Setelah peristiwa di Lumina, tim memasuki reruntuhan Nexus kuno.", state).is_err() { return false; }
    if narrator("Di sinilah rahasia terbesar The Shattering terungkap.", state).is_err() { return false; }
    
    // ORION APPEARS
    if narrator("Sosok misterius muncul dari celah waktu. Jubahnya bercahaya keperakan.", state).is_err() { return false; }
    if print_dialogue("Orion", "Aku Orion, penjaga waktu. Aku datang untuk memberi tahu kebenaran.", state).is_err() { return false; }
    if print_dialogue("Aiden", "Kebenaran apa? Jangan bertele-tele!", state).is_err() { return false; }
    
    if important_dialogue("Orion", "The Shattering bukan bencana alam. Itu adalah... pilihan.", state).is_err() { return false; }
    if narrator("Semua orang terdiam. Udara terasa semakin berat.", state).is_err() { return false; }
    
    if print_dialogue("Orion", "Weaver, kau bukan manusia biasa. Kau adalah wadah Core Element yang tercipta dari sisa-sisa Nexus.", state).is_err() { return false; }
    if print_dialogue("Orion", "Kau adalah satu-satunya yang bisa menyatukan kembali elemen-elemen yang tercerai.", state).is_err() { return false; }
    
    // CHOICE 1: REACTION TO TRUTH
    let choice = match show_choice(state, "Kebenaran ini menghancurkan. Apa reaksimu?", 
        vec![
            "😨 Syok dan tidak percaya",
            "💪 Menerima takdir sebagai Weaver",
            "😠 Marah karena dibohongi selama ini"
        ]) {
        Ok(c) => c,
        Err(_) => return false,
    };
    
    match choice {
        1 => {
            if narrator("Kau mundur selangkah. Hatimu berdebar kencang.", state).is_err() { return false; }
            if important_dialogue("Weaver", "Tidak mungkin... Aku hanya manusia biasa.", state).is_err() { return false; }
        }
        2 => {
            if narrator("Kau menggenggam tanganmu. Energi elemen mulai mengalir di tubuhmu.", state).is_err() { return false; }
            if important_dialogue("Weaver", "Jika ini takdirku, aku akan menjalaninya. Tapi dengan caraku sendiri.", state).is_err() { return false; }
            state.add_affinity("Orion", "Weaver", 15);
        }
        3 => {
            if narrator("Kau mengepalkan tangan. Matahari bersinar lebih terang di sekitarmu.", state).is_err() { return false; }
            if important_dialogue("Weaver", "Kenapa tidak ada yang pernah memberitahuku?!", state).is_err() { return false; }
            state.add_affinity("Orion", "Weaver", -10);
        }
        _ => {}
    }
    
    if narrator("Dari balik reruntuhan, tawa nyaring terdengar.", state).is_err() { return false; }
    if print_dialogue("Nyx", "Hahaha... betapa manisnya pertemuan ini!", state).is_err() { return false; }
    if narrator("Seorang wanita dengan rambut ungu dan mata kuning keluar dari bayang-bayang.", state).is_err() { return false; }
    if important_dialogue("Nyx", "Aku Nyx. Aku benci keteraturan. Aku benci takdir. Aku benci keseimbangan.", state).is_err() { return false; }
    
    // BATTLE AGAINST CHAOS BEAST
    let player_team = get_active_team_characters(state);
    let enemy_team = get_enemy_team_chapter3();
    let state_rc = Rc::new(RefCell::new(state.clone()));
    let mut battle = Battle::new(player_team, enemy_team).with_state(state_rc.clone());
    let win = battle.run();
    
    if let Ok(new_state) = state_rc.try_borrow_mut() {
        *state = new_state.clone();
    }
    
    if !win { return false; }
    
    clear_screen();
    
    if narrator("Setelah pertempuran, Nyx tertawa lagi meskipun kalah.", state).is_err() { return false; }
    if print_dialogue("Nyx", "Kau pikir ini akhir? Tidak. Chaos tidak bisa dikalahkan. Chaos adalah kebebasan.", state).is_err() { return false; }
    
    // CHOICE 2: RESPOND TO NYX
    let choice2 = match show_choice(state, "Nyx percaya chaos adalah kebebasan. Bagaimana responmu?", 
        vec![
            "🌪️ Setuju bahwa terlalu banyak aturan membelenggu",
            "⚖️ Tetap percaya keseimbangan adalah kunci",
            "🤝 Mengajak Nyx untuk bergabung mencari jalan tengah"
        ]) {
        Ok(c) => c,
        Err(_) => return false,
    };
    
    match choice2 {
        1 => {
            if narrator("Kau mengangguk. 'Kadang, kita butuh kekacauan untuk melihat kebenaran.'", state).is_err() { return false; }
            state.add_affinity("Nyx", "Weaver", 15);
        }
        2 => {
            if narrator("Kau menggeleng. 'Tanpa keseimbangan, dunia ini akan hancur.'", state).is_err() { return false; }
            state.add_affinity("Nyx", "Weaver", -5);
        }
        3 => {
            if narrator("Kau mengulurkan tangan. 'Ayo cari jalan bersama. Bukan chaos, bukan orde, tapi harmoni.'", state).is_err() { return false; }
            state.add_affinity("Nyx", "Weaver", 20);
        }
        _ => {}
    }
    
    if choice2 == 3 || state.get_affinity("Nyx", "Weaver") >= 10 {
        if narrator("Nyx menatap tangannya, ragu-ragu.", state).is_err() { return false; }
        if print_dialogue("Nyx", "Tidak pernah ada yang menawarkan itu sebelumnya...", state).is_err() { return false; }
        state.unlocked_characters.push("Nyx".to_string());
        state.add_affinity("Nyx", "Weaver", 10);
    } else {
        if narrator("Nyx tertawa dan menghilang ke dalam bayang-bayang.", state).is_err() { return false; }
        if narrator("Suaranya masih terdengar di kejauhan. 'Kita akan bertemu lagi, Weaver!'", state).is_err() { return false; }
    }
    
    state.unlocked_characters.push("Orion".to_string());
    
    if important_dialogue("Orion", "Waktu hampir habis. Void Sovereign akan segera menyelesaikan rencananya.", state).is_err() { return false; }
    if important_dialogue("Weaver", "Apa yang harus aku lakukan?", state).is_err() { return false; }
    if print_dialogue("Orion", "Kumpulkan semua elemen. Satukan mereka. Hanya itu yang bisa menyelamatkan Aetherfall.", state).is_err() { return false; }
    
    state.current_chapter = 4;
    save_game(state, 1);
    true
}
