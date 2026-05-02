use crate::core::state::GameState;
use crate::story::dialogue::{print_dialogue, narrator, important_dialogue};
use crate::data::enemies::get_final_boss;
use crate::battle::battle::Battle;
use crate::utils::input::get_input;
use crate::systems::team_builder::get_active_team_characters;
use crate::utils::clear::clear_screen;
use std::rc::Rc;
use std::cell::RefCell;

pub fn run_final_battle(state: &mut GameState) -> String {
    if narrator("══════ FINAL BATTLE: THE NULL SOVEREIGN ══════", state).is_err() { return "default".to_string(); }
    if narrator("Kau memasuki ruang hampa. Void Sovereign berdiri di tengah, dikelilingi energi kegelapan.", state).is_err() { return "default".to_string(); }
    
    if important_dialogue("Null Sovereign", "Kau datang. Seperti yang sudah ku duga, Weaver.", state).is_err() { return "default".to_string(); }
    if print_dialogue("Null Sovereign", "Tapi kau tidak mengerti. Aku tidak menghancurkan dunia. Aku ingin menyelamatkannya.", state).is_err() { return "default".to_string(); }
    if print_dialogue("Weaver", "Dengan menghapus semua elemen? Itu bukan menyelamatkan!", state).is_err() { return "default".to_string(); }
    
    if important_dialogue("Null Sovereign", "Elemen adalah sumber konflik. Selama elemen ada, peperangan tidak akan pernah berhenti.", state).is_err() { return "default".to_string(); }
    if print_dialogue("Null Sovereign", "Aku menciptakan The Shattering untuk memisahkan elemen. Tapi dunia malah semakin kacau.", state).is_err() { return "default".to_string(); }
    if important_dialogue("Null Sovereign", "Maka, satu-satunya jalan adalah... menghapus semuanya. Mengembalikan dunia ke kekosongan.", state).is_err() { return "default".to_string(); }
    
    if important_dialogue("Weaver", "Apa yang akan kau lakukan jika aku menolak?", state).is_err() { return "default".to_string(); }
    if narrator("Sovereign mengangkat tangannya. Energi hitam berkumpul di telapak tangannya.", state).is_err() { return "default".to_string(); }
    if important_dialogue("Null Sovereign", "Maka kau akan mati bersamaku. Tidak ada yang tersisa.", state).is_err() { return "default".to_string(); }
    
    let total_affinity: i32 = state.affinity.values().flat_map(|m| m.values()).sum();
    let has_secret = total_affinity >= 200;
    
    // Clone nilai boolean untuk menghindari borrow issue
    let has_plan = state.global_flags.get("got_plan").unwrap_or(&false).clone();
    let has_blessing = state.global_flags.get("got_blessing").unwrap_or(&false).clone();
    
    if has_plan {
        if narrator("Berkat rencanamu, tim bergerak lebih terkoordinasi.", state).is_err() { return "default".to_string(); }
    }
    if has_blessing {
        if narrator("Cahaya elemen menyelimuti tim. Mereka merasa lebih kuat.", state).is_err() { return "default".to_string(); }
    }
    
    let player_team = get_active_team_characters(state);
    let boss_team = vec![get_final_boss()];
    let state_rc = Rc::new(RefCell::new(state.clone()));
    let mut battle = Battle::new(player_team, boss_team).with_state(state_rc.clone());
    let win = battle.run();
    
    if let Ok(new_state) = state_rc.try_borrow_mut() {
        *state = new_state.clone();
    }
    
    if win {
        clear_screen();
        
        if narrator("Sovereign jatuh berlutut. Energi hitamnya mulai memudar.", state).is_err() { return "default".to_string(); }
        if important_dialogue("Null Sovereign", "Kau... berhasil. Mungkin... kau benar. Mungkin aku yang terlalu takut pada konflik.", state).is_err() { return "default".to_string(); }
        if narrator("Sovereign mulai menghilang seperti debu.", state).is_err() { return "default".to_string(); }
        if print_dialogue("Null Sovereign", "Sekarang... dunia ini di tanganmu, Weaver. Jangan buat kesalahan yang sama sepertiku.", state).is_err() { return "default".to_string(); }
        
        if narrator("Dengan lenyapnya Sovereign, elemen-elemen mulai bergetar. Mereka menunggu keputusanmu.", state).is_err() { return "default".to_string(); }
        
        println!("\n{}══════════════ PILIHAN TERAKHIR ══════════════{}", 
            crate::core::config::COLOR_MAGENTA, crate::core::config::COLOR_RESET);
        println!("Dunia ada di tanganmu. Apa yang akan kau lakukan dengan elemen-elemen ini?");
        println!();
        println!("1. {}Satukan semua elemen kembali{}", crate::core::config::COLOR_CYAN, crate::core::config::COLOR_RESET);
        println!("   → Dunia akan stabil lagi. Tapi kelemahan manusia tetap ada.");
        println!("   → Konflik akan muncul lagi suatu hari nanti.");
        println!();
        println!("2. {}Hapus semua elemen{}", crate::core::config::COLOR_RED, crate::core::config::COLOR_RESET);
        println!("   → Dunia jadi kosong. Tidak ada konflik.");
        println!("   → Tapi juga tidak ada warna. Tidak ada kehidupan yang berarti.");
        println!();
        println!("3. {}Biarkan chaos berlangsung{}", crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
        println!("   → Elemen tetap liar. Dunia penuh petualangan.");
        println!("   → Bahaya selalu mengintai, tapi kehidupan terus berkembang.");
        
        if has_secret {
            println!();
            println!("4. {}🌟 KORBANKAN DIRI SEBAGAI CORE ELEMENT BARU 🌟{}", crate::core::config::COLOR_GREEN, crate::core::config::COLOR_RESET);
            println!("   → (Requirement: Affinity semua karakter total >= 200)");
            println!("   → Dunia stabil tanpa mengorbankan siapa pun.");
            println!("   → Tapi kau akan menjadi bagian dari dunia selamanya.");
        }
        
        println!();
        let choice = get_input("Pilih 1/2/3");
        
        match choice.as_str() {
            "1" => {
                if narrator("Kau memutuskan untuk menyatukan elemen.", state).is_err() { return "default".to_string(); }
                if narrator("Cahaya keemasan menyelimuti dunia. Elemen-elemen mulai bersatu kembali.", state).is_err() { return "default".to_string(); }
                if print_dialogue("Weaver", "Ini bukan akhir. Ini awal yang baru.", state).is_err() { return "default".to_string(); }
                "harmony".to_string()
            }
            "2" => {
                if narrator("Kau mengangkat tangan dan melepaskan semua elemen.", state).is_err() { return "default".to_string(); }
                if narrator("Pelan-pelan, warna mulai memudar dari dunia. Hanya putih dan hitam yang tersisa.", state).is_err() { return "default".to_string(); }
                if print_dialogue("Weaver", "Tidak ada yang bisa terluka lagi... tidak ada konflik lagi.", state).is_err() { return "default".to_string(); }
                "void".to_string()
            }
            "3" => {
                if narrator("Kau melepas kendali. Elemen-elemen bebas berkeliaran.", state).is_err() { return "default".to_string(); }
                if narrator("Dunia menjadi liar, penuh kejutan. Bahaya dan keindahan berbaur menjadi satu.", state).is_err() { return "default".to_string(); }
                if print_dialogue("Weaver", "Biarkan kehidupan mengalir. Aku tidak akan menghentikannya.", state).is_err() { return "default".to_string(); }
                "chaos".to_string()
            }
            "4" if has_secret => {
                if narrator("Kau menutup mata dan memusatkan energimu.", state).is_err() { return "default".to_string(); }
                if important_dialogue("Weaver", "Jika ini yang terbaik untuk semua... Aku rela.", state).is_err() { return "default".to_string(); }
                if narrator("Tubuhmu mulai berubah menjadi cahaya. Elemen-elemen mengalir ke dalam dirimu.", state).is_err() { return "default".to_string(); }
                if narrator("Kau menjadi Core Element yang baru. Dunia stabil. Dan karakter-karaktermu menjadi penjaga dunia.", state).is_err() { return "default".to_string(); }
                if print_dialogue("Lyra", "Weaver... tidak... Kau tidak seharusnya...", state).is_err() { return "default".to_string(); }
                if important_dialogue("Weaver", "Jangan sedih. Aku akan selalu bersama kalian. Dalam angin, dalam air, dalam api, dalam tanah, dalam cahaya, dan dalam gelap.", state).is_err() { return "default".to_string(); }
                "secret".to_string()
            }
            _ => {
                if narrator("Dunia terus berputar tanpa keputusan yang jelas...", state).is_err() { return "default".to_string(); }
                "default".to_string()
            }
        }
    } else {
        if narrator("Kamu kalah. Sovereign menghilang bersama dunia.", state).is_err() { return "default".to_string(); }
        "default".to_string()
    }
}
