use std::mem;
use std::slice;
use std::ffi::CStr;

use super::ffi;

use ::libc;

#[inline]
pub unsafe fn proc_get_info(pid: libc::c_int) {
  let len: libc::size_t = mem::size_of::<ffi::proc_bsdinfo>();
  let buffersize: libc::c_int = len as libc::c_int;
  let pathbuf: [libc::c_uchar; ffi::PROC_PIDPATHINFO_MAXSIZE_U + 1] = mem::zeroed();
  let mut buffer: ffi::proc_bsdinfo = mem::zeroed();
 
 print!("rust::proc_pidinfo({}, {}, {}, {}, {})", mem::size_of::<libc::c_int>(), mem::size_of::<libc::c_int>(), mem::size_of::<libc::uint64_t>(), mem::size_of::<*mut ffi::proc_bsdinfo>(), mem::size_of::<libc::c_int>());
  print!("rust::proc_pidinfo({}, {}, {}, {:p}, {})",
         pid, ffi::PROC_PIDTBSDINFO, 0, &mut buffer, buffersize
        );
  let st: libc::c_int = ffi::proc_pidinfo (
         pid, ffi::PROC_PIDTBSDINFO, 0, &mut buffer, buffersize
        );
  println!(" -> {}", st);
print!("rust::proc_pidpath({}, {}, {})", mem::size_of::<libc::c_int>(), mem::size_of::<*const libc::c_char>(), mem::size_of::<libc::uint64_t>() );
  print!("rust::proc_pidpath({}, {:p}, {})",
         pid, pathbuf.as_ptr(), ffi::PROC_PIDPATHINFO_MAXSIZE
        );
  let ss: libc::c_int = ffi::proc_pidpath (
      pid,
      pathbuf.as_ptr(),
      ffi::PROC_PIDPATHINFO_MAXSIZE,
  );
  println!(" -> {}", ss);
  println!("{}", CStr::from_bytes_with_nul_unchecked(&pathbuf[..]).to_string_lossy());


  println!("pid::{}", pid);
  let buffer: *mut ffi::proc_bsdinfo = &mut buffer;
  let buffer: &[libc::c_uchar] = slice::from_raw_parts_mut(buffer as *mut libc::c_uchar, len);
  (0..len).all(|index: libc::size_t| {
      print!("{} ", buffer.get_unchecked(index));
      true
  });
}
