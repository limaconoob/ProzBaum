extern crate libc;

mod ffi;
use ffi::{Pid, Proz}; //ENUMS
use ffi::{ProcBsdInfo}; //STRUCTS
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

impl ProcBsdInfo
{ pub fn new(/*proz: Vec<u8>*/) -> Self
  { ProcBsdInfo
    { pbi_flags:            0,//u8_to_u32(&proz, 0),
      pbi_status:           0,
      pbi_xstatus:          0,
      pbi_pid:              0,
      pbi_ppid:             0,
      pbi_uid:              0,
      pbi_gid:              0,
      pbi_ruid:             0,
      pbi_rgid:             0,
      pbi_svuid:            0,
      pbi_svgid:            0,
      rfu_1:                0,
      pbi_comm:             [0; Pid::MAXCOMLEN as usize],
      pbi_name:             [0; 2 * Pid::MAXCOMLEN as usize],
      pbi_nfiles:           0,
      pbi_pgid:             0,
      pbi_pjobc:            0,
      e_tdev:               0,
      e_tpgid:              0,
      pbi_nice:             0,
      pbi_start_tvsec:      0,
      pbi_start_tvusec:     0, }}}

fn u8_to_u64(c: &[u8], k: usize) -> libc::uint64_t
{ let mut ret: u64 = 0;
  {0..8}.all(|i|
  { ret |= (c[i + k] as u64) << (i * 8);
    true });
  println!("ret::{}", ret);
  ret }

fn u8_to_u32(c: &[u8], k: usize) -> libc::uint32_t
{ let mut ret: u32 = 0;
  {0..4}.all(|i|
  { ret |= (c[i + k] as u32) << (i * 8);
  println!("c[{}]::{}", i, c[i + k]);
    true });
  println!("ret::{}", ret);
  ret }

//proc_pidinfo(father, PROC_PIDTBSDINFO, 0, &proc, PROC_PIDTBSDINFO_SIZE);

fn proc_get_info(pid: libc::pid_t) -> Option<ProcBsdInfo>
{ unsafe
  { let size = std::mem::size_of::<ProcBsdInfo>();
    let mut proz: Vec<u8> = Vec::with_capacity(size);
    let bonjour = proz.as_ptr() as *mut libc::c_void;
    let ret = proc_pidinfo(pid, Pid::PROC_PIDTBSDINFO as i32, 0, bonjour, size as i32);
    proz = Vec::from_raw_parts(bonjour as *mut u8, size, proz.capacity());
    println!("pid::{}", pid);
    //let bonjour = proz.clone();
    for i in proz
    { print!("{} ", i); }
    let info = ProcBsdInfo::new();
    Some(info)
}}

fn vec_pids(bonjour: *mut i32, connard: usize, the_vec: Vec<libc::pid_t>) -> Vec<libc::pid_t>
{ unsafe
  { let mut pids = Vec::from_raw_parts(bonjour, connard, the_vec.capacity());
    pids.retain(|&p| p > 0i32);
    pids.reverse();
    pids }}

fn proc_get_pids<'a>(flag: Pid, pid: libc::pid_t) -> Option<Vec<libc::pid_t>>
{ unsafe
  { let mut the_vec: Vec<libc::pid_t> = Vec::with_capacity(Proz::MAXCHILDS as usize);
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
    proc_get_info(pid);
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
    static mut pl: char = '|';
    if baum.pid > 0
    { for _ in 0..(nb + 1)
      { print!(" "); }
      print!("{}--= ", pl);
      pl = '|';
      println!("{}", baum.pid); }
    if !baum.childs.is_empty()
    { if baum.pid > 0
      { nb += 1;
        pl = '\\'; }
      let k = baum.childs.clone();
      let len = k.capacity();
      for i in k
      { baum_printer(i); }}
    else
    { nb = 0; }}}

fn main()
{ unsafe
  { println!("size::{}", std::mem::size_of::<ProcBsdInfo>());
    let _x = Baum::default();
    baum_printer(_x);
    println!("THE::{}", libc::getpid());
    while true {;} }}
