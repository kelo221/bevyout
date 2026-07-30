use super::*;

#[test]
fn master_slots_resolve_through_the_global_load_order() {
    let resolver = FormIdResolver::new(7, vec![1, 4]);
    assert_eq!(resolver.adjust(0x0000_1234), 0x0100_1234);
    assert_eq!(resolver.adjust(0x0100_1234), 0x0400_1234);
}

#[test]
fn current_and_unknown_slots_preserve_the_existing_fallback() {
    let resolver = FormIdResolver::new(7, vec![1, 4]);
    assert_eq!(resolver.adjust(0x0200_1234), 0x0700_1234);
    assert_eq!(resolver.adjust(0xff00_1234), 0x0700_1234);
}

#[test]
fn display_is_stable_lowercase_eight_digit_hex() {
    assert_eq!(FormId(0x00ab_cdef).to_string(), "00abcdef");
}
