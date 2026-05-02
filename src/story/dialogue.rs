use crate::core::state::GameState;
use crate::core::game::try_save_game;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::io::{self, Write, Read};
use std::thread;
use std::time::Duration;

pub fn print_dialogue(speaker: &str, text: &str, state: &mut GameState) -> Result<(), bool> {
    println!("\n{}┌─ [{}]{}", crate::core::config::COLOR_CYAN, speaker, crate::core::config::COLOR_RESET);
    print!("{}│{} ", crate::core::config::COLOR_CYAN, crate::core::config::COLOR_RESET);
    
    let skip_flag = Arc::new(AtomicBool::new(false));
    let skip_flag_clone = skip_flag.clone();
    
    // Thread untuk mendeteksi input Enter
    let _handle = thread::spawn(move || {
        let mut buffer = [0u8; 1];
        let _ = io::stdin().read(&mut buffer);
        skip_flag_clone.store(true, Ordering::Relaxed);
    });
    
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if skip_flag.load(Ordering::Relaxed) {
            // Cetak sisa teks langsung
            for j in i..chars.len() {
                print!("{}", chars[j]);
            }
            break;
        }
        print!("{}", chars[i]);
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(25));
        i += 1;
    }
    println!();
    println!("{}└──────────────────────────────────────────────────────────{}", 
        crate::core::config::COLOR_CYAN, crate::core::config::COLOR_RESET);
    
    println!("\n{}⏎ Tekan Enter untuk melanjutkan...{}", 
        crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();
    let input = dummy.trim().to_string();
    if input == ".sv" {
        try_save_game(state);
        return Err(true);
    }
    Ok(())
}

pub fn narrator(text: &str, state: &mut GameState) -> Result<(), bool> {
    println!("\n{}┌─ [Narrator]{}", crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
    print!("{}│{} ", crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
    
    let skip_flag = Arc::new(AtomicBool::new(false));
    let skip_flag_clone = skip_flag.clone();
    
    let _handle = thread::spawn(move || {
        let mut buffer = [0u8; 1];
        let _ = io::stdin().read(&mut buffer);
        skip_flag_clone.store(true, Ordering::Relaxed);
    });
    
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if skip_flag.load(Ordering::Relaxed) {
            for j in i..chars.len() {
                print!("{}", chars[j]);
            }
            break;
        }
        print!("{}", chars[i]);
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(25));
        i += 1;
    }
    println!();
    println!("{}└──────────────────────────────────────────────────────────{}", 
        crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
    
    println!("\n{}⏎ Tekan Enter untuk melanjutkan...{}", 
        crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();
    let input = dummy.trim().to_string();
    if input == ".sv" {
        try_save_game(state);
        return Err(true);
    }
    Ok(())
}

pub fn show_choice(state: &mut GameState, prompt: &str, options: Vec<&str>) -> Result<usize, bool> {
    println!("\n{}╔════════════════════════════════════════════════════════╗{}", 
        crate::core::config::COLOR_MAGENTA, crate::core::config::COLOR_RESET);
    println!("{}║                    🎯 PILIHAN 🎯                       ║{}", 
        crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
    println!("{}╠════════════════════════════════════════════════════════╣{}", 
        crate::core::config::COLOR_MAGENTA, crate::core::config::COLOR_RESET);
    println!("{}║  📌 {}{}", 
        crate::core::config::COLOR_MAGENTA, prompt, crate::core::config::COLOR_RESET);
    println!("{}║                                                      ║{}", 
        crate::core::config::COLOR_MAGENTA, crate::core::config::COLOR_RESET);
    
    for (i, opt) in options.iter().enumerate() {
        let color = match i {
            0 => crate::core::config::COLOR_RED,
            1 => crate::core::config::COLOR_BLUE,
            _ => crate::core::config::COLOR_GREEN,
        };
        println!("{}║    {}. {}{}{}", 
            crate::core::config::COLOR_MAGENTA, i+1, color, opt, crate::core::config::COLOR_RESET);
    }
    println!("{}╚════════════════════════════════════════════════════════╝{}", 
        crate::core::config::COLOR_MAGENTA, crate::core::config::COLOR_RESET);
    
    loop {
        println!("\n{}⏎ Tekan Enter untuk melanjutkan (atau ketik .sv untuk save)...{}", 
            crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim().to_string();
        
        if trimmed == ".sv" {
            try_save_game(state);
            return Err(true);
        }
        
        if let Ok(num) = trimmed.parse::<usize>() {
            if num >= 1 && num <= options.len() {
                return Ok(num);
            }
        }
        println!("{}❌ Pilihan tidak valid. Masukkan angka 1-{} atau .sv{}", 
            crate::core::config::COLOR_RED, options.len(), crate::core::config::COLOR_RESET);
    }
}

pub fn important_dialogue(speaker: &str, text: &str, state: &mut GameState) -> Result<(), bool> {
    println!("\n{}╔════════════════════════════════════════════════════════╗{}", 
        crate::core::config::COLOR_MAGENTA, crate::core::config::COLOR_RESET);
    println!("{}║                    ⚡ PERINGATAN ⚡                     ║{}", 
        crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
    println!("{}╠════════════════════════════════════════════════════════╣{}", 
        crate::core::config::COLOR_MAGENTA, crate::core::config::COLOR_RESET);
    println!("{}║  [{}]{}", 
        crate::core::config::COLOR_MAGENTA, speaker, crate::core::config::COLOR_RESET);
    println!("{}║  📢 {}{}", 
        crate::core::config::COLOR_MAGENTA, text, crate::core::config::COLOR_RESET);
    println!("{}╚════════════════════════════════════════════════════════╝{}", 
        crate::core::config::COLOR_MAGENTA, crate::core::config::COLOR_RESET);
    
    println!("\n{}⏎ Tekan Enter untuk melanjutkan...{}", 
        crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
    let mut dummy = String::new();
    std::io::stdin().read_line(&mut dummy).unwrap();
    let input = dummy.trim().to_string();
    if input == ".sv" {
        try_save_game(state);
        return Err(true);
    }
    Ok(())
}
