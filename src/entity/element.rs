use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Element {
    Fire,
    Water,
    Wind,
    Earth,
    Light,
    Dark,
}

impl Element {
    pub fn name(&self) -> &str {
        match self {
            Element::Fire => "🔥 Api",
            Element::Water => "💧 Air",
            Element::Wind => "🍃 Angin",
            Element::Earth => "🪨 Tanah",
            Element::Light => "✨ Cahaya",
            Element::Dark => "🌑 Gelap",
        }
    }

    pub fn advantage(&self, target: Element) -> f32 {
        match (*self, target) {
            (Element::Fire, Element::Wind) => 1.5,
            (Element::Wind, Element::Earth) => 1.5,
            (Element::Earth, Element::Water) => 1.5,
            (Element::Water, Element::Fire) => 1.5,
            (Element::Light, Element::Dark) => 1.5,
            (Element::Dark, Element::Light) => 1.5,
            (Element::Fire, Element::Water) => 0.5,
            (Element::Wind, Element::Fire) => 0.5,
            (Element::Earth, Element::Wind) => 0.5,
            (Element::Water, Element::Earth) => 0.5,
            _ => 1.0,
        }
    }
}
