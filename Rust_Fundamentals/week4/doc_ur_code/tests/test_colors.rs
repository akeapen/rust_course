use doc_ur_code::colors::{ColorString, Color};

#[test]
fn test_red_color_string() {
    let mut color_string = ColorString{
        color: Color::Red,
        string: "Red".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[31mRed\x1b[0m");
}

#[test]
fn test_green_color_string() {
    let mut color_string = ColorString{
        color: Color::Green,
        string: "Green".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[32mGreen\x1b[0m");
}