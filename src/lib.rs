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

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ptr(*const ());

unsafe impl Send for ptr {}

unsafe impl Sync for ptr {}

impl ptr {
  pub const NULL: Self = Self::invalid(0);

  #[inline(always)]
  pub const fn invalid(addr: usize) -> Self {
    Self(unsafe { core::mem::transmute::<usize, *const ()>(addr) })
  }

  #[inline(always)]
  pub fn addr(self) -> usize {
    // NB: not `pub const fn addr`
    unsafe { core::mem::transmute::<*const (), usize>(self.0) }
  }

  #[inline(always)]
  pub fn from_const_ptr<T: ?Sized>(p: *const T) -> Self {
    Self(p as *const ())
  }

  #[inline(always)]
  pub fn from_mut_ptr<T: ?Sized>(p: *mut T) -> Self {
    Self(p as *const T as *const ())
  }

  #[inline(always)]
  pub fn from_ref<T: ?Sized>(p: &T) -> Self {
    Self::from_const_ptr(p)
  }

  #[inline(always)]
  pub fn from_mut_ref<T: ?Sized>(p: &mut T) -> Self {
    Self::from_mut_ptr(p)
  }

  #[inline(always)]
  pub fn from_non_null<T: ?Sized>(p: core::ptr::NonNull<T>) -> Self {
    Self::from_mut_ptr(p.as_ptr())
  }

  #[inline(always)]
  pub fn is_null(self) -> bool {
    self.addr() == 0
  }

  #[inline(always)]
  pub const fn add(self, offset: usize) -> Self {
    Self((self.0 as *const u8).wrapping_add(offset) as *const ())
  }

  #[inline(always)]
  pub const fn sub(self, offset: usize) -> Self {
    Self((self.0 as *const u8).wrapping_sub(offset) as *const ())
  }

  #[inline(always)]
  pub fn diff(self, base: Self) -> usize {
    self.addr().wrapping_sub(base.addr())
  }

  #[inline(always)]
  pub fn mask(self, mask: usize) -> Self {
    self.sub(self.addr() & ! mask)
  }

  #[inline(always)]
  pub unsafe fn read<T>(p: Self) -> T {
    let p = p.as_const_ptr();
    unsafe { core::ptr::read(p) }
  }

  #[inline(always)]
  pub unsafe fn read_unaligned<T>(p: Self) -> T {
    let p = p.as_const_ptr();
    unsafe { core::ptr::read_unaligned(p) }
  }

  #[inline(always)]
  pub unsafe fn write<T>(p: Self, value: T) {
    let p = p.as_mut_ptr();
    unsafe { core::ptr::write(p, value) }
  }

  #[inline(always)]
  pub unsafe fn write_unaligned<T>(p: Self, value: T) {
    let p = p.as_mut_ptr();
    unsafe { core::ptr::write_unaligned(p, value) }
  }

  #[inline(always)]
  pub unsafe fn replace<T>(p: Self, value: T) -> T {
    let p = p.as_mut_ptr();
    unsafe { core::ptr::replace(p, value) }
  }

  #[inline(always)]
  pub unsafe fn copy_nonoverlapping<T>(src: Self, dst: Self, count: usize) {
    let src = src.as_const_ptr();
    let dst = dst.as_mut_ptr();
    unsafe { core::ptr::copy_nonoverlapping::<T>(src, dst, count) };
  }

  #[inline(always)]
  pub unsafe fn swap_nonoverlapping<T>(p: Self, q: Self, count: usize) {
    let p = p.as_mut_ptr();
    let q = q.as_mut_ptr();
    unsafe { core::ptr::swap_nonoverlapping::<T>(p, q, count) };
  }

  #[inline(always)]
  pub unsafe fn drop_in_place<T>(p: Self) {
    let p = p.as_mut_ptr();
    unsafe { core::ptr::drop_in_place::<T>(p) }
  }

  #[inline(always)]
  pub const fn as_const_ptr<T>(self) -> *const T {
    self.0 as *const T
  }

  #[inline(always)]
  pub const fn as_mut_ptr<T>(self) -> *mut T {
    self.0 as *const T as *mut T
  }

  #[inline(always)]
  pub const fn as_slice_const_ptr<T>(self, len: usize) -> *const [T] {
    core::ptr::slice_from_raw_parts(self.as_const_ptr(), len)
  }

  #[inline(always)]
  pub const fn as_slice_mut_ptr<T>(self, len: usize) -> *mut [T] {
    self.as_slice_const_ptr::<T>(len) as *mut [T]
  }

  #[inline(always)]
  pub const unsafe fn as_ref<'a, T>(self) -> &'a T {
    let x = self.as_const_ptr();
    unsafe { &*x }
  }

  #[inline(always)]
  pub unsafe fn as_mut_ref<'a, T>(self) -> &'a mut T {
    let x = self.as_mut_ptr();
    unsafe { &mut *x }
  }

  #[inline(always)]
  pub const unsafe fn as_slice_ref<'a, T>(self, len: usize) -> &'a [T] {
    let x = self.as_slice_const_ptr(len);
    unsafe { &*x }
  }

  #[inline(always)]
  pub unsafe fn as_slice_mut_ref<'a, T>(self, len: usize) -> &'a mut [T] {
    let x = self.as_slice_mut_ptr(len);
    unsafe { &mut *x }
  }

  #[inline(always)]
  pub fn as_non_null<T>(self) -> Option<core::ptr::NonNull<T>> {
    core::ptr::NonNull::new(self.as_mut_ptr())
  }
}

impl<T: ?Sized> From<*const T> for ptr {
  #[inline(always)]
  fn from(value: *const T) -> Self {
    Self::from_const_ptr(value)
  }
}

impl<T: ?Sized> From<*mut T> for ptr {
  #[inline(always)]
  fn from(value: *mut T) -> Self {
    Self::from_mut_ptr(value)
  }
}

impl<T: ?Sized> From<&T> for ptr {
  #[inline(always)]
  fn from(value: &T) -> Self {
    Self::from_ref(value)
  }
}

impl<T: ?Sized> From<&mut T> for ptr {
  #[inline(always)]
  fn from(value: &mut T) -> Self {
    Self::from_mut_ref(value)
  }
}

impl<T: ?Sized> From<core::ptr::NonNull<T>> for ptr {
  #[inline(always)]
  fn from(value: core::ptr::NonNull<T>) -> Self {
    Self::from_non_null(value)
  }
}

impl<T> From<ptr> for *const T {
  #[inline(always)]
  fn from(value: ptr) -> *const T {
    value.as_const_ptr()
  }
}

impl<T> From<ptr> for *mut T {
  #[inline(always)]
  fn from(value: ptr) -> *mut T {
    value.as_mut_ptr()
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
