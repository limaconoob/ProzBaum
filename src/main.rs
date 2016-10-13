extern crate libc;

mod ffi;
mod process_macos;
mod convert;

mod proc_bsdinfo;
use proc_bsdinfo::{ProcBsdInfo};

mod baum;
use baum::{Baum, BaumBenutz, current_pid};

fn main()
{ println!("size::{}", std::mem::size_of::<ProcBsdInfo>());
  let _x = Baum::default();
  _x.anzeigt();
  println!("Cur_pid::{}", current_pid()); }
