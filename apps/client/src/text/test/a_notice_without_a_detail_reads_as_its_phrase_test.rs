use crate::{Language, Notice, Phrase};

#[test]
fn test_a_notice_without_a_detail_reads_as_its_phrase() {
    let phrase = Phrase { english: "Wrong password.", russian: "Неверный пароль.", ukrainian: "Неправильний пароль." };
    assert_eq!(
        Notice::from(phrase).text(Language::Ukrainian),
        "Неправильний пароль.",
        "a notice with nothing to add is just its phrase"
    );
}
