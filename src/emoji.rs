#[derive(PartialEq)]
pub struct Emoji<'a> {
    name: &'a str,
    code: &'a str,
}

impl Emoji<'_> {
    pub fn emoji_code<'a>(&'a self) -> &'a str {
        &self.code
    }
}

const UNKNOWN: Emoji = Emoji {
    name: "unknown",
    code: "\u{1F937}",
};

const EMOJIS: &'static [&Emoji] = &[
    &Emoji {
        name: "ox",
        code: "\u{1F402}",
    },
    &Emoji {
        name: "camel",
        code: "\u{1F42A}",
    },
    &UNKNOWN,
];

pub fn name_to_emoji(name: &str) -> &Emoji {
    for emoji in EMOJIS {
        if emoji.name == name {
            return emoji;
        }
    }
    &UNKNOWN
}

#[cfg(test)]
mod emoji_tests {
    use super::*;

    // #[test]
    // fn name_to_emoji_works() {
    //     assert_eq!(name_to_emoji("ox"), Emoji::Ox)
    // }
}
