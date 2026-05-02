use crate::utils::input::wait_for_enter;
use crate::utils::clear::clear_screen;

pub fn show_ending(ending_id: &str) {
    clear_screen();
    println!("\n{}═══════════════════════════════════════════════════════════{}", 
        crate::core::config::COLOR_MAGENTA, crate::core::config::COLOR_RESET);
    println!("{}                        EPILOGUE                        {}", 
        crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
    println!("{}═══════════════════════════════════════════════════════════{}\n", 
        crate::core::config::COLOR_MAGENTA, crate::core::config::COLOR_RESET);
    
    match ending_id {
        "harmony" => {
            println!("{}✨ ENDING: HARMONY ✨{}", 
                crate::core::config::COLOR_CYAN, crate::core::config::COLOR_RESET);
            println!();
            println!("Dunia Aetherfall kembali stabil. Elemen-elemen bersatu dalam harmoni.");
            println!("Keseimbangan tercapai, meskipun rapuh seperti kristal.");
            println!();
            println!("Para karakter melanjutkan hidup mereka masing-masing:");
            println!("  • Aiden menjadi panglima perra, menjaga kedamaian kerajaan.");
            println!("  • Lyra membuka rumah sakit umum, menyembuhkan siapa pun tanpa memandang status.");
            println!("  • Kael menjadi penjaga gerbang Nexus, memastikan tidak ada yang menyalahgunakan elemen.");
            println!("  • Mira berkeliling dunia sebagai kurir kebebasan, menyebarkan kabar dan harapan.");
            println!("  • Elara mendirikan ordo baru untuk melindungi yang lemah.");
            println!("  • Vex, meskipun masih sinis, mulai percaya bahwa kebaikan itu nyata.");
            println!("  • Orion dan Nyx menjadi penjaga waktu, memastikan sejarah tidak terulang.");
            println!();
            println!("Dan kau, Weaver, menjadi legenda yang dikenang sepanjang masa.");
            println!("Kisahmu diceritakan dari generasi ke generasi.");
            println!("Tapi di dalam hati kecilmu, kau tahu bahwa harmoni ini tidak akan bertahan selamanya.");
            println!("Suatu hari, konflik akan muncul lagi. Dan saat itu tiba...");
            println!();
            println!("{}'... Mungkin akan ada Weaver berikutnya.'{}", 
                crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
        }
        "void" => {
            println!("{}🌑 ENDING: THE GREAT VOID 🌑{}", 
                crate::core::config::COLOR_RED, crate::core::config::COLOR_RESET);
            println!();
            println!("Semua elemen lenyap. Warna memudar dari dunia.");
            println!("Langit menjadi abu-abu. Tanah menjadi hitam pekat.");
            println!("Tidak ada api, tidak ada air, tidak ada angin, tidak ada tanah.");
            println!("Tidak ada cahaya, tidak ada gelap. Hanya... kekosongan.");
            println!();
            println!("Para karakter menghilang satu per satu.");
            println!("Mereka tidak mati. Mereka... menjadi bagian dari kekosongan.");
            println!("Aiden: 'Aku tidak merasakan amarah lagi... Aku tidak merasakan apa-apa.'");
            println!("Lyra: 'Tidak ada air mata yang bisa jatuh di sini.'");
            println!("Kael: 'Tanah... tidak ada lagi tanah.'");
            println!("Mira: 'Angin... diam.'");
            println!("Elara: 'Cahaya... padam.'");
            println!("Vex: 'Kegelapan... juga mati.'");
            println!();
            println!("Kau berdiri sendiri di tengah kekosongan.");
            println!("Tidak ada suara. Tidak ada gerakan. Hanya kau dan keabadian.");
            println!();
            println!("{}'Apa ini yang kuinginkan? Damai tanpa konflik?'{}", 
                crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
            println!("{}'Atau... aku baru saja membunuh dunia?'{}", 
                crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
            println!();
            println!("Tidak ada jawaban. Hanya keheningan abadi.");
        }
        "chaos" => {
            println!("{}🌪️ ENDING: CHAOS UNBOUND 🌪️{}", 
                crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
            println!();
            println!("Elemen tetap liar. Dunia menjadi tempat yang tak terduga.");
            println!("Api bisa meledak kapan saja. Air bisa berubah menjadi badai.");
            println!("Tanah berguncang tanpa sebab. Angin membawa bisuhan misterius.");
            println!("Cahaya bisa menyilaukan. Gelap bisa menelan seluruh kota.");
            println!();
            println!("Tapi di tengah kekacauan, kehidupan berkembang pesat.");
            println!("Makhluk-makhluk baru lahir dari sisa-sisa elemen.");
            println!("Para karakter menjadi penjelajah, mencari petualangan di setiap sudut dunia.");
            println!();
            println!("Aiden: 'Hidup tidak pernah membosankan!'");
            println!("Lyra: 'Setiap hari adalah kejutan.'");
            println!("Kael: 'Aku tidak bisa memprediksi apa pun lagi... dan itu menyenangkan.'");
            println!("Mira: 'Angin membawa cerita baru setiap pagi.'");
            println!("Elara: 'Kita tidak bisa menyelamatkan semua orang... tapi kita bisa mencoba.'");
            println!("Vex: 'Akhirnya, dunia yang jujur tentang kekejamannya.'");
            println!();
            println!("Kau, Weaver, menjadi simbol kebebasan.");
            println!("Ada yang mengagumimu. Ada yang membencimu.");
            println!("Tapi satu hal yang pasti: petualangan tidak akan pernah berakhir.");
            println!();
            println!("{}'Dunia ini gila... dan aku mencintainya.'{}", 
                crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
        }
        "secret" => {
            println!("{}🌟 SECRET ENDING: THE ETERNAL WEAVER 🌟{}", 
                crate::core::config::COLOR_GREEN, crate::core::config::COLOR_RESET);
            println!();
            println!("Kau mengorbankan dirimu. Tubuhmu berubah menjadi cahaya.");
            println!("Elemen-elemen mengalir ke dalam dirimu, dan kau menjadi Core Element yang baru.");
            println!();
            println!("Dunia stabil. Warna kembali. Kehidupan pulih.");
            println!("Tapi kau tidak lagi memiliki wujud manusia.");
            println!("Kau adalah sungai yang mengalir. Kau adalah api yang menghangatkan.");
            println!("Kau adalah angin yang membawa benih. Kau adalah tanah tempat orang berpijak.");
            println!("Kau adalah cahaya fajar. Kau adalah kegelapan yang melindungi mimpi.");
            println!();
            println!("Para karakter tidak menangis. Mereka tersenyum.");
            println!("Aiden: 'Dia ada di setiap api yang kusulut.'");
            println!("Lyra: 'Aku merasakannya di setiap tetes air yang menyembuhkan.'");
            println!("Kael: 'Dia adalah kekuatan di bawah kakiku.'");
            println!("Mira: 'Setiap hembusan angin membawa pesannya.'");
            println!("Elara: 'Cahaya ini... adalah senyumnya.'");
            println!("Vex: 'Bahkan dalam gelap, dia menjagaku.'");
            println!("Orion: 'Dia menjadi bagian dari waktu itu sendiri.'");
            println!("Nyx: 'Chaos... dan keteraturan... bersatu dalam dirinya.'");
            println!();
            println!("Mereka menjadi penjaga dunia, meneruskan misimu.");
            println!("Dan setiap malam, ketika bintang-bintang bersinar, mereka tahu...");
            println!("Kau ada di sana. Menjaga mereka. Selamanya.");
            println!();
            println!("{}'Terima kasih, Weaver. Untuk semuanya.'{}", 
                crate::core::config::COLOR_CYAN, crate::core::config::COLOR_RESET);
        }
        _ => {
            println!("{}❓ ENDING: UNKNOWN ❓{}", 
                crate::core::config::COLOR_RED, crate::core::config::COLOR_RESET);
            println!();
            println!("Dunia terus berputar tanpa arah yang jelas.");
            println!("Tidak ada yang tahu apa yang terjadi pada para karakter.");
            println!("Beberapa cerita mengatakan mereka masih berjuang.");
            println!("Yang lain mengatakan mereka telah menyerah.");
            println!();
            println!("Mungkin... jawabannya tergantung pada siapa yang bertanya.");
            println!();
            println!("{}'Cerita belum berakhir. Mungkin... belum dimulai.'{}", 
                crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
        }
    }
    
    println!("\n{}═══════════════════════════════════════════════════════════{}", 
        crate::core::config::COLOR_MAGENTA, crate::core::config::COLOR_RESET);
    println!("{}                TERIMA KASIH TELAH BERMAIN                {}", 
        crate::core::config::COLOR_GREEN, crate::core::config::COLOR_RESET);
    println!("{}═══════════════════════════════════════════════════════════{}", 
        crate::core::config::COLOR_MAGENTA, crate::core::config::COLOR_RESET);
    
    wait_for_enter();
}
