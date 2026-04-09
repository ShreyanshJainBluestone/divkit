use divkit::generated::divtext::DivText;
use divkit::types::Expression;
use serde_json::json;

#[test]
fn test_json_serialization_matches() {
    let div_text = DivText::builder()
        .text(Expression::value(String::from("Hello, DivKit!")))
        .build();

    let expected_json = json!({
        "type": "text",
        "text": "Hello, DivKit!",
        "alpha": 1.0,
        "capture_focus_on_action": true,
        "font_size": 12,
        "font_size_unit": "sp",
        "height": {
            "type": "wrap_content"
        },
        "letter_spacing": 0.0,
        "selectable": true,
        "strike": "none",
        "text_alignment_horizontal": "start",
        "text_alignment_vertical": "top",
        "text_color": "#FF000000",
        "truncate": "end",
        "underline": "none",
        "width": {
            "type": "match_parent"
        },
        "tighten_width": true,
        "visibility": "visible",
        "action_animation": {
            "duration": 100,
            "end_value": 0.6,
            "name": "fade",
            "start_value": 1.0
        }
    });

    let serialized = serde_json::to_value(&div_text).unwrap();
    
    // Assert exactly
    assert_eq!(serialized, expected_json, "Rust div-text output must match exact DivKit JSON specification (including generated defaults)");
}
