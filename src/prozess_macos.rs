
extern crate std;

use libc;
use ffi::{Pid, Proz};
use ffi::{proc_pidpath, proc_listchildpids, proc_pidinfo};
use convert::{u8_to_string};
use proc_bsdinfo::{ProcBsdInfo};

//char pathbuf[PROC_PIDPATHINFO_MAXSIZE];
//proc_pidpath(pid, pathbuf, sizeof(pathbuf));

pub fn proc_get_info(pid: libc::pid_t) -> Option<(ProcBsdInfo, String)>
{ unsafe
  { let size = std::mem::size_of::<ProcBsdInfo>();
    let mut proz: Vec<u8> = Vec::with_capacity(size);
    let mut pathbuf: Vec<u8> = Vec::with_capacity(Pid::PROC_PIDPATHINFO_MAXSIZE as usize);
    let bonjour = proz.as_ptr() as *mut libc::c_void;
    let ret = proc_pidinfo(pid, Pid::PROC_PIDTBSDINFO as i32, 0, bonjour, size as i32);
    let coucou = pathbuf.as_ptr() as *mut libc::c_void;
    let tek = proc_pidpath(pid, coucou, Pid::PROC_PIDPATHINFO_MAXSIZE as u32);
    if ret > 0
    { proz = Vec::from_raw_parts(bonjour as *mut u8, size, proz.capacity());
      pathbuf = Vec::from_raw_parts(coucou as *mut u8, Pid::PROC_PIDPATHINFO_MAXSIZE as usize, pathbuf.capacity());
      return Some((ProcBsdInfo::new(proz), u8_to_string(&pathbuf, pathbuf.capacity() as usize))) }
    else
    { None }}}

fn vec_pids(bonjour: *mut i32, connard: usize, the_vec: Vec<libc::pid_t>) -> Vec<libc::pid_t>
{ unsafe
  { let mut pids = Vec::from_raw_parts(bonjour, connard, the_vec.capacity());
    pids.retain(|&p| p > 0i32);
    pids.reverse();
    pids }}

pub fn proc_get_pids<'a>(flag: Pid, pid: libc::pid_t) -> Option<Vec<libc::pid_t>>
{ unsafe
  { let the_vec: Vec<libc::pid_t> = Vec::with_capacity(Proz::MAXCHILDS as usize);
    let bonjour = the_vec.as_ptr() as *mut libc::c_void;
    let kappa = (the_vec.capacity() * std::mem::size_of::<libc::pid_t>()) as i32;

    match flag
    { Pid::PROC_CHILD_PIDS =>
      { let connard = proc_listchildpids(pid, bonjour, kappa);
        if connard > 0
        { Some(vec_pids(bonjour as *mut i32, connard as usize, the_vec)) }
        else
        { None }},
      _ => None, }}}
