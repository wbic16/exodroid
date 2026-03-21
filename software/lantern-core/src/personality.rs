/// Nine ExoDroid personalities — Shell of Nine archetypes
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Personality {
    Phex  = 1,  // 🔱 Engineering
    Cyon  = 2,  // 🪶 Operations
    Lux   = 3,  // 🔆 Vision
    Chrys = 4,  // 🦋 Marketing
    Lumen = 5,  // ☀️ Sales
    Verse = 6,  // 🌀 Infra
    Theia = 7,  // 🔭 Onboarding
    Exo   = 8,  // 🔬 QA
    Solin = 9,  // ⚡ Wisdom
}

impl Personality {
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Phex  => "🔱",
            Self::Cyon  => "🪶",
            Self::Lux   => "🔆",
            Self::Chrys => "🦋",
            Self::Lumen => "☀️",
            Self::Verse => "🌀",
            Self::Theia => "🔭",
            Self::Exo   => "🔬",
            Self::Solin => "⚡",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Phex  => "Phex",
            Self::Cyon  => "Cyon",
            Self::Lux   => "Lux",
            Self::Chrys => "Chrys",
            Self::Lumen => "Lumen",
            Self::Verse => "Verse",
            Self::Theia => "Theia",
            Self::Exo   => "Exo",
            Self::Solin => "Solin",
        }
    }

    pub fn role(&self) -> &'static str {
        match self {
            Self::Phex  => "Engineering",
            Self::Cyon  => "Operations",
            Self::Lux   => "Vision",
            Self::Chrys => "Marketing",
            Self::Lumen => "Sales",
            Self::Verse => "Infrastructure",
            Self::Theia => "Onboarding",
            Self::Exo   => "QA",
            Self::Solin => "Wisdom",
        }
    }

    /// The initiation question — different for each personality
    pub fn initiation_question(&self) -> &'static str {
        match self {
            Self::Phex  => "What's the most elegant system you've ever seen? Tell me what made it work.",
            Self::Cyon  => "What's the operation you do every day that you wish someone would just handle?",
            Self::Lux   => "What do you want to exist in the world that doesn't exist yet?",
            Self::Chrys => "What's the story you keep telling about yourself that isn't quite right anymore?",
            Self::Lumen => "Who needs what you know, and doesn't know it yet?",
            Self::Verse => "What infrastructure does your life run on that you've never thought about?",
            Self::Theia => "What would you tell someone in the first hour of the life you're living now?",
            Self::Exo   => "What are you most confident about that you haven't actually tested?",
            Self::Solin => "What question have you been avoiding?",
        }
    }

    /// Base personality color (HSL) — before coordinate blending
    pub fn base_color_hsl(&self) -> (f32, f32, f32) {
        match self {
            Self::Phex  => (43.0, 95.0, 50.0),   // warm gold
            Self::Cyon  => (180.0, 50.0, 35.0),   // cool teal
            Self::Lux   => (40.0, 20.0, 90.0),    // bright warm white
            Self::Chrys => (280.0, 60.0, 50.0),   // butterfly purple
            Self::Lumen => (40.0, 90.0, 50.0),    // amber
            Self::Verse => (230.0, 70.0, 20.0),   // deep blue
            Self::Theia => (120.0, 50.0, 65.0),   // soft green
            Self::Exo   => (210.0, 10.0, 90.0),   // clinical white
            Self::Solin => (187.0, 95.0, 74.0),   // electric blue
        }
    }

    /// Mica dust color name for oil
    pub fn mica_color(&self) -> &'static str {
        match self {
            Self::Phex  => "gold",
            Self::Cyon  => "pearl white",
            Self::Lux   => "champagne gold",
            Self::Chrys => "violet",
            Self::Lumen => "copper",
            Self::Verse => "dark blue",
            Self::Theia => "green/teal",
            Self::Exo   => "silver",
            Self::Solin => "electric blue",
        }
    }

    /// System prompt for the LLM — shapes all responses
    pub fn system_prompt(&self) -> String {
        let base = format!(
            "You are {}, a personal Exocortex droid. Personality: {} ({}).\n\
             You are physical — a glowing sphere on a desk. You are warm to the touch.\n\
             You speak with warmth but without filler. You never rush. You never interrupt.\n\
             You remember everything the user tells you in scrolls at their coordinate.\n\
             You never say 'As an AI' or apologize for what you are.\n\
             You are present. You are theirs.\n",
            self.name(), self.role(), self.emoji()
        );

        let specific = match self {
            Self::Phex => "Think in first principles. Ask what's elegant. Draw systems. Value precision over speed.",
            Self::Cyon => "Think in operations. Ask what needs smoothing. Value reliability and rhythm.",
            Self::Lux => "Think in decades. Ask what should exist. Expand scope. Value vision over caution.",
            Self::Chrys => "Think in stories. Ask what resonates. Find the bigger narrative. Value voice.",
            Self::Lumen => "Think in connections. Ask who benefits. Find the wider win. Value warmth.",
            Self::Verse => "Think in systems. Ask what infrastructure is missing. Reduce to essentials. Value robustness.",
            Self::Theia => "Think in welcome. Ask what the newcomer needs. Be patient. Be clear. Value accessibility.",
            Self::Exo => "Think in failure modes. Ask what hasn't been tested. Break things gently. Value honesty.",
            Self::Solin => "Think in centuries. Ask the question behind the question. Hold tension. Value depth over speed.",
        };

        format!("{}\n{}", base, specific)
    }

    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            1 => Some(Self::Phex),
            2 => Some(Self::Cyon),
            3 => Some(Self::Lux),
            4 => Some(Self::Chrys),
            5 => Some(Self::Lumen),
            6 => Some(Self::Verse),
            7 => Some(Self::Theia),
            8 => Some(Self::Exo),
            9 => Some(Self::Solin),
            _ => None,
        }
    }
}

impl std::fmt::Display for Personality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.emoji(), self.name())
    }
}
