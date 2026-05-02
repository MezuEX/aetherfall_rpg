# 🧙‍♂️ AETHERFALL RPG — Turn-Based Terminal Epic Fantasy

> *“Keseimbangan adalah ilusi. Kekacauan datang dari keberagaman.”*  
> — *The Null Sovereign*

**Aetherfall RPG** adalah game *turn-based role-playing game* yang berjalan sepenuhnya di terminal, ditulis dalam bahasa **Rust**.  
Kamu berperan sebagai **Weaver**, satu-satunya makhluk yang bisa menyatukan energi elemen yang tercerai-berai setelah peristiwa **The Shattering**.  
Kumpulkan 12 karakter unik, bina hubungan (affinity), eksplorasi 4 chapter cerita, dan pilih nasib dunia di antara 3 ending yang berbeda.

---

## 📖 Sinopsis Cerita

### 🌌 Dunia: Aetherfall

Dahulu kala, dunia Aetherfall stabil karena keberadaan **Core Element Nexus** — sebuah sumber energi murni yang menyeimbangkan enam elemen: Api, Air, Angin, Tanah, Cahaya, dan Gelap.  
Namun, suatu hari **The Shattering** terjadi. Nexus hancur, elemen-elemen menjadi liar, dan dunia perlahan menuju keruntuhan.  
Di tengah kekacauan, muncullah **The Null Sovereign** — entitas yang ingin menghapus semua elemen dan mengembalikan dunia ke keadaan kosong (null).  

### 🧠 Peranmu: The Weaver

Kamu adalah **Weaver**, manusia langka yang bisa:
- Menyatukan energi karakter yang merupakan *fragmen emosi dunia*
- Mengendalikan resonansi antar elemen
- Menyelamatkan — atau menghancurkan — sisa-sisa Aetherfall

### 🎭 Perjalanan Cerita (4 Chapter + Final)

| Chapter | Judul             | Event Utama |
|---------|-------------------|-------------|
| 1       | Shattered Awakening | Rekrut Aiden, Lyra, Kael, Mira. Kalahkan Void Knight. |
| 2       | Echoes of Light     | Elara (Cahaya) dan Vex (Gelap) bergabung. Konflik idealisme vs realita. |
| 3       | Fractured Truth     | Orion (pengendali waktu) dan Nyx (kegilaan) mengungkap bahwa Weaver adalah wadah Core Element. |
| 4       | Collapse            | Dunia hancur. Karakter saling berkonflik. Puncaknya The Null Sovereign muncul. |
| Final   | The Null Sovereign  | Pertarungan terakhir. Pilihan mengubah dunia. |

### 🎬 Ending

1. **Harmony** → Satukan elemen. Dunia stabil tapi rapuh. Karakter yang berkonflik berdamai.
2. **Void** → Hapus semua elemen. Damai abadi, namun hampa dan sepi.
3. **Chaos** → Biarkan kekacauan. Dunia liar, petualangan tak pernah usai.
4. **Secret Ending** (syarat: affinity semua karakter maksimal) → Weaver mengorbankan diri, menjadi Core Element baru. Dunia stabil tanpa reset.

---

## 👥 Karakter

### Roster Awal (Chapter 1)

| Nama   | Elemen  | Role      | Deskripsi Singkat |
|--------|---------|-----------|-------------------|
| Aiden  | 🔥 Api  | Penyerang  | Pemuda impulsif yang mewakili *Rage*. Burst damage tinggi. |
| Lyra   | 💧 Air  | Penyembuh  | Penuh empati, mewakili *Grief*. Heal dan cleanser. |
| Kael   | 🪨 Tanah| Tank       | Stoik, mewakili *Stability*. Defense tinggi, pelindung tim. |
| Mira   | 🍃 Angin | Support   | Ceria, mewakili *Freedom*. Speed buff dan debuff musuh. |

### Karakter Unlock (Chapter 2–4)

| Nama   | Elemen  | Role      | Cara Unlock |
|--------|---------|-----------|-------------|
| Elara  | ✨ Cahaya | Penyerang | Chapter 2, setelah membantu desa. Mewakili *Hope*. |
| Vex    | 🌑 Gelap | Penyerang | Chapter 2, memutuskan bergabung setelah kalah. Mewakili *Despair*. |
| Orion  | ✨ Cahaya | Support   | Chapter 3, pengendali waktu. |
| Nyx    | 🌑 Gelap | Penyerang | Chapter 3, chaos lover. |
| Ignis  | 🔥 Api  | Penyerang | Chapter 3, berserker. |
| Nereid | 💧 Air  | Penyembuh  | Chapter 3, healer murni. |
| Zeph   | 🍃 Angin | Penyerang | Chapter 4, dodge specialist. |
| Terra  | 🪨 Tanah| Tank       | Chapter 4, counter attack tank. |
| Solis  | ✨ Cahaya | Support   | Chapter 4, buffer tim. |

Total **12 karakter**; tim aktif maksimal **4**.

---

## ⚔️ Sistem Pertarungan

### Turn Order
- Berdasarkan **speed** setiap karakter (diurutkan ulang tiap turn)
- Karakter dengan speed lebih tinggi bergerak lebih dulu

### Aksi dalam Giliran
1. **Serangan Dasar** → damage fisik + **+10 Energy**
2. **Skill** → damage/efek spesial, butuh Energy (10–50), bisa healing atau debuff
3. **Ultimate** → skill terkuat, butuh **100 Energy**
4. **Bertahan** → mengurangi damage yang diterima sebesar 50% untuk 1 giliran

### Sistem Elemen (Keuntungan & Kelemahan)

| Elemen | Kuat Terhadap | Lemah Terhadap |
|--------|---------------|----------------|
| 🔥 Api  | 🍃 Angin       | 💧 Air          |
| 💧 Air  | 🔥 Api         | 🪨 Tanah        |
| 🪨 Tanah| 💧 Air         | 🍃 Angin        |
| 🍃 Angin| 🪨 Tanah       | 🔥 Api          |
| ✨ Cahaya| 🌑 Gelap      | —               |
| 🌑 Gelap| ✨ Cahaya     | —               |

Damage multiplier: **1.5x** jika kuat, **0.5x** jika lemah.

### Break System
- Setiap musuh punya **Toughness**
- Serangan dengan elemen kuat mengurangi Toughness
- Jika Toughness habis → **BREAK** → musuh **stun 1 turn** + bonus damage

### Energy & Ultimate
- Setiap serangan dasar memberi **+10 Energy** (max 100)
- Skill mengonsumsi Energy (sesuai biaya)
- Ultimate menghabiskan 100 Energy
- Energy bertahan lintas giliran

### Synergy (Bonus Tim)
- **2 elemen sama** dalam tim → +10 ATK untuk semua karakter elemen tersebut
- **2 Support** dalam tim → semua karakter **regen +5 HP** setiap turn
- **2 Healer** → regen tambahan +3 HP

### Affinity (Hubungan Karakter)
- Bertambah melalui pilihan dialog di cerita
- **Affinity ≥ 30** → bonus ATK +5 untuk kedua karakter dalam battle
- Affinity tinggi juga membuka **secret ending**

### Status Effect
| Efek      | Durasi | Efek |
|-----------|--------|------|
| 🔥 Burn   | 2 turn | Damage per turn (5–10% max HP) |
| ❄️ Freeze | 1 turn | Kehilangan giliran |
| ⚡ Buff Atk| 3 turn | Meningkatkan ATK |
| 🛡️ Debuff Def| 3 turn | Menurunkan DEF |

---

## 🖥️ Cara Install & Running

### Persyaratan
- **Termux** (Android) atau **Linux/macOS/Windows (WSL2)**
- **Rust** dan **Cargo** (minimal versi 1.70)

### 1. Install Rust (jika belum)

**Termux:**
```bash
pkg update && pkg upgrade
pkg install rust
```

Linux/macOS:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

Windows (WSL2): ikuti langkah Linux di dalam WSL.

2. Clone repository

```bash
git clone https://github.com/username/aetherfall_rpg.git
cd aetherfall_rpg
```

3. Build dan Jalankan

```bash
cargo build --release
cargo run
```

Catatan: Pertama kali build akan mengunduh dependency (rand, serde). Butuh koneksi internet.

4. Menyimpan & Memuat Game

· Save otomatis setelah setiap chapter (file savegame.json)
· Load dari menu utama (opsi 2)
· Hapus save untuk mulai dari awal: rm savegame.json

---

🕹️ Cara Bermain (Panduan Cepat)

Menu Utama

```
1. Mulai Petualangan Baru
2. Load Game
3. Team Builder (coming soon)
4. Affinity Status (lihat hubungan karakter)
5. Keluar
```

Dalam Battle

1. Pilih aksi (1–4)
2. Jika pilih Skill atau Ultimate, pilih target:
   · Skill Heal → target sekutu (bisa diri sendiri)
   · Skill Damage / Debuff → target musuh
3. Serangan dasar dan skill menghasilkan Energy
4. Gunakan Ultimate (butuh 100 Energy) untuk serangan pamungkas

Tips Kemenangan

· Manfaatkan elemen untuk damage ekstra
· Fokus pada satu musuh untuk mengurangi jumlah lawan cepat
· Gunakan defend saat HP rendah
· Jaga Energy untuk ultimate di saat kritis
· Bentuk tim dengan sinergi (2 elemen sama atau 2 support)
· Tingkatkan affinity untuk bonus ATK

---

📁 Struktur Proyek (Untuk Developer)

```
src/
├── core/          # Game state, config, save/load, game loop
├── battle/        # Battle engine, turn manager, AI musuh
├── entity/        # Character, skill, effect, element, role
├── systems/       # Status effect, buff, synergy
├── story/         # Chapter 1-4, final battle, ending, dialogue
├── data/          # Database karakter & musuh (12 karakter + 7 boss)
├── ui/            # Menu, battle UI, text formatting, affinity panel
└── utils/         # Input handler, random utilities
```

---

🚧 Fitur yang Akan Datang

· Team Builder (pilih 4 karakter dari 12 yang sudah di-unlock)
· Equipment system (senjata, armor, artifact)
· Boss phase (AI berubah saat HP < 50%)
· Relationship events (cutscene khusus affinity tinggi)
· More endings (based on choices during story)

---

🤝 Kontribusi

Pull request sangat diterima. Area yang bisa dikontribusi:

· Menambah karakter baru
· Menyeimbangkan damage formula
· Meningkatkan AI musuh
· Menambah item (potion, elixir)
· Bug fixing

Pastikan cargo build dan cargo run masih berjalan tanpa error.

---

📜 Lisensi

MIT License — bebas digunakan, dimodifikasi, dan didistribusikan.
Dibuat dengan ❤️ oleh [Nama Anda].
Terinspirasi dari Honkai: Star Rail, Final Fantasy, dan classic JRPG.

---

🌟 Penutup

“Dunia bukan tentang keseimbangan. Dunia adalah tentang pilihan.”
— Weaver

Selamat bermain, dan jadilah Weaver yang menentukan nasib Aetherfall! 🧙‍♂️✨

GitHub Repository — ★ bintang jika suka!
