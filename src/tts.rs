use minreq::get;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use crate::constants::GOOGLE_TTS_MAX_CHARS;
pub use crate::lang::Language;
use crate::url::core::Core;

#[derive(Debug)]
/// # The Google Text-to-Speech Client
/// ## Example
/// ```
/// use gtts_rs::tts::GttsClient;
/// use gtts_rs::tts::Speed;
/// use gtts_rs::tts::Language;
/// let narrator = GttsClient::new(1.0, Speed::Normal, Language::English, "com");
/// narrator.speak("Hello!").unwrap();
/// ```
pub struct GttsClient {
  /// Volume level from 0.0 to 1.0 (1.0 is recommended)
  /// Note: This only affects playback volume, not the audio file itself.
  pub volume: f32,
  /// Speed of the speech
  pub speed: Speed,
  /// Language for the TTS
  ///
  /// See `Language` enum for supported languages
  pub language: Language,
  /// Top-level domain for gTTS
  ///
  /// Examples: "com", "co.uk", "ca", etc.
  pub tld: &'static str,
}

#[derive(Debug, Clone)]
/// Speed options for speech in gTTS
pub enum Speed {
  Normal,
  Slow,
}

impl GttsClient {
  /// Creates a new gTTS client instance
  pub fn new(
    volume: f32,
    speed: Speed,
    language: Language,
    tld: &'static str,
  ) -> Self {
    GttsClient {
      volume,
      speed,
      language,
      tld,
    }
  }

  /// Save the TTS audio to a file
  pub fn save_to_file(&self, text: &str, filename: &str) -> Result<(), String> {
    let rep = self.get_tts_response(text)?;
    let bytes = rep.as_bytes();

    if bytes.is_empty() {
        return Err("Received empty TTS response".into());
    }

    let mut file = File::create(filename)
        .map_err(|e| format!("Failed to create file '{}': {}", filename, e))?;

    file.write_all(bytes)
        .map_err(|e| format!("Failed to write to '{}': {}", filename, e))?;

    Ok(())
}


  fn check_tld(&self) -> Result<(), String> {
    let valid_tlds = vec![
      "com", "co.uk", "ca", "com.au", "de", "fr", "it", "es", "nl", "co.in",
    ];
    if !valid_tlds.contains(&self.tld) {
      return Err(format!("Invalid TLD: {}", self.tld));
    }
    Ok(())
  }
  /// Get the TTS response by making a request to the translate.google.{tld}/translate_tts
  pub fn get_tts_response(
    &self,
    text: &str,
  ) -> Result<minreq::Response, String> {

    self.check_tld()?;
    let speed = match self.speed {
      Speed::Normal => "1",
      Speed::Slow => "0.2",
    };
    let len = text.len();
    if len > GOOGLE_TTS_MAX_CHARS {
      return Err(format!(
        "The text is too long. Max length is {}",
        GOOGLE_TTS_MAX_CHARS
      ));
    } else if len == 0 {
      return Err("The text is empty.".to_string());
    }
    let language = Language::as_code(self.language.clone());
    let text = Core::fragmenter(text)?;
    let base_url =
      format!("https://translate.google.{}/translate_tts", self.tld);
    let query_params = format!(
      "?ie=UTF-8&q={}&tl={}&total=1&idx=0&textlen={}&tl={}&client=tw-ob&ttsspeed={}",
      text.encoded, language, len, language, speed
    );
    let full_url = format!("{}{}", base_url, query_params);

    let rep = get(full_url).send().map_err(|e| format!("{}", e))?;
    Ok(rep)
  }

  /// Play the mp3 file at the given path with the specified volume
  fn play_mp3(&self, mp3: &str) -> Result<(), String> {
    let (_stream, handle) = rodio::OutputStream::try_default()
        .map_err(|e| e.to_string())?;
    let sink = rodio::Sink::try_new(&handle)
        .map_err(|e| e.to_string())?;
    sink.set_volume(self.volume);

    let file = File::open(mp3).map_err(|e| e.to_string())?;
    let decoder = rodio::Decoder::new(BufReader::new(file))
        .map_err(|e| e.to_string())?;

    sink.append(decoder);
    sink.sleep_until_end();

    Ok(())
}

  /// Speak the input according to the volume and language
  pub fn speak(&self, input: &str) -> Result<(), String> {
    self.save_to_file(input, "audio.mp3")?;
    self.play_mp3("audio.mp3")?;

    if let Err(e) = fs::remove_file("./audio.mp3") {
        eprintln!("Warning: failed to delete audio file: {}", e);
    }

    Ok(())
}
}

impl Default for GttsClient {
  /// Creates a default gTTS client with volume 1.0, normal speed, English language, and "com" TLD.
  fn default() -> Self {
    Self::new(1.0, Speed::Normal, Language::English, "com")
  }
}
impl Clone for GttsClient {
  /// Clones the gTTS client instance
  fn clone(&self) -> Self {
    GttsClient {
      volume: self.volume,
      speed: self.speed.clone(),
      language: self.language.clone(),
      tld: self.tld,
    }
  }
}
