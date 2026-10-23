use crate::{Language, Notice, Phrase};

#[test]
fn test_a_notice_puts_the_detail_after_its_phrase() {
    let notice = Notice {
        phrase: Phrase {
            english: "The Mac refused the connection:",
            russian: "Mac отказал в подключении:",
            ukrainian: "Mac відмовив у підключенні:",
        },
        detail: Some("too many viewers".to_owned()),
    };
    assert_eq!(
        notice.text(Language::English),
        "The Mac refused the connection: too many viewers",
        "the reason the Mac gives follows the words of the client"
    );
}
