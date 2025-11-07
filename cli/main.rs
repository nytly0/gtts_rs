use gtts_rs::tts::{GttsClient, Language, Speed};
use quicli::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct GttsCli {
  text: String,

  #[structopt(short = "l", long = "lang", default_value = "en")]
  language: Language,
}

fn main() -> CliResult {
  let args = GttsCli::from_args();
  let client = GttsClient {
    volume: 1.0,
    speed: Speed::Normal,
    language: args.language,
    tld: "com",
  };
  if let Err(msg) = client.speak(&args.text) {
    println!("An error occurred: {}", msg);
  }
  Ok(())
}
