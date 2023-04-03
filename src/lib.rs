#![doc = include_str!("../README.md")]
#![no_std]

#![allow(non_camel_case_types)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(elided_lifetimes_in_paths)]
#![warn(non_ascii_idents)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[inline(always)]
fn convert<T, U: From<T>>(value: T) -> U {
  U::from(value)
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ptr(*const ());

impl ptr {
  pub const NULL: Self = Self::invalid(0);

  #[inline(always)]
  pub const fn new<T: ?Sized>(x: *const T) -> Self {
    Self(x as *const ())
  }

  #[inline(always)]
  pub const fn invalid(addr: usize) -> Self {
    Self::new(unsafe { core::mem::transmute::<usize, *const ()>(addr) })
  }

  #[inline(always)]
  pub fn addr(self) -> usize {
    // NB: not const
    let x = convert(self);
    unsafe { core::mem::transmute::<*const (), usize>(x) }
  }

  #[inline(always)]
  pub fn is_null(self) -> bool {
    self == Self::NULL
  }

  #[inline(always)]
  pub unsafe fn get<T>(self) -> T {
    let x = convert(self);
    unsafe { core::ptr::read(x) }
  }

  #[inline(always)]
  pub unsafe fn set<T>(self, value: T) {
    let x = convert(self);
    unsafe { core::ptr::write(x, value) }
  }

  #[inline(always)]
  pub unsafe fn replace<T>(self, value: T) -> T {
    let x = convert(self);
    unsafe { core::ptr::replace(x, value) }
  }

  #[inline(always)]
  pub unsafe fn copy_nonoverlapping<T>(src: Self, dst: Self, count: usize) {
    let src = convert(src);
    let dst = convert(dst);
    unsafe { core::ptr::copy_nonoverlapping::<T>(src, dst, count) };
  }

  #[inline(always)]
  pub unsafe fn swap_nonoverlapping<T>(x: Self, y: Self, count: usize) {
    let x = convert(x);
    let y = convert(y);
    unsafe { core::ptr::swap_nonoverlapping::<T>(x, y, count) };
  }

  #[inline(always)]
  pub unsafe fn drop_in_place<T>(self) {
    let x = convert(self);
    unsafe { core::ptr::drop_in_place::<T>(x) }
  }

  #[inline(always)]
  pub unsafe fn as_ref<'a, T>(self) -> &'a T {
    let x = convert::<_, *const T>(self);
    unsafe { &*x }
  }

  #[inline(always)]
  pub unsafe fn as_mut_ref<'a, T>(self) -> &'a mut T {
    let x = convert::<_, *mut T>(self);
    unsafe { &mut *x }
  }

  #[inline(always)]
  pub unsafe fn as_slice_ref<'a, T>(self, len: usize) -> &'a [T] {
    let x = convert(self);
    let x = core::ptr::slice_from_raw_parts(x, len);
    unsafe { &*x }
  }

  #[inline(always)]
  pub unsafe fn as_slice_mut_ref<'a, T>(self, len: usize) -> &'a mut [T] {
    let x = convert(self);
    let x = core::ptr::slice_from_raw_parts_mut(x, len);
    unsafe { &mut *x }
  }

  #[cfg(feature = "alloc")]
  #[inline(always)]
  pub unsafe fn alloc(layout: alloc::alloc::Layout) -> Self {
    Self::new(unsafe { alloc::alloc::alloc(layout) })
  }

  #[cfg(feature = "alloc")]
  #[inline(always)]
  pub unsafe fn alloc_zeroed(layout: alloc::alloc::Layout) -> Self {
    Self::new(unsafe { alloc::alloc::alloc_zeroed(layout) })
  }

  #[cfg(feature = "alloc")]
  #[inline(always)]
  pub unsafe fn dealloc(x: Self, layout: alloc::alloc::Layout) {
    let x = convert(x);
    unsafe { alloc::alloc::dealloc(x, layout) }
  }
}

impl<T: ?Sized> From<*const T> for ptr {
  #[inline(always)]
  fn from(value: *const T) -> Self {
    Self::new(value)
  }
}

impl<T: ?Sized> From<*mut T> for ptr {
  #[inline(always)]
  fn from(value: *mut T) -> Self {
    Self::new(value)
  }
}

impl<T: ?Sized> From<&T> for ptr {
  #[inline(always)]
  fn from(value: &T) -> Self {
    Self::new(value)
  }
}

impl<T: ?Sized> From<&mut T> for ptr {
  #[inline(always)]
  fn from(value: &mut T) -> Self {
    Self::new(value)
  }
}

impl<T: ?Sized> From<core::ptr::NonNull<T>> for ptr {
  #[inline(always)]
  fn from(value: core::ptr::NonNull<T>) -> Self {
    Self::new(value.as_ptr())
  }
}

impl<T> From<ptr> for *const T {
  #[inline(always)]
  fn from(value: ptr) -> *const T {
    value.0 as *const T
  }
}

impl<T> From<ptr> for *mut T {
  #[inline(always)]
  fn from(value: ptr) -> *mut T {
    value.0 as *mut T
  }
}

impl core::ops::Add<usize> for ptr {
  type Output = Self;

  #[inline(always)]
  fn add(self, rhs: usize) -> Self::Output {
    Self::new(convert::<_, *const u8>(self).wrapping_add(rhs))
  }
}

impl core::ops::AddAssign<usize> for ptr {
  #[inline(always)]
  fn add_assign(&mut self, rhs: usize) {
    *self = *self + rhs;
  }
}

impl core::ops::Sub<usize> for ptr {
  type Output = Self;

  #[inline(always)]
  fn sub(self, rhs: usize) -> Self::Output {
    Self::new(convert::<_, *const u8>(self).wrapping_sub(rhs))
  }
}

impl core::ops::SubAssign<usize> for ptr {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: usize) {
    *self = *self - rhs;
  }
}

impl core::ops::Sub<ptr> for ptr {
  type Output = usize;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    self.addr().wrapping_sub(rhs.addr())
  }
}

impl core::ops::BitAnd<usize> for ptr {
  type Output = Self;

  #[inline(always)]
  fn bitand(self, rhs: usize) -> Self::Output {
    self - (self.addr() & ! rhs)
  }
}

impl core::ops::BitAndAssign<usize> for ptr {
  #[inline(always)]
  fn bitand_assign(&mut self, rhs: usize) {
    *self = *self & rhs;
  }
}

impl core::fmt::Debug for ptr {
  fn fmt(&self, out: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(out, "0x{:01$x}", self.addr(), usize::BITS as usize / 4)
  }
}
