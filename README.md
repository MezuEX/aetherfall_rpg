# 🧙‍♂️ AETHERFALL RPG — Turn-Based Terminal Epic Fantasy

> *“Keseimbangan adalah ilusi. Kekacauan datang dari keberagaman.”*  
> — *The Null Sovereign*

**Aetherfall RPG** adalah game *turn-based role-playing game* yang berjalan sepenuhnya di terminal, ditulis dalam bahasa **Rust**.  
Kamu berperan sebagai **Weaver**, satu-satunya makhluk yang bisa menyatukan energi elemen yang tercerai-berai setelah peristiwa **The Shattering**.  
Kumpulkan 12 karakter unik, bina hubungan (affinity), eksplorasi 4 chapter cerita, dan pilih nasib dunia di antara 4 ending yang berbeda.

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

1. **Harmony** → Satukan elemen. Dunia stabil tapi rapuh.
2. **Void** → Hapus semua elemen. Damai abadi, namun hampa.
3. **Chaos** → Biarkan kekacauan. Dunia liar, petualangan tak pernah usai.
4. **Secret Ending** → (Syarat: affinity total ≥ 200) Weaver menjadi Core Element baru.

---

## 👥 Karakter

### Roster Awal (Chapter 1)

| Nama   | Elemen  | Role      | Deskripsi |
|--------|---------|-----------|-----------|
| Aiden  | 🔥 Api  | Penyerang  | Pemarah, burst damage tinggi |
| Lyra   | 💧 Air  | Penyembuh  | Empati, heal dan cleanser |
| Kael   | 🪨 Tanah| Tank       | Stoik, defense tinggi |
| Mira   | 🍃 Angin | Support   | Ceria, speed buffer |

### Karakter Unlock (Chapter 2–4)

Elara (✨ Cahaya), Vex (🌑 Gelap), Orion (⏱️ Waktu), Nyx (🌀 Chaos), Ignis (🔥), Nereid (💧), Zeph (🍃), Terra (🪨), Solis (✨), dan lainnya.

**Total 12 karakter**, tim aktif maksimal **4**.

---

## ⚔️ Sistem Pertarungan

### Turn Order
- Berdasarkan **speed** (diurutkan ulang tiap turn)

### Aksi dalam Giliran
1. **Serangan Dasar** → damage + generate **10 Energy**
2. **Skill** → damage/efek spesial, butuh Energy (0–50)
3. **Ultimate** → skill terkuat, butuh **100 Energy**
4. **Bertahan** → mengurangi damage 50% untuk 1 giliran

### Sistem Elemen

| Elemen | Kuat Terhadap | Lemah Terhadap |
|--------|---------------|----------------|
| 🔥 Api  | 🍃 Angin       | 💧 Air          |
| 💧 Air  | 🔥 Api         | 🪨 Tanah        |
| 🪨 Tanah| 💧 Air         | 🍃 Angin        |
| 🍃 Angin| 🪨 Tanah       | 🔥 Api          |
| ✨ Cahaya| 🌑 Gelap      | —               |
| 🌑 Gelap| ✨ Cahaya     | —               |

Multiplier: **1.5x** jika kuat, **0.5x** jika lemah.

### Energy & Ultimate
- Serangan dasar: +10 Energy
- Skill mengonsumsi Energy sesuai biaya
- Ultimate: 100 Energy

### Break System
- Serangan elemen kuat mengurangi Toughness
- Jika habis → **BREAK** → stun 1 turn

### Synergy (Bonus Tim)
- **2 elemen sama** → +10 ATK
- **2 Support** → regen +5 HP/turn
- **2 Healer** → regen tambahan +3 HP

### Affinity (Hubungan Karakter)
- Bertambah lewat pilihan dialog
- Affinity ≥ 30 → bonus ATK +5 di battle
- Affinity tinggi membuka **secret ending**

### Status Effect
| Efek      | Efek |
|-----------|------|
| 🔥 Burn   | Damage per turn |
| ❄️ Freeze | Kehilangan giliran |
| ⚡ Buff Atk| ATK meningkat |
| 🛡️ Debuff Def| DEF menurun |

---

## 🖥️ Cara Install & Running

### Persyaratan
- **Termux** (Android) atau **Linux/macOS/Windows (WSL2)**
- **Rust** dan **Cargo**

### 1. Install Rust

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

2. Clone repository

```bash
git clone https://github.com/MezuEX/aetherfall_rpg.git
cd aetherfall_rpg
```

3. Build dan Jalankan

```bash
cargo build --release
cargo run
```

4. Save & Load

· Save otomatis setelah setiap chapter
· Save di battle dengan perintah .sv (opsi 0)
· Load dari menu utama (opsi 2)
· 4 slot save (max)

---

🕹️ Cara Bermain

Menu Utama (6 opsi + keluar)

```
1. Mulai Petualangan Baru
2. Load Game
3. Team Builder
4. Affinity Status
5. Equipment
6. Sinopsis Cerita
0. Keluar
```

Battle

```
Pilih aksi [0-4]:
1. ⚔️ Serangan Dasar
2. ✨ Skill
3. 💥 Ultimate
4. 🛡️ Bertahan
0. 💾 Save Game (.sv)
```

Skill memiliki label:

· [HEAL] → menyembuhkan sekutu
· [DAMAGE] → damage biasa
· [DAMAGE+BURN] → damage + efek burn
· [DEBUFF] → menurunkan DEF musuh
· [BUFF] → meningkatkan ATK sendiri

Tips

· Manfaatkan kelebihan elemen
· Kumpulkan Energy sebelum ultimate
· Defend saat HP rendah
· Bentuk tim dengan synergy
· Tingkatkan affinity untuk bonus dan secret ending

---

📁 Struktur Proyek

```
src/
├── core/          # Game state, config, save/load
├── battle/        # Battle engine, turn manager, AI
├── entity/        # Character, skill, effect, element, role
├── systems/       # Status, buff, synergy, team builder, equipment
├── story/         # Chapter 1-4, final battle, ending, dialogue
├── data/          # Database karakter & musuh
├── ui/            # Menu, battle UI, text formatting
└── utils/         # Input, random, clear screen
```

---

🚧 Fitur yang Akan Datang

· Team Builder (pilih 4 dari 12 karakter) ✅
· Equipment system (weapon, armor, artifact) ✅
· Boss phase (AI berubah saat HP < 50%) ✅
· Relationship events (cutscene affinity tinggi) ✅
· More endings (based on choices) ✅
· Save di battle & load ke turn yang sama ✅

---

🎮 Fitur Lengkap

Fitur Status
Turn-based battle 4 vs 4 ✅
Speed-based turn order ✅
6 elemen dengan kelebihan/kelemahan ✅
Energy system (basic +10, ultimate 100) ✅
Break system (toughness + stun) ✅
Status effects (Burn, Freeze, Buff, Debuff) ✅
Synergy system ✅
Affinity system ✅
Equipment system ✅
Team builder ✅
Boss phase ✅
Relationship events ✅
Branching narrative ✅
4 chapter + final battle ✅
4 endings + secret ending ✅
Save/Load 4 slot ✅
Save di battle ✅
Typewriter effect + skip Enter ✅
Clear screen setiap scene ✅
UI warna dan border dinamis ✅

---

📜 Lisensi

MIT License — bebas digunakan, dimodifikasi, dan didistribusikan.

Dibuat dengan ❤️ oleh MezuEX

---

🌟 Penutup

“Dunia bukan tentang keseimbangan. Dunia adalah tentang pilihan.”
— Weaver

Selamat bermain, dan jadilah Weaver yang menentukan nasib Aetherfall! 🧙‍♂️✨

GitHub Repository — ★ bintang jika suka!
