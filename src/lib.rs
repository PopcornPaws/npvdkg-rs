#![allow(non_snake_case)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod hash_to_curve;
pub mod npvdkgrs;
pub mod participant;
pub mod polynomial;
pub mod pvsh;
pub mod share;

const FP_BYTES: usize = 32;
const G2_BYTES: usize = 96; // compressed
