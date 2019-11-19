#![allow(clippy::float_cmp)]
use bytemuck::*;
use wide::*;

#[test]
fn f32x4_new_order() {
  let f = f32x4::new(0.0, 1.0, 2.0, 3.0);
  assert_eq!(f[0], 0.0);
  assert_eq!(f[1], 1.0);
  assert_eq!(f[2], 2.0);
  assert_eq!(f[3], 3.0);
}

#[test]
fn f32x4_array_cast_order() {
  let f = cast::<[f32; 4], f32x4>([0.0, 1.0, 2.0, 3.0]);
  assert_eq!(f[0], 0.0);
  assert_eq!(f[1], 1.0);
  assert_eq!(f[2], 2.0);
  assert_eq!(f[3], 3.0);
}

#[test]
fn f32x4_merge() {
  let f = f32x4::new(0.0, 1.0, 2.0, 3.0);

  let mask = f.cmp_eq(f32x4::ZERO);
  assert!(mask[0].is_nan());
  assert_eq!(mask[1], 0.0);
  assert_eq!(mask[2], 0.0);
  assert_eq!(mask[3], 0.0);

  let combined = mask.merge(f32x4::ONE, f32x4::ZERO);
  assert_eq!(combined[0], 1.0);
  assert_eq!(combined[1], 0.0);
  assert_eq!(combined[2], 0.0);
  assert_eq!(combined[3], 0.0);

  let mask = f.cmp_gt(f32x4::from(1.5));
  assert_eq!(mask[0], 0.0);
  assert_eq!(mask[1], 0.0);
  assert!(mask[2].is_nan());
  assert!(mask[3].is_nan());

  let combined = mask.merge(f32x4::ONE, f32x4::ZERO);
  assert_eq!(combined[0], 0.0);
  assert_eq!(combined[1], 0.0);
  assert_eq!(combined[2], 1.0);
  assert_eq!(combined[3], 1.0);
}

#[test]
fn f32x4_signum() {
  let a = f32x4::new(0.0, 1.0, 2.0, core::f32::INFINITY).signum();
  assert_eq!(a[0], 1.0);
  assert_eq!(a[1], 1.0);
  assert_eq!(a[2], 1.0);
  assert_eq!(a[3], 1.0);

  let b = f32x4::new(-0.0, -1.0, -2.0, core::f32::NEG_INFINITY).signum();
  assert_eq!(b[0], -1.0);
  assert_eq!(b[1], -1.0);
  assert_eq!(b[2], -1.0);
  assert_eq!(b[3], -1.0);

  let b = f32x4::new(core::f32::NAN, -1.0, 2.0, core::f32::NEG_INFINITY).signum();
  assert!(b[0].is_nan(), -1.0);
  assert_eq!(b[1], -1.0);
  assert_eq!(b[2], 1.0);
  assert_eq!(b[3], -1.0);
}

#[test]
fn f32x4_fraction() {
  let a = f32x4::new(3.5, 5.0, 0.1, -1.5).fract();
  assert_eq!(a[0], 0.5);
  assert_eq!(a[1], 0.0);
  assert_eq!(a[2], 0.1);
  assert_eq!(a[3], -0.5);
}
