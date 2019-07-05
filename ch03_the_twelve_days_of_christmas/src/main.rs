mod lyrics;

fn make_lyrics() -> String {
    let gifts = [
        "Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Waiting",
        "10 Lords a Leaping",
        "'leven Pipers Piping",
        "12 Drummers Drumming",
    ];

    let mut lyrics = String::new();
    for day in 0..gifts.len() {
        lyrics.push_str(&format!(
            "On the {} day of Christmas\nMy true love gave to me:\n",
            day + 1
        ));

        for gift_index in (0..=day).rev() {
            let gift = if gift_index == 0 {
                if day == 0 {
                    format!("A {}", gifts[gift_index])
                } else {
                    format!("And a {}", gifts[gift_index])
                }
            } else {
                String::from(gifts[gift_index])
            };
            lyrics.push_str(&gift);
            lyrics.push_str("\n");
        }
        lyrics.push_str("\n");
    }

    // TODO: fix the second allocation.
    String::from(lyrics.trim_end())
}

fn main() {
    println!("{}", make_lyrics());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_proper_lyrics() {
        assert_eq!(make_lyrics(), lyrics::LYRICS);
    }
}
