use std::io::{self, Write};

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn get_usize(prompt: &str, max: usize) -> usize {
    loop {
        let input = get_input(prompt);
        if let Ok(num) = input.parse::<usize>() {
            if num >= 1 && num <= max {
                return num;
            }
        }
        println!("❌ Masukkan angka antara 1 dan {}", max);
    }
}

pub fn wait_for_enter() {
    println!("Tekan Enter untuk melanjutkan...");
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();
}
