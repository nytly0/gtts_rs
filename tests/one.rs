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
  fn test_unwrap() {
    let client = GttsClient::new(1.0, Speed::Normal, Language::English, "com");
    client.speak("This is a test.").unwrap();
  }
}
