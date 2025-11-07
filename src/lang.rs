use std::str::FromStr;

/// Enum containing all the Language supported by the GTTS API
#[derive(Debug, Clone)]
pub enum Language {
  /// ISO code: af
  Afrikaans,
  /// ISO code: ar       
  Arabic,
  /// ISO code: bg         
  Bulgarian,
  /// ISO code: bn
  Bengali,
  /// ISO code: bs
  Bosnian,
  /// ISO code: ca
  Catalan,
  /// ISO code: cs
  Czech,
  /// ISO code: cy
  Welsh,
  /// ISO code: da
  Danish,
  /// ISO code: de
  German,
  /// ISO code: el
  Greek,
  /// ISO code: en
  English,
  /// ISO code: eo
  Esperanto,
  /// ISO code: es
  Spanish,
  /// ISO code: et
  Estonian,
  /// ISO code: fi
  Finnish,
  /// ISO code: fr
  French,
  /// ISO code: gu
  Gujarati,
  /// ISO code: hi
  Hindi,
  /// ISO code: hr
  Croatian,
  /// ISO code: hu
  Hungarian,
  /// ISO code: hy
  Armenian,
  /// ISO code: id
  Indonesian,
  /// ISO code: is
  Icelandic,
  /// ISO code: it
  Italian,
  /// ISO code: ja
  Japanese,
  /// ISO code: jw
  Javanese,
  /// ISO code: km
  Khmer,
  /// ISO code: kn
  Kannada,
  /// ISO code: ko
  Korean,
  /// ISO code: la
  Latin,
  /// ISO code: lv
  Latvian,
  /// ISO code: mk
  Macedonian,
  /// ISO code: ml
  Malayalam,
  /// ISO code: mr
  Marathi,
  /// ISO code: my
  Burmese,
  /// ISO code: ne
  Nepali,
  /// ISO code: nl
  Dutch,
  /// ISO code: no
  Norwegian,
  /// ISO code: pl
  Polish,
  /// ISO code: pt
  Portuguese,
  /// ISO code: ro
  Romanian,
  /// ISO code: ru
  Russian,
  /// ISO code: si
  Sinhala,
  /// ISO code: sk
  Slovak,
  /// ISO code: sq
  Albanian,
  /// ISO code: sr
  Serbian,
  /// ISO code: su
  Sundanese,
  /// ISO code: sv
  Swedish,
  /// ISO code: sw
  Swahili,
  /// ISO code: ta
  Tamil,
  /// ISO code: te
  Telugu,
  /// ISO code: th
  Thai,
  /// ISO code: tl
  Filipino,
  /// ISO code: tr
  Turkish,
  /// ISO code: uk
  Ukrainian,
  /// ISO code: ur
  Urdu,
  /// ISO code: vi
  Vietnamese,
  /// ISO code: zh-CN
  Chinese,
}
impl FromStr for Language {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "af" => Ok(Language::Afrikaans),
      "ar" => Ok(Language::Arabic),
      "bg" => Ok(Language::Bulgarian),
      "bn" => Ok(Language::Bengali),
      "bs" => Ok(Language::Bosnian),
      "ca" => Ok(Language::Catalan),
      "cs" => Ok(Language::Czech),
      "cy" => Ok(Language::Welsh),
      "da" => Ok(Language::Danish),
      "de" => Ok(Language::German),
      "el" => Ok(Language::Greek),
      "en" => Ok(Language::English),
      "eo" => Ok(Language::Esperanto),
      "es" => Ok(Language::Spanish),
      "et" => Ok(Language::Estonian),
      "fi" => Ok(Language::Finnish),
      "fr" => Ok(Language::French),
      "gu" => Ok(Language::Gujarati),
      "hi" => Ok(Language::Hindi),
      "hr" => Ok(Language::Croatian),
      "hu" => Ok(Language::Hungarian),
      "hy" => Ok(Language::Armenian),
      "id" => Ok(Language::Indonesian),
      "is" => Ok(Language::Icelandic),
      "it" => Ok(Language::Italian),
      "ja" => Ok(Language::Japanese),
      "jw" => Ok(Language::Javanese),
      "km" => Ok(Language::Khmer),
      "kn" => Ok(Language::Kannada),
      "ko" => Ok(Language::Korean),
      "la" => Ok(Language::Latin),
      "lv" => Ok(Language::Latvian),
      "mk" => Ok(Language::Macedonian),
      "ml" => Ok(Language::Malayalam),
      "mr" => Ok(Language::Marathi),
      "my" => Ok(Language::Burmese),
      "ne" => Ok(Language::Nepali),
      "nl" => Ok(Language::Dutch),
      "no" => Ok(Language::Norwegian),
      "pl" => Ok(Language::Polish),
      "pt" => Ok(Language::Portuguese),
      "ro" => Ok(Language::Romanian),
      "ru" => Ok(Language::Russian),
      "si" => Ok(Language::Sinhala),
      "sk" => Ok(Language::Slovak),
      "sq" => Ok(Language::Albanian),
      "sr" => Ok(Language::Serbian),
      "su" => Ok(Language::Sundanese),
      "sv" => Ok(Language::Swedish),
      "sw" => Ok(Language::Swahili),
      "ta" => Ok(Language::Tamil),
      "te" => Ok(Language::Telugu),
      "th" => Ok(Language::Thai),
      "tl" => Ok(Language::Filipino),
      "tr" => Ok(Language::Turkish),
      "uk" => Ok(Language::Ukrainian),
      "ur" => Ok(Language::Urdu),
      "vi" => Ok(Language::Vietnamese),
      "zh-CN" => Ok(Language::Chinese),
      _ => Err(format!(
        "Unknown language: {}. Make sure to use all the supported Language",
        s
      )),
    }
  }
}

impl Language {
  pub fn as_code(l: Language) -> &'static str {
    match l {
      Language::Afrikaans => "af",
      Language::Albanian => "sq",
      Language::Arabic => "ar",
      Language::Armenian => "hy",
      Language::Bengali => "bn",
      Language::Bosnian => "bs",
      Language::Bulgarian => "bg",
      Language::Catalan => "ca",
      Language::Chinese => "zh-CN",
      Language::Croatian => "hr",
      Language::Czech => "cs",
      Language::Danish => "da",
      Language::Dutch => "nl",
      Language::English => "en",
      Language::Esperanto => "eo",
      Language::Estonian => "et",
      Language::Filipino => "tl",
      Language::Finnish => "fi",
      Language::French => "fr",
      Language::German => "de",
      Language::Greek => "el",
      Language::Gujarati => "gu",
      Language::Hindi => "hi",
      Language::Hungarian => "hu",
      Language::Icelandic => "is",
      Language::Indonesian => "id",
      Language::Italian => "it",
      Language::Japanese => "ja",
      Language::Javanese => "jw",
      Language::Kannada => "kn",
      Language::Khmer => "km",
      Language::Korean => "ko",
      Language::Latin => "la",
      Language::Latvian => "lv",
      Language::Macedonian => "mk",
      Language::Marathi => "mr",
      Language::Nepali => "ne",
      Language::Norwegian => "no",
      Language::Polish => "pl",
      Language::Portuguese => "pt",
      Language::Romanian => "ro",
      Language::Russian => "ru",
      Language::Serbian => "sr",
      Language::Sinhala => "si",
      Language::Slovak => "sk",
      Language::Spanish => "es",
      Language::Swahili => "sw",
      Language::Swedish => "sv",
      Language::Tamil => "ta",
      Language::Telugu => "te",
      Language::Thai => "th",
      Language::Turkish => "tr",
      Language::Ukrainian => "uk",
      Language::Urdu => "ur",
      Language::Vietnamese => "vi",
      Language::Welsh => "cy",
      Language::Burmese => "my",
      Language::Malayalam => "ml",
      Language::Sundanese => "su",
    }
  }
}
