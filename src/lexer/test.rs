#[test]
fn test_valid_continuation() {
    assert!(super::identifier::valid_continuation('a'));
    assert!(super::identifier::valid_continuation('Z'));
    assert!(super::identifier::valid_continuation('_'));
    assert!(super::identifier::valid_continuation('0'));

    assert!(!super::identifier::valid_continuation('-'));
    assert!(!super::identifier::valid_continuation('&'));
    assert!(!super::identifier::valid_continuation('.'));
    assert!(!super::identifier::valid_continuation(' '));
    assert!(!super::identifier::valid_continuation('\n'));
}

#[test]
fn test_valid_start() {
    assert!(super::identifier::valid_start('a'));
    assert!(super::identifier::valid_start('Z'));
    assert!(super::identifier::valid_start('_'));

    assert!(!super::identifier::valid_start('0'));

    assert!(!super::identifier::valid_start('-'));
    assert!(!super::identifier::valid_start('&'));
    assert!(!super::identifier::valid_start('.'));
    assert!(!super::identifier::valid_start(' '));
    assert!(!super::identifier::valid_start('\n'));
}
