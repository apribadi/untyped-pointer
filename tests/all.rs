use expect_test::expect;
use ptr_type::*;

#[test]
fn test_debug() {
  expect!["0x000000000000002a"].assert_eq(&format!("{:?}", ptr::invalid(42)));
}
