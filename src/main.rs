extern crate libc;

mod ffi;
mod prozess_macos;
mod convert;

mod proc_bsdinfo;
use proc_bsdinfo::{ProcBsdInfo};

mod baum;
use baum::{Baum, baum_printer};

fn main()
{ unsafe
  { println!("size::{}", std::mem::size_of::<ProcBsdInfo>());
    let _x = Baum::default();
    baum_printer(_x);
    println!("Getpid::{}", libc::getpid()); }}
