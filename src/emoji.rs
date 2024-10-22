#[derive(PartialEq)]
pub struct Emoji<'a> {
    name: &'a str,
    code: &'a str,
}

const OX: Emoji = Emoji {
    name: "ox",
    code: "\u{1F402}",
};
const CAMEL: Emoji = Emoji {
    name: "camel",
    code: "\u{1F42A}",
};
const UNKNOWN: Emoji = Emoji {
    name: "unknown",
    code: "\u{1F937}",
};

pub fn name_to_emoji(name: &str) -> &Emoji {
    match name {
        "ox" => &OX,
        "camel" => &CAMEL,
        _ => &UNKNOWN,
    }
}

pub fn emoji_code<'a>(emoji: &'a Emoji) -> &'a str {
    &emoji.code
}

#[cfg(test)]
mod emoji_tests {
    use super::*;

    // #[test]
    // fn name_to_emoji_works() {
    //     assert_eq!(name_to_emoji("ox"), Emoji::Ox)
    // }
}
