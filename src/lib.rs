#![doc = include_str!("../README.md")]
#![no_std]

#![allow(non_camel_case_types)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(elided_lifetimes_in_paths)]
#![warn(non_ascii_idents)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

#[cfg(feature = "alloc")]
extern crate alloc;

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
    unsafe { core::mem::transmute::<*const (), usize>(self.as_typed_ptr()) }
  }

  #[inline(always)]
  pub fn is_null(self) -> bool {
    self == Self::NULL
  }

  #[inline(always)]
  pub unsafe fn get<T>(self) -> T {
    unsafe { core::ptr::read(self.as_typed_ptr()) }
  }

  #[inline(always)]
  pub unsafe fn set<T>(self, value: T) {
    unsafe { core::ptr::write(self.as_typed_mut_ptr(), value) }
  }

  #[inline(always)]
  pub unsafe fn replace<T>(self, value: T) -> T {
    unsafe { core::ptr::replace(self.as_typed_mut_ptr(), value) }
  }

  #[inline(always)]
  pub unsafe fn copy_nonoverlapping<T>(src: Self, dst: Self, count: usize) {
    let src = src.as_typed_ptr::<T>();
    let dst = dst.as_typed_mut_ptr::<T>();
    unsafe { core::ptr::copy_nonoverlapping(src, dst, count) };
  }

  #[inline(always)]
  pub unsafe fn swap_nonoverlapping<T>(x: Self, y: Self, count: usize) {
    let x = x.as_typed_mut_ptr::<T>();
    let y = y.as_typed_mut_ptr::<T>();
    unsafe { core::ptr::swap_nonoverlapping(x, y, count) };
  }

  #[inline(always)]
  pub unsafe fn drop_in_place<T>(self) {
    unsafe { core::ptr::drop_in_place::<T>(self.as_typed_mut_ptr()) }
  }

  #[inline(always)]
  pub const fn as_typed_ptr<T>(self) -> *const T {
    self.0 as *const T
  }

  #[inline(always)]
  pub const fn as_typed_mut_ptr<T>(self) -> *mut T {
    self.as_typed_ptr::<T>() as *mut T
  }

  #[inline(always)]
  pub const fn as_slice_ptr<T>(self, len: usize) -> *const [T] {
    core::ptr::slice_from_raw_parts(self.as_typed_ptr(), len)
  }

  #[inline(always)]
  pub const fn as_slice_mut_ptr<T>(self, len: usize) -> *mut [T] {
    self.as_slice_ptr::<T>(len) as *mut [T]
  }

  #[inline(always)]
  pub unsafe fn as_ref<'a, T>(self) -> &'a T {
    unsafe { &*self.as_typed_ptr() }
  }

  #[inline(always)]
  pub unsafe fn as_mut_ref<'a, T>(self) -> &'a mut T {
    unsafe { &mut *self.as_typed_mut_ptr() }
  }

  #[inline(always)]
  pub unsafe fn as_slice_ref<'a, T>(self, len: usize) -> &'a [T] {
    unsafe { &*self.as_slice_ptr(len) }
  }

  #[inline(always)]
  pub unsafe fn as_slice_mut_ref<'a, T>(self, len: usize) -> &'a mut [T] {
    unsafe { &mut *self.as_slice_mut_ptr(len) }
  }

  #[cfg(feature = "alloc")]
  #[inline(always)]
  pub unsafe fn alloc(layout: alloc::alloc::Layout) -> Self {
    unsafe { Self::new(alloc::alloc::alloc(layout)) }
  }

  #[cfg(feature = "alloc")]
  #[inline(always)]
  pub unsafe fn alloc_zeroed(layout: alloc::alloc::Layout) -> Self {
    unsafe { Self::new(alloc::alloc::alloc_zeroed(layout)) }
  }

  #[cfg(feature = "alloc")]
  #[inline(always)]
  pub unsafe fn dealloc(x: Self, layout: alloc::alloc::Layout) {
    unsafe { alloc::alloc::dealloc(x.as_typed_mut_ptr(), layout) }
  }
}

impl core::ops::Add<usize> for ptr {
  type Output = Self;

  #[inline(always)]
  fn add(self, rhs: usize) -> Self::Output {
    Self::new(self.as_typed_ptr::<u8>().wrapping_add(rhs))
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
    Self::new(self.as_typed_ptr::<u8>().wrapping_sub(rhs))
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
