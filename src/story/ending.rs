pub fn show_ending(ending_id: &str) {
    println!("\n\x1b[35m========== ENDING ==========\x1b[0m");
    match ending_id {
        "harmony" => {
            println!("Anda memilih harmoni. Elemen-elemen bersatu kembali.");
            println!("Dunia Aetherfall hidup dalam keseimbangan yang rapuh, namun indah.");
            println!("Para pahlawan dikenang sebagai penyeluruh dunia.");
        }
        "void" => {
            println!("Anda memilih kehampaan. Semua elemen lenyap.");
            println!("Dunia menjadi sunyi, tanpa warna, tanpa konflik.");
            println!("Kesunyian abadi menyelimuti segalanya.");
        }
        "chaos" => {
            println!("Anda membiarkan kekacauan. Elemen liar berkeliaran.");
            println!("Dunia menjadi tempat berbahaya namun penuh kejutan.");
            println!("Petualangan tak akan pernah usai.");
        }
        _ => {
            println!("Dunia terus berputar tanpa arah yang jelas...");
        }
    }
    println!("\x1b[35m============================\x1b[0m");
}
