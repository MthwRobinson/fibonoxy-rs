#[derive(PartialEq, Debug)]
enum Emoji {
    Ox,
    Camel,
    Unknown,
}

fn name_to_emoji(name: &str) -> Emoji {
    match name {
        "ox" => Emoji::Ox,
        "camel" => Emoji::Camel,
        _ => Emoji::Unknown,
    }
}

fn emoji_code(emoji: Emoji) -> String {
    match emoji {
        Emoji::Ox => String::from("\u{1F402}"),
        Emoji::Camel => String::from("\u{1F42A}"),
        _ => String::from("\u{1F937}"),
    }
}

#[cfg(test)]
mod emoji_tests {
    use super::*;

    #[test]
    fn name_to_emoji_works() {
        assert_eq!(name_to_emoji("ox"), Emoji::Ox)
    }
}
