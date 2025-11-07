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
}
