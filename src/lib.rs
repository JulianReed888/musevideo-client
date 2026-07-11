//! Lightweight helpers for Muse Video-style AI video generation workflows.
//!
//! This crate provides simple request, aspect-ratio, and metadata types for
//! Rust applications that need to organize AI video generation prompts.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Common video aspect ratios.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AspectRatio {
    /// 16:9 widescreen video.
    Widescreen,
    /// 9:16 vertical video for short-form platforms.
    Vertical,
    /// 1:1 square video.
    Square,
    /// Custom aspect ratio, such as "4:3" or "21:9".
    Custom(String),
}

/// A structured request for an AI video generation workflow.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoRequest {
    /// Main text prompt.
    pub prompt: String,
    /// Desired aspect ratio.
    pub aspect_ratio: AspectRatio,
    /// Duration in seconds.
    pub duration_seconds: u32,
    /// Optional negative prompt.
    pub negative_prompt: Option<String>,
    /// Optional model name or provider identifier.
    pub model: Option<String>,
}

impl VideoRequest {
    /// Creates a new video request with sensible defaults.
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            aspect_ratio: AspectRatio::Widescreen,
            duration_seconds: 8,
            negative_prompt: None,
            model: None,
        }
    }

    /// Sets the aspect ratio.
    pub fn aspect_ratio(mut self, aspect_ratio: AspectRatio) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    /// Sets the duration in seconds.
    pub fn duration_seconds(mut self, seconds: u32) -> Self {
        self.duration_seconds = seconds;
        self
    }

    /// Sets a negative prompt.
    pub fn negative_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.negative_prompt = Some(prompt.into());
        self
    }

    /// Sets a model identifier.
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_default_video_request() {
        let request = VideoRequest::new("A cinematic city scene");

        assert_eq!(request.duration_seconds, 8);
        assert_eq!(request.aspect_ratio, AspectRatio::Widescreen);
    }

    #[test]
    fn updates_video_request_fields() {
        let request = VideoRequest::new("A product video")
            .aspect_ratio(AspectRatio::Vertical)
            .duration_seconds(12)
            .negative_prompt("blurry")
            .model("muse-video");

        assert_eq!(request.duration_seconds, 12);
        assert_eq!(request.aspect_ratio, AspectRatio::Vertical);
        assert_eq!(request.negative_prompt, Some("blurry".to_string()));
        assert_eq!(request.model, Some("muse-video".to_string()));
    }
}
