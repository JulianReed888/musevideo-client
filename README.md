# musevideo-client

Typed Rust primitives for describing generation jobs on [Muse Video](https://musevideo.art/) — an AI platform that turns text, images, and existing footage into finished video.

Rather than wrapping any single HTTP endpoint, this crate gives you small, dependency-free building blocks: a request you can construct fluently, an aspect-ratio enum that covers the common social formats, and optional Serde derives so the same structs move cleanly between your service, a queue, and disk.

## What's inside

- `VideoRequest` — a builder-style prompt request (prompt, negative prompt, duration, model)
- `AspectRatio` — widescreen, vertical, square, or a custom ratio string
- Zero required dependencies; enable the `serde` feature only when you need (de)serialization

## Quick start

```rust
use musevideo_client::{VideoRequest, AspectRatio};

let job = VideoRequest::new("A slow drone pass over a neon-lit night market")
    .aspect_ratio(AspectRatio::Vertical)
    .duration_seconds(6)
    .negative_prompt("text overlays, watermark");

assert_eq!(job.duration_seconds, 6);
```

Turn on serialization when you need to persist or transmit a job:

```toml
musevideo-client = { version = "0.1", features = ["serde"] }
```

## When to reach for it

CLIs, batch workers, and backend services that assemble Muse Video jobs tend to re-invent the same prompt/parameter structs. Keeping them in one small, portable crate makes those jobs easy to build, log, and pass around without pulling in a heavy SDK.

Docs and the full platform live at [musevideo.art](https://musevideo.art/).
