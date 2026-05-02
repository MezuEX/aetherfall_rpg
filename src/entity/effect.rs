#[derive(Debug, Clone)]
pub enum Effect {
    Burn(u32),      // damage per turn (persen dari max_hp atau fixed disesuaikan)
    Freeze,         // skip turn
    Heal(u32),      // instant heal (raw value)
    BuffAtk(u32),   // increase attack (flat)
    DebuffDef(u32), // decrease defense (flat)
}
