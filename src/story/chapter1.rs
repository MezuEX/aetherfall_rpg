use crate::core::state::GameState;
use crate::story::dialogue::{print_dialogue, narrator, show_choice, important_dialogue};
use crate::data::enemies::get_enemy_team_chapter1;
use crate::battle::battle::Battle;
use crate::systems::team_builder::get_active_team_characters;
use crate::core::save_manager::save_game;
use crate::utils::clear::clear_screen;
use crate::utils::input::wait_for_enter;
use std::rc::Rc;
use std::cell::RefCell;

pub fn run_chapter1(state: &mut GameState) -> bool {
    if narrator("═══ CHAPTER 1: SHATTERED AWAKENING ═══", state).is_err() { return false; }
    if narrator("Langit di atas Aetherfall terbakar merah. The Shattering telah menghancurkan Nexus.", state).is_err() { return false; }
    if narrator("Enam elemen yang dulu harmonis kini saling bertarung. Dunia berada di ambang kehancuran.", state).is_err() { return false; }
    
    if print_dialogue("Aiden", "Kita harus melindungi desa! Void Knight sudah di depan mata!", state).is_err() { return false; }
    if print_dialogue("Lyra", "Tapi kita kalah jumlah... Aku tidak bisa menyembuhkan semua orang sekaligus.", state).is_err() { return false; }
    if print_dialogue("Lyra", "Aiden... jangan gegabah. Aku tidak mau kehilanganmu lagi.", state).is_err() { return false; }
    if print_dialogue("Kael", "Aku akan menahan serangan mereka. Kalian serang dari belakang.", state).is_err() { return false; }
    if print_dialogue("Kael", "Tanah ini adalah rumahku. Aku tidak akan mundur.", state).is_err() { return false; }
    if print_dialogue("Mira", "Angin membawa kabar buruk... musuh punya perangkap. Tapi...", state).is_err() { return false; }
    if print_dialogue("Mira", "Aku juga mendengar bisikan harapan. Mungkin kita bisa mengubah takdir.", state).is_err() { return false; }
    
    let choice = match show_choice(state, "Sebagai Weaver, bagaimana strategimu menghadapi Void Knight?", 
        vec![
            "🔥 Serangan frontal penuh amarah! (Aiden setuju)", 
            "🌊 Strategi pertahanan dan penyembuhan (Lyra setuju)", 
            "🪨 Formasi bertahan yang kokoh (Kael setuju)",
            "🍃 Gerilya dan serangan mendadak (Mira setuju)"
        ]) {
        Ok(c) => c,
        Err(_) => return false,
    };
    
    match choice {
        1 => {
            if narrator("Kamu memerintahkan serangan frontal. Aiden memimpin dengan amarah membara.", state).is_err() { return false; }
            if print_dialogue("Aiden", "AKU TIDAK AKAN MENGAMPUNI MEREKA!", state).is_err() { return false; }
            if narrator("Amarah Aiden membangkitkan kekuatan api yang dahsyat. Musuh mundur ketakutan.", state).is_err() { return false; }
            state.add_affinity("Aiden", "Weaver", 15);
            state.add_choice("chapter1", "fury");
        }
        2 => {
            if narrator("Kamu memilih strategi hati-hati. Lyra menyembuhkan para pejuang yang terluka.", state).is_err() { return false; }
            if print_dialogue("Lyra", "Air kehidupan, pulihkan mereka!", state).is_err() { return false; }
            if narrator("Cahaya biru menyelimuti tim. Mereka bertahan lebih lama dari perkiraan.", state).is_err() { return false; }
            state.add_affinity("Lyra", "Weaver", 15);
            state.add_choice("chapter1", "healing");
        }
        3 => {
            if narrator("Kamu memilih formasi bertahan. Kael menjadi perisai yang tak tertembus.", state).is_err() { return false; }
            if print_dialogue("Kael", "Tidak ada yang melewati batas ini!", state).is_err() { return false; }
            if narrator("Perisai batu Kael menahan setiap serangan musuh.", state).is_err() { return false; }
            state.add_affinity("Kael", "Weaver", 15);
            state.add_choice("chapter1", "defense");
        }
        4 => {
            if narrator("Kamu memilih taktik gerilya. Mira memanfaatkan kecepatan angin.", state).is_err() { return false; }
            if print_dialogue("Mira", "Terlalu lambat! Awas dari belakang!", state).is_err() { return false; }
            if narrator("Mira melesat seperti badai, membingungkan barisan musuh.", state).is_err() { return false; }
            state.add_affinity("Mira", "Weaver", 15);
            state.add_choice("chapter1", "guerrilla");
        }
        _ => {}
    }
    
    let player_team = get_active_team_characters(state);
    let enemy_team = get_enemy_team_chapter1();
    let state_rc = Rc::new(RefCell::new(state.clone()));
    let mut battle = Battle::new(player_team, enemy_team).with_state(state_rc.clone());
    let win = battle.run();
    
    if let Ok(new_state) = state_rc.try_borrow_mut() {
        *state = new_state.clone();
    }
    
    if !win { return false; }
    
    clear_screen();
    
    if narrator("Pertempuran berakhir. Void Knight tumbang, tapi Aiden terluka parah.", state).is_err() { return false; }
    if print_dialogue("Aiden", "Aku... aku baik-baik saja. Hanya goresan kecil.", state).is_err() { return false; }
    if print_dialogue("Lyra", "Jangan berbohong, Aiden! Darahmu...", state).is_err() { return false; }
    if narrator("Lyra berlari ke arah Aiden, air matanya jatuh perlahan.", state).is_err() { return false; }
    if important_dialogue("Lyra", "Jangan pernah pergi... jangan tinggalkan aku.", state).is_err() { return false; }
    
    let choice2 = match show_choice(state, "Lyra sangat mengkhawatirkan Aiden. Apa yang akan kau lakukan?", 
        vec![
            "💖 Menenangkan Lyra dan berjanji tidak akan ada yang mati",
            "⚔️ Mendorong Aiden untuk bangkit dan terus berjuang",
            "🤝 Menawarkan bantuan untuk menyembuhkan Aiden bersama Lyra"
        ]) {
        Ok(c) => c,
        Err(_) => return false,
    };
    
    match choice2 {
        1 => {
            if narrator("Kamu mendekati Lyra dan memegang tangannya dengan lembut.", state).is_err() { return false; }
            if important_dialogue("Weaver", "Tidak ada yang akan mati hari ini. Aku berjanji.", state).is_err() { return false; }
            if narrator("Lyra menatapmu dengan mata berkaca-kaca. Dia mengangguk pelan.", state).is_err() { return false; }
            state.add_affinity("Lyra", "Weaver", 20);
            state.add_affinity("Aiden", "Weaver", 10);
        }
        2 => {
            if narrator("Kamu menepuk bahu Aiden dan berbicara dengan tegas.", state).is_err() { return false; }
            if important_dialogue("Weaver", "Kita belum selesai, Aiden. Dunia masih butuh kita.", state).is_err() { return false; }
            if narrator("Aiden tersenyum tipis dan berdiri perlahan, meskipun tubuhnya menggigil.", state).is_err() { return false; }
            state.add_affinity("Aiden", "Weaver", 20);
        }
        3 => {
            if narrator("Kamu menghampiri mereka dan memfokuskan energimu.", state).is_err() { return false; }
            if important_dialogue("Weaver", "Ayo kita lakukan bersama. Tenang, fokus.", state).is_err() { return false; }
            if narrator("Cahaya keemasan menyelimuti Aiden. Lukanya perlahan pulih.", state).is_err() { return false; }
            state.add_affinity("Lyra", "Weaver", 15);
            state.add_affinity("Aiden", "Weaver", 15);
        }
        _ => {}
    }
    
    if narrator("Saat fajar menyingsing, sosok bercahaya muncul dari timur.", state).is_err() { return false; }
    if print_dialogue("Elara", "Aku Elara, Knight of Dawn. Aku datang untuk membantu.", state).is_err() { return false; }
    if print_dialogue("Elara", "Tapi... kau bukan manusia biasa, bukan? Aku merasakan kehadiran Core Element.", state).is_err() { return false; }
    
    let choice3 = match show_choice(state, "Elara mengenali kekuatanmu. Bagaimana responmu?", 
        vec![
            "🤝 Jujur tentang siapa dirimu sebagai Weaver",
            "❓ Meminta Elara menjelaskan lebih dulu tentang Core Element",
            "🛡️ Tidak mempercayainya dan memintanya pergi"
        ]) {
        Ok(c) => c,
        Err(_) => return false,
    };
    
    match choice3 {
        1 => {
            if narrator("Kamu mengambil napas dalam-dalam dan berbicara jujur.", state).is_err() { return false; }
            if important_dialogue("Weaver", "Aku Weaver. Aku bisa menyatukan elemen-elemen yang tercerai-berai.", state).is_err() { return false; }
            if print_dialogue("Elara", "Aku tahu... Aku sudah mencarimu sejak lama.", state).is_err() { return false; }
            state.unlocked_characters.push("Elara".to_string());
            state.add_affinity("Elara", "Weaver", 20);
        }
        2 => {
            if narrator("Kamu menatap Elara dengan waspada namun ingin tahu.", state).is_err() { return false; }
            if important_dialogue("Weaver", "Jelaskan apa yang kau ketahui tentang Core Element.", state).is_err() { return false; }
            if narrator("Elara mulai bercerita tentang Nexus dan ramalan kuno tentang Weaver.", state).is_err() { return false; }
            state.unlocked_characters.push("Elara".to_string());
            state.add_affinity("Elara", "Weaver", 15);
        }
        3 => {
            if narrator("Kamu menggelengkan kepala. Kecurigaan masih membelenggu hatimu.", state).is_err() { return false; }
            if important_dialogue("Weaver", "Aku tidak butuh bantuanmu. Pergilah.", state).is_err() { return false; }
            if narrator("Elara tersenyum sedih. 'Kau akan berubah pikiran, Weaver. Aku akan menunggu.'", state).is_err() { return false; }
            state.unlocked_characters.push("Elara".to_string());
            state.add_affinity("Elara", "Weaver", 0);
            state.add_choice("chapter1", "rejected_elara");
        }
        _ => {}
    }
    
    if narrator("Di kejauhan, siluet hitam muncul di atas bukit. Void Sovereign mengamati dari jauh.", state).is_err() { return false; }
    if important_dialogue("Void Sovereign", "Weaver... akhirnya kita bertemu. Tapi kau belum siap.", state).is_err() { return false; }
    if narrator("Suara itu bergema di kepala semua orang. Kemudian sosok itu menghilang.", state).is_err() { return false; }
    
    state.current_chapter = 2;
    save_game(state, 1);
    wait_for_enter();
    true
}
