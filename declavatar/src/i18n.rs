const LOG_MESSAGES_EN_US: &str = include_str!("../i18n/log.en-us.json");
const LOG_MESSAGES_JA_JP: &str = include_str!("../i18n/log.ja-jp.json");

pub fn get_log_messages(locale: &str) -> Option<&'static str> {
    let canonical_locale = locale.to_lowercase().replace('_', "-");
    let json = match &canonical_locale[..] {
        "en-us" => LOG_MESSAGES_EN_US,
        "ja-jp" => LOG_MESSAGES_JA_JP,
        _ => return None,
    };
    Some(json)
}
