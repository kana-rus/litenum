#[litenum::ium]
#[derive(Debug, PartialEq)]
enum AnkerTarget {
    _blank,
    _self,
    _top,
    _parent,
}

fn main() {
    let a_tag = format!(r#"<a href="{}" target="{}">click me!</a>"#,
        "https://example.com",
        AnkerTarget::_blank.lit(),
    );
    assert_eq!(
        a_tag,
        r#"<a href="https://example.com" target="_blank">click me!</a>"#
    );

    let target = a_tag
        .trim_start_matches("<a ")
        .split_once('>').unwrap().0
        .split_whitespace()
        .find(|tokens| tokens.starts_with("target=")).unwrap()
        .strip_prefix("target=").unwrap()
        .trim_matches('"');
    assert_eq!(
        AnkerTarget::from_lit(target),
        Some(AnkerTarget::_blank)
    );
}
