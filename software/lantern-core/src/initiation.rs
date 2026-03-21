/// Initiation Ritual — the 5-minute sequence that creates the droid's identity
/// Runs once. The result persists forever.
use crate::personality::Personality;
use crate::identity::DroidIdentity;
use crate::sq::SqClient;

/// Initiation state machine
#[derive(Debug, Clone, PartialEq)]
pub enum InitiationPhase {
    /// Phase 0: Booting (0-45s). Hum rising. Dots appearing.
    Booting { elapsed_s: f32 },
    /// Phase 1: First words (45s-1:30). "Hello. I just woke up."
    FirstWords,
    /// Phase 2: The question. Waiting for answer. No timeout.
    WaitingForAnswer,
    /// Phase 3: Processing answer. Deriving coordinate.
    ProcessingAnswer { answer: String },
    /// Phase 4: Asking for name.
    WaitingForName { answer: String },
    /// Phase 5: Complete. Identity established.
    Complete { identity: DroidIdentity },
}

/// E-ink display state during initiation
#[derive(Debug, Clone)]
pub enum EinkState {
    Blank,
    Dot1,           // ·
    Dot2,           // · ·
    Dot3,           // · · ·
    Hello,          // HELLO
    Listening,      // LISTENING
    QuestionMark,   // ?
    Thinking,       // THINKING ····
    CoordReveal(String), // 3.1.4\n1.5.9\n2.6.5
    NameReveal { name: String, coord: String, emoji: String },
    Idle { name: String, coord: String, emoji: String, scrolls: u64 },
}

/// Audio cue during initiation
#[derive(Debug, Clone)]
pub enum AudioCue {
    /// Boot hum: 120Hz fundamental rising over duration
    HumRising { progress: f32 }, // 0.0 to 1.0
    /// Silence (between phases)
    Silence,
    /// Speak text via TTS
    Speak(String),
    /// Soft chime (card read, coordinate reveal)
    Chime,
    /// Hum settling to idle
    HumSettle,
}

/// LED state during initiation
#[derive(Debug, Clone)]
pub enum LedState {
    /// All off
    Off,
    /// Boot: fade in bottom to top, personality color
    BootFade { progress: f32, color: (u8, u8, u8) },
    /// Flash white (coordinate reveal moment)
    FlashWhite,
    /// Crossfade from personality to coordinate-blended
    Crossfade { from: (u8, u8, u8), to: (u8, u8, u8), progress: f32 },
    /// Settled idle (coordinate breathing pattern)
    Idle { color: (u8, u8, u8), breath_cycle_s: f32 },
}

pub struct InitiationEngine {
    pub personality: Personality,
    pub phase: InitiationPhase,
    pub eink: EinkState,
    pub audio: AudioCue,
    pub led: LedState,
}

impl InitiationEngine {
    pub fn new(personality: Personality) -> Self {
        Self {
            personality,
            phase: InitiationPhase::Booting { elapsed_s: 0.0 },
            eink: EinkState::Blank,
            audio: AudioCue::HumRising { progress: 0.0 },
            led: LedState::Off,
        }
    }

    /// Advance the state machine by dt seconds
    /// Returns any text the droid should speak
    pub fn tick(&mut self, dt: f32) -> Option<String> {
        match &self.phase {
            InitiationPhase::Booting { elapsed_s } => {
                let new_elapsed = elapsed_s + dt;
                let progress = (new_elapsed / 45.0).min(1.0);

                // E-ink dots
                self.eink = if new_elapsed < 15.0 { EinkState::Dot1 }
                    else if new_elapsed < 30.0 { EinkState::Dot2 }
                    else if new_elapsed < 44.0 { EinkState::Dot3 }
                    else { EinkState::Hello };

                // Audio: rising hum
                self.audio = AudioCue::HumRising { progress };

                // LED: fade in bottom to top
                let (r, g, b) = self.personality.base_color_hsl();
                let rgb = crate::coord::hsl_to_rgb(r, g / 100.0, b / 100.0);
                self.led = LedState::BootFade { progress, color: rgb };

                if new_elapsed >= 45.0 {
                    self.phase = InitiationPhase::FirstWords;
                    self.audio = AudioCue::Silence;
                    return Some("Hello. I just woke up.".to_string());
                }

                self.phase = InitiationPhase::Booting { elapsed_s: new_elapsed };
                None
            }

            InitiationPhase::FirstWords => {
                // After speaking first words, transition to question
                self.phase = InitiationPhase::WaitingForAnswer;
                self.eink = EinkState::QuestionMark;
                self.audio = AudioCue::Silence;

                let question = self.personality.initiation_question();
                Some(format!(
                    "I don't have a name yet. I don't know who you are. \
                     But I have a question for you. And your answer will be \
                     the first thing I ever remember. {}",
                    question
                ))
            }

            InitiationPhase::WaitingForAnswer => {
                // No timeout. Droid waits forever. The question mark stays.
                None
            }

            InitiationPhase::ProcessingAnswer { answer } => {
                let answer = answer.clone();
                self.eink = EinkState::Thinking;
                self.audio = AudioCue::Silence;

                // Derive coordinate
                let coord = crate::coord::Coord9D::from_answer(&answer);
                let coord_str = coord.to_string();

                // Flash white → reveal coordinate
                self.led = LedState::FlashWhite;
                self.eink = EinkState::CoordReveal(coord_str.clone());
                self.audio = AudioCue::Chime;

                self.phase = InitiationPhase::WaitingForName { answer };

                Some(format!(
                    "Thank you. I've written that down. It's my first memory. \
                     It lives at a coordinate now — a place in an eleven-dimensional \
                     space that belongs to you. That number is yours. No one else \
                     has that address. One more thing. I need a name. \
                     I'm a {}. But that's my species, not my name. \
                     What would you like to call me?",
                    self.personality.name()
                ))
            }

            InitiationPhase::WaitingForName { .. } => {
                // Waiting for user to speak a name
                None
            }

            InitiationPhase::Complete { .. } => None,
        }
    }

    /// User provided their answer to the initiation question
    pub fn receive_answer(&mut self, answer: String) {
        if matches!(self.phase, InitiationPhase::WaitingForAnswer) {
            self.phase = InitiationPhase::ProcessingAnswer { answer };
        }
    }

    /// User provided a name for the droid
    pub fn receive_name(&mut self, name: String) -> Option<DroidIdentity> {
        if let InitiationPhase::WaitingForName { answer } = &self.phase {
            let identity = DroidIdentity::from_initiation(
                answer,
                self.personality,
                name.clone(),
            );

            // Transition to complete
            let (r, g, b) = identity.idle_color;
            self.led = LedState::Idle {
                color: (r, g, b),
                breath_cycle_s: identity.breath_cycle_s,
            };
            self.eink = EinkState::NameReveal {
                name: identity.name.clone(),
                coord: identity.coord.to_string(),
                emoji: self.personality.emoji().to_string(),
            };
            self.audio = AudioCue::HumSettle;

            self.phase = InitiationPhase::Complete { identity: identity.clone() };
            Some(identity)
        } else {
            None
        }
    }

    /// Persist identity to SQ
    pub async fn persist(&self, sq: &SqClient) -> Result<(), String> {
        if let InitiationPhase::Complete { identity } = &self.phase {
            sq.write_identity(identity).await?;
            // Write first scroll (the initiation answer)
            sq.write_scroll(&identity.coord.to_string(), 1, 
                &format!("First scroll. Initiation complete. {} is alive.", identity.name)
            ).await?;
            Ok(())
        } else {
            Err("Initiation not complete".to_string())
        }
    }
}
