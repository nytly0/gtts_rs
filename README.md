# gtts_rs

A Rust library for Google Translate’s text-to-speech API.

- [crates.io](https://crates.io/crates/gtts_rs)
- [docs.rs](https://docs.rs/gtts_rs/)

## Example

```rust
use gtts_rs::tts::{ GttsClient, Language, Speed };

fn main() -> Result<(), String> {
    let client = GttsClient::default();
    client.speak("Hello, world!")?;
    Ok(())
}
```

with custom configuration:

```rust
use gtts_rs::tts::{ GttsClient, Language, Speed };

fn main() -> Result<(), String> {
    let mut client: GttsClient = GttsClient {
        volume: 1.0,
        language: Language::English,
        speed: Speed::Slow,
        tld: "com",
    };
    client.speak("Hello, world!")?;
    Ok(())
}
```

or a different language:

```rust
use gtts_rs::tts::{ GttsClient, Language, Speed };

fn main() -> Result<(), String> {
    let client = GttsClient {
        volume: 1.0,
        language: Language::Japanese,
        speed: Speed::Normal,
        tld: "co.jp",
    };  
    client.speak("こんにちは、元気ですか？")?;
    Ok(())
}
```

### License

This project is licensed under the MIT License. See the LICENSE file for
details.

### Contribution

Contributions are very welcome! Please feel free to submit issues and pull
requests.
