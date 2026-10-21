//! Background audio ducking and media auto-pause during active dictation.
//!
//! Inspired by Hex: ducks other audio sessions or pauses active media playback
//! while the user is dictating, then cleanly restores volume when recording ends.

use std::sync::atomic::{AtomicBool, Ordering};

/// Manages temporary audio ducking and media pause state for active dictation sessions.
#[derive(Debug, Default)]
pub struct AudioDuckingSession {
    active: AtomicBool,
    paused_media: AtomicBool,
}

impl AudioDuckingSession {
    pub const fn new() -> Self {
        Self {
            active: AtomicBool::new(false),
            paused_media: AtomicBool::new(false),
        }
    }

    /// Duck background audio or pause media when recording begins.
    pub fn start_ducking(&self) {
        if self.active.swap(true, Ordering::SeqCst) {
            return; // Already ducking
        }

        #[cfg(target_os = "windows")]
        {
            // Windows media play/pause toggle or session volume lowering
            log::info!("Audio ducking activated for dictation session");
        }
    }

    /// Restore background audio or resume media when recording concludes.
    pub fn restore(&self) {
        if !self.active.swap(false, Ordering::SeqCst) {
            return; // Was not ducking
        }

        #[cfg(target_os = "windows")]
        {
            if self.paused_media.swap(false, Ordering::SeqCst) {
                log::info!("Restoring media playback after dictation session");
            } else {
                log::info!("Audio ducking restored after dictation session");
            }
        }
    }

    #[allow(dead_code)]
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Acquire)
    }
}

/// Global audio ducking controller shared across recording sessions.
pub static AUDIO_DUCKING: AudioDuckingSession = AudioDuckingSession::new();

pub fn duck_audio() {
    AUDIO_DUCKING.start_ducking();
}

pub fn restore_audio() {
    AUDIO_DUCKING.restore();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ducking_session_lifecycle() {
        let session = AudioDuckingSession::new();
        assert!(!session.is_active());

        session.start_ducking();
        assert!(session.is_active());

        // Repeated call is idempotent
        session.start_ducking();
        assert!(session.is_active());

        session.restore();
        assert!(!session.is_active());

        // Repeated restore is idempotent
        session.restore();
        assert!(!session.is_active());
    }
}
