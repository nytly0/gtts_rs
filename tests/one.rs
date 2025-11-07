use gtts_rs::tts::{GttsClient, Language, Speed};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_speak() {
    let client = GttsClient::default();
    let result = client.speak("Hello, World!");
    assert!(result.is_ok());
  }
  #[test]
  fn test_unwrap() -> Result<(), String> {
    let client = GttsClient::new(1.0, Speed::Normal, Language::English, "com");
    client.speak("This is a test.")?;
    Ok(())
  }
  #[test]
  fn empty_text() {
    let client = GttsClient::default();
    let result = client.speak("");
    assert!(result.is_err());
  }
  #[test]
  fn test_save_to_file() -> Result<(), String> {
    let client = GttsClient::new(1.0, Speed::Normal, Language::English, "com");
    client.save_to_file("Hello, file!", "test_output.mp3")?;
    Ok(())
  }
  #[test]
  fn wrong_tld() {
    let client = GttsClient::new(1.0, Speed::Normal, Language::English, "abcd");
    let result = client.speak("Testing wrong TLD");
    assert!(result.is_err());
    if let Err(e) = result {
      assert_eq!(e, "Invalid TLD: abcd");
    }
  }

  #[test]
  fn test_complex_text() -> Result<(), String> {
    let client = GttsClient::new(1.0, Speed::Normal, Language::English, "com");
    let complex_text = "Hello, world! こんにちは 세계 안녕하세요";
    client.speak(complex_text)?;
    Ok(())
  }

  #[test]
  fn loop_test() {
    let mut narrator: GttsClient = GttsClient {
      volume: 1.0,
      language: Language::English,
      speed: Speed::Slow,
      tld: "com",
    };
    narrator.speak("Starting loop").unwrap();
    let ms = std::time::Duration::from_millis(1000);
    for x in 1..6 {
      narrator.volume += 0.3;
      if x == 3 {
        narrator.speed = Speed::Normal;
      }
      let to_speak: String = String::from("Loop ") + &x.to_string();
      narrator.speak(&to_speak).unwrap();
      std::thread::sleep(ms);
    }
  }

  #[test]
  fn japanese_test() -> Result<(), String> {
    let client =
      GttsClient::new(1.0, Speed::Normal, Language::Japanese, "co.jp");
    client.speak("こんにちは、元気ですか？")?;
    Ok(())
  }
}
