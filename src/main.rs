extern crate libc;

mod ffi;
mod prozess_macos;
mod convert;

mod proc_bsdinfo;
use proc_bsdinfo::{ProcBsdInfo};

mod baum;
use baum::{Baum, BaumBenutz};

fn main()
{ unsafe
  { println!("size::{}", std::mem::size_of::<ProcBsdInfo>());
    let _x = Baum::default();
    _x.anzeigt();
    println!("Getpid::{}", libc::getpid()); }}
