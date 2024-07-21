use super::*;

pub fn list_emojis() -> impl Iterator<Item = &'static Emoji> {
    emojis::iter().filter(|e| e.unicode_version() < emojis::UnicodeVersion::new(13, 0))
}
