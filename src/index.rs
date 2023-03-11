use rstest::*;

#[fixture]
pub fn fixture() -> u32 {
    42
}

#[rstest]
fn should_success(fixture: u32) {
    assert_eq!(fixture, 42);
}

#[rstest]
fn metadata() {
    use crate::print_metadata;

    print_metadata();
}
