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
pub struct ptr(*const u8);

impl ptr {
  pub const NULL: Self = Self::invalid(0);

  #[inline(always)]
  pub const fn new<T: ?Sized>(p: *const T) -> Self {
    Self(p as *const u8)
  }

  #[inline(always)]
  pub const fn invalid(p: usize) -> Self {
    Self::new(unsafe { core::mem::transmute::<usize, *const u8>(p) })
  }

  #[inline(always)]
  pub fn addr(self) -> usize {
    // NB: not const
    unsafe { core::mem::transmute::<*const u8, usize>(self.as_ptr()) }
  }

  #[inline(always)]
  pub fn is_null(self) -> bool {
    self == Self::NULL
  }

  #[inline(always)]
  pub const fn add(self, offset: usize) -> Self {
    Self::new(self.as_mut_ptr::<u8>().wrapping_add(offset))
  }

  #[inline(always)]
  pub const fn sub(self, offset: usize) -> Self {
    Self::new(self.as_mut_ptr::<u8>().wrapping_sub(offset))
  }

  #[inline(always)]
  pub fn diff(self, other: Self) -> usize {
    self.addr().wrapping_sub(other.addr())
  }

  #[inline(always)]
  pub const fn strided_add<T>(self, offset: usize) -> Self {
    self.add(core::mem::size_of::<T>().wrapping_mul(offset))
  }

  #[inline(always)]
  pub const fn strided_sub<T>(self, offset: usize) -> Self {
    self.sub(core::mem::size_of::<T>().wrapping_mul(offset))
  }

  #[inline(always)]
  pub fn strided_diff<T>(self, other: Self) -> usize {
    self.diff(other) / core::mem::size_of::<T>()
  }

  #[inline(always)]
  pub fn mask(self, mask: usize) -> Self {
    self.sub(self.addr() & ! mask)
  }

  #[inline(always)]
  pub unsafe fn get<T>(self) -> T {
    unsafe { core::ptr::read(self.as_ptr()) }
  }

  #[inline(always)]
  pub unsafe fn set<T>(self, value: T) {
    unsafe { core::ptr::write(self.as_mut_ptr(), value) }
  }

  #[inline(always)]
  pub unsafe fn replace<T>(self, value: T) -> T {
    unsafe { core::ptr::replace(self.as_mut_ptr(), value) }
  }

  #[inline(always)]
  pub unsafe fn drop_in_place<T>(self) {
    unsafe { core::ptr::drop_in_place::<T>(self.as_mut_ptr()) }
  }

  #[inline(always)]
  pub const fn as_ptr<T>(self) -> *const T {
    self.0 as *const T
  }

  #[inline(always)]
  pub const fn as_mut_ptr<T>(self) -> *mut T {
    self.as_ptr::<T>() as *mut T
  }

  #[inline(always)]
  pub const fn as_slice_ptr<T>(self, len: usize) -> *const [T] {
    core::ptr::slice_from_raw_parts(self.as_ptr(), len)
  }

  #[inline(always)]
  pub const fn as_slice_mut_ptr<T>(self, len: usize) -> *mut [T] {
    self.as_slice_ptr::<T>(len) as *mut [T]
  }

  #[inline(always)]
  pub unsafe fn as_ref<'a, T>(self) -> &'a T {
    unsafe { &*self.as_ptr() }
  }

  #[inline(always)]
  pub unsafe fn as_mut_ref<'a, T>(self) -> &'a mut T {
    unsafe { &mut *self.as_mut_ptr() }
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
  pub unsafe fn alloc(self, layout: alloc::alloc::Layout) -> Self {
    unsafe { Self::new(alloc::alloc::alloc(layout)) }
  }

  #[cfg(feature = "alloc")]
  #[inline(always)]
  pub unsafe fn alloc_zeroed(self, layout: alloc::alloc::Layout) -> Self {
    unsafe { Self::new(alloc::alloc::alloc_zeroed(layout)) }
  }

  #[cfg(feature = "alloc")]
  #[inline(always)]
  pub unsafe fn dealloc(self, layout: alloc::alloc::Layout) {
    unsafe { alloc::alloc::dealloc(self.as_mut_ptr(), layout) }
  }
}

impl core::ops::Add<usize> for ptr {
  type Output = Self;

  #[inline(always)]
  fn add(self, rhs: usize) -> Self::Output {
    self.add(rhs)
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
    self.sub(rhs)
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
    self.diff(rhs)
  }
}

impl core::ops::BitAnd<usize> for ptr {
  type Output = Self;

  #[inline(always)]
  fn bitand(self, rhs: usize) -> Self::Output {
    self.mask(rhs)
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
