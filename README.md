# musevideo-client

A lightweight Rust helper toolkit for building [Muse Video](https://musevideo.art/) generation workflows.

Muse Video is an AI video generation platform focused on text, image and video to video generation, visual reference handling, scene planning, and structured generation pipelines.

This crate provides small Rust types and helpers for representing prompt requests, visual references, generation settings, and workflow metadata.

## Features

- Prompt request structures
- Video generation settings (aspect ratio, duration, resolution)
- Reference image metadata
- Workflow state helpers
- Optional Serde support for serialization

## Example

```rust
use musevideo_client::{VideoRequest, AspectRatio};

let request = VideoRequest::new("A cinematic aerial shot of a futuristic coastal city")
    .aspect_ratio(AspectRatio::Widescreen)
    .duration_seconds(8);

assert_eq!(request.duration_seconds, 8);
```

## Why this crate?

AI video workflows often need structured prompt data, generation parameters, and consistent metadata. This crate keeps those pieces simple and portable for Rust-based tools, CLIs, and backend services.

Learn more about [Muse Video](https://musevideo.art/).
