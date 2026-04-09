use divkit::generated::divtext::DivText;
use divkit::types::Expression;

#[test]
fn test_div_text_builder() {
    let div_text = DivText::builder()
        .text(Expression::value(String::from("Hello, World!")))
        .build();

    assert_eq!(div_text.text, Expression::value(String::from("Hello, World!")));
    // test that default alpha is applied
    assert_eq!(div_text.alpha, Some(Expression::value(1.0_f64)));
}
