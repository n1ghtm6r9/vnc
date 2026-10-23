use crate::{Language, Phrase};

#[test]
fn test_a_phrase_reads_in_the_chosen_language() {
    let phrase = Phrase { english: "Connect", russian: "Подключиться", ukrainian: "Підключитися" };
    for (language, expected) in
        [(Language::English, "Connect"), (Language::Russian, "Подключиться"), (Language::Ukrainian, "Підключитися")]
    {
        assert_eq!(phrase.text(language), expected, "the {language:?} interface shows the {language:?} words");
    }
}
