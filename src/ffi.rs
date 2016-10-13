use ::libc;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum Pid
{ PROC_ALL_PIDS = 0x01,
  PROC_PIDTBSDINFO = 0x03,
  PROC_CHILD_PIDS = 0x07,
  MAXCOMLEN = 0x10,
  MAXPATHLEN = 0xFF,
  PROC_PIDPATHINFO_MAXSIZE = 4 * 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Proz
{ EPROC_CTTY = 0x01,
  EPROC_SLEADER = 0x02,
  WMESGLEN = 0x07,
  COMAPT_MAXLOGNAME = 0x0C,
  MAXCHILDS = 100, }

#[cfg(target_os = "macos")]
extern "C"
{ pub fn proc_listchildpids(_ppid: libc::pid_t, _buff: *mut libc::c_void, _size: libc::c_int) -> libc::c_int;
  pub fn proc_pidpath(_pid: libc::c_int, _buff: *mut libc::c_void, _size: libc::uint32_t) -> libc::c_int;
  pub fn proc_pidinfo(_pid: libc::c_int, _flav: libc::c_int, _arg: libc::uint64_t, _buff: *mut libc::c_void, _size: libc::c_int) -> libc::c_int; }
