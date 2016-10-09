extern crate libc;

mod ffi;
use ffi::{Pid, Proz};
use ffi::{proc_pidpath, proc_listchildpids, proc_pidinfo};

/*
{
   int mib[2];
   size_t len;
   struct kinfo_proc info;
   mib[0] = CTL_KERN;
   mib[1] = KERN_PROC;
   len = sizeof(int);
   sysctl(mib, 2, &info, &len, NULL, 0);
   printf("jobc::%d, pflag::%d, pstat::%d, ppid::%d, poppid::%d, dupfd::%d, userstack::%s, exitthread::%s, debug::%d, sigwait::%d, pestcpu::%d, pcpsticks::%d, ppctcpu::%d, sleepaddress::%s, sleepreason::%s, pswtime::%d, pslptime::%d, prealtimer::, prtime::, ptraceflag::%d, ptracep::%p, psiglist::%d, \n", info.kp_eproc.e_jobc,
info.kp_proc.p_flag,
  (int)info.kp_proc.p_stat,
  info.kp_proc.p_pid,
  info.kp_proc.p_oppid,
  info.kp_proc.p_dupfd,
  info.kp_proc.user_stack,
  info.kp_proc.exit_thread,
  info.kp_proc.p_debugger,
  info.kp_proc.sigwait,
  info.kp_proc.p_estcpu,
  info.kp_proc.p_cpticks,
  info.kp_proc.p_pctcpu,
  info.kp_proc.p_wchan,
  info.kp_proc.p_wmesg,
  info.kp_proc.p_swtime,
  info.kp_proc.p_slptime,
  info.kp_proc.p_realtimer,
  info.kp_proc.p_rtime,
  info.kp_proc.p_uticks,
  info.kp_proc.p_sticks,
  info.kp_proc.p_iticks,
  info.kp_proc.p_traceflag,
  info.kp_proc.p_tracep,
  info.kp_proc.p_siglist,
  info.kp_proc.p_textvp,
  info.kp_proc.p_holdcnt,
  info.kp_proc.p_sigmask,
  info.kp_proc.p_sigignore,
  info.kp_proc.p_sigcatch,
  info.kp_proc.p_priority,
  info.kp_proc.p_usrpri,
  info.kp_proc.p_nice,
  info.kp_proc.p_comm,
  info.kp_proc.p_pgrp,
  info.kp_proc.p_addr,
  info.kp_proc.p_xstat,
  info.kp_proc.p_acflag,
  info.kp_proc.p_ru);
}
*/

/*
int bufsize = proc_listpids(PROC_ALL_PIDS, 0, NULL, 0);
struct proc_bsdinfo proc;
pid_t pids[2 * bufsize / sizeof(pid_t)];
bufsize = proc_listpids(PROC_ALL_PIDS, 0, pids, sizeof(pids));
size_t num_pids = bufsize / sizeof(pid_t);
printf("size::%d, pids::%zu\n", bufsize, num_pids);

char pathbuf[PROC_PIDPATHINFO_MAXSIZE];
proc_pidpath(pid, pathbuf, sizeof(pathbuf));
*/

//proc_pidinfo(father, PROC_PIDTBSDINFO, 0, &proc, PROC_PIDTBSDINFO_SIZE);

fn vec_pids(bonjour: *mut i32, connard: usize, the_vec: Vec<libc::pid_t>) -> Vec<libc::pid_t>
{ unsafe
  { let mut pids = Vec::from_raw_parts(bonjour, connard, the_vec.capacity());
    println!("connard::{}", connard);
    pids.retain(|&p| p > 0i32);
    pids.reverse();
    pids }}

fn proc_get_pids<'a>(flag: Pid, pid: libc::pid_t) -> Option<Vec<libc::pid_t>>
{ unsafe
  { let mut the_vec: Vec<libc::pid_t> = Vec::with_capacity(Pid::MAXCHILDS as usize);
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

#[derive(Clone, Debug)]
pub struct Baum
{ pid: libc::pid_t,
  childs: Vec<Baum>, }

/*
impl Drop for Baum
{ fn drop(&mut self)
  {}}
*/

impl Baum
{ fn new(pid: libc::pid_t) -> Self
  { let pids = proc_get_pids(Pid::PROC_CHILD_PIDS, pid);
    let mut childs = Vec::new();
    match pids
    { Some(vec) =>
      { for i in vec
        { childs.push(Baum::new(i)) }
        Baum { pid: pid, childs: childs, }},
      None => Baum { pid: pid, childs: childs, }, }}}

impl Default for Baum
{ fn default() -> Baum
  { Baum { pid: 0, childs: vec![Baum::new(0)], }}}

fn baum_printer(baum: Baum)
{ unsafe
  { static mut nb: u32 = 0;
    if !baum.childs.is_empty()
    { for i in 0..nb
      { print!("|"); }
      nb += 1;
      println!("pid::{}\n", baum.pid);
      let k = baum.childs.clone();
      for i in k
      { baum_printer(i); }}
    else
    { for i in 0..nb
      { print!("|"); }
      println!("pid::{}", baum.pid); }}}

fn main()
{ unsafe
  { let _x = Baum::new(libc::getppid());
    baum_printer(_x);
    while true
    {;} }}
