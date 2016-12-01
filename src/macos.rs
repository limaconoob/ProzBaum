
extern crate std;

use libc;
use ffi::{Pid};
use ffi::{proc_listchildpids, proc_pidinfo};
use ffi;
use ffi::{proc_bsdinfo};
use std::mem;

unsafe fn get_unchecked_str(cp: *mut u8, start: *mut u8) -> String {
  let len = cp as usize - start as usize;
  let part = Vec::from_raw_parts(start, len, len);
  let tmp = String::from_utf8_unchecked(part.clone());
  mem::forget(part);
  tmp
}


pub fn proc_get_info(pid: libc::pid_t) -> (libc::pid_t, libc::pid_t, libc::c_uchar, String)
{ unsafe
  { let taskallinfo_size = mem::size_of::<ffi::proc_taskallinfo>() as i32;
    let mut mib: [libc::c_int; 3] = [ffi::CTL_KERN, ffi::KERN_ARGMAX, 0];
    let mut argmax = 0;
    let mut size = mem::size_of::<libc::c_int>();
    while libc::sysctl(mib.as_mut_ptr(), 2, (&mut argmax) as *mut i32 as *mut libc::c_void, &mut size, ::std::ptr::null_mut(), 0).eq(&-1) {}
    mib[0] = ffi::CTL_KERN;
    mib[1] = ffi::KERN_PROCARGS2;
    let mut proc_args: Vec<u8> = Vec::with_capacity(argmax as usize);
    pousse(pid, taskallinfo_size as i32, &mut proc_args, mib, argmax as libc::size_t) }}

fn pousse(pid: libc::c_int, taskallinfo_size: i32, proc_args: &mut Vec<u8>, mut mib: [libc::c_int; 3], argmax: libc::size_t) -> (libc::pid_t, libc::pid_t, libc::c_uchar, String)
{ unsafe
  { let mut task_info = mem::zeroed::<ffi::proc_taskallinfo>();
    if ffi::proc_pidinfo(pid, ffi::PROC_PIDTASKALLINFO, 0, &mut task_info as *mut ffi::proc_taskallinfo as *mut libc::c_void, taskallinfo_size) != taskallinfo_size
    { return (pid, 0, 0, "No_path".to_string()); }
    let ptr = proc_args.as_mut_slice().as_mut_ptr();
    mib[2] = pid as libc::c_int;
    let mut size = argmax;
    let mut name = String::new();

    if libc::sysctl(mib.as_mut_ptr(), 3, ptr as *mut libc::c_void, &mut size, ::std::ptr::null_mut(), 0) != -1
    { let mut cp = ptr.offset(mem::size_of::<libc::c_int>() as isize);
      let start = cp;
      if cp < ptr.offset(size as isize)
      { while cp < ptr.offset(size as isize) && *cp != 0
        { cp = cp.offset(1); }
        let exe = get_unchecked_str(cp, start);
        if let Some(l) = exe.split("/").last()
        { name = l.to_owned(); }}
        (pid, task_info.pbsd.ppbi_pid as i32, (task_info.pbsd.pbi_nice as u32 ^ task_info.pbsd.pbi_status) as u8, name) }
    else
    { (pid, 0, 0, "No_path".to_string()) }}}

fn vec_pids(bonjour: *mut i32, coucou: usize, the_vec: Vec<libc::pid_t>) -> Vec<libc::pid_t>
{ unsafe
  { let mut pids = Vec::from_raw_parts(bonjour, coucou, the_vec.capacity());
    pids.retain(|&p| p > 0i32);
    pids.reverse();
    pids }}

pub fn proc_get_pids<'a>(flag: Pid, pid: libc::pid_t) -> Option<Vec<libc::pid_t>>
{ unsafe
  { let the_vec: Vec<libc::pid_t> = Vec::with_capacity(Pid::MAXCHILDS as usize);
    let bonjour = the_vec.as_ptr() as *mut libc::c_void;
    let kappa = (the_vec.capacity() * std::mem::size_of::<libc::pid_t>()) as i32;

    match flag
    { Pid::PROC_CHILD_PIDS =>
      { let coucou = proc_listchildpids(pid, bonjour, kappa);
        if coucou > 0
        { Some(vec_pids(bonjour as *mut i32, coucou as usize, the_vec)) }
        else
        { None }},
      _ => None, }}}
