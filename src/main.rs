extern crate libc;

mod ffi;
use ffi::proc_bsdinfo;

mod macos;
mod convert;

mod event;
use event::{Proc};

mod baum;
use baum::{Baum, BaumBenutz, current_pid};

fn main()
{ let _x = Baum::default();
  _x.anzeigt(); }
