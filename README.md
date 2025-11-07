# `gtts_rs`

A Rust library for Google Translate’s text-to-speech API.

- [crates.io](https://crates.io/crates/gtts_rs)
- [docs.rs](https://docs.rs/gtts_rs/)

## Example

```rust
use gtts_rs::tts::{ GttsClient, Language, Speed };

fn main() {
    let mut narrator = GttsClient {
        volume: 1.0,
        speed: Speed::Normal,
        language: Language::English,
        tld: "com",
    };
    narrator.speak("Hello, World!").unwrap();
}
```

### ...Or a more advanced one

```rust
use tts_rust::{ tts::GTTSClient, languages::Languages };

fn main() {
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::English,
        tld: "com",
    };
    narrator.speak("Starting test?").unwrap();
    let ms = std::time::Duration::from_millis(1000);
    for _x in 1..9 {
        narrator.volume += 1.0;
        let to_speak: String = String::from("Loop ") + &narrator.volume.to_string();
        narrator.speak(&to_speak).unwrap();
        std::thread::sleep(ms);
    }
}
```

### License

#### MIT
