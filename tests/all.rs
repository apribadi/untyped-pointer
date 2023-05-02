use expect_test::expect;
use untyped_pointer::ptr;

#[test]
fn test_special_traits() {
  fn is_clone<T: Clone>() {}
  fn is_copy<T: Copy>() {}
  fn is_ref_unwind_safe<T: core::panic::RefUnwindSafe>() {}
  fn is_send<T: Send>() {}
  fn is_sync<T: Sync>() {}
  fn is_unpin<T: Unpin>() {}
  fn is_unwind_safe<T: core::panic::UnwindSafe>() {}

  is_clone::<ptr>();
  is_copy::<ptr>();
  is_ref_unwind_safe::<ptr>();
  is_send::<ptr>();
  is_sync::<ptr>();
  is_unpin::<ptr>();
  is_unwind_safe::<ptr>();
}

#[test]
fn test_debug() {
  expect!["0x000000000000002a"].assert_eq(&format!("{:?}", ptr::invalid(42)));
}
