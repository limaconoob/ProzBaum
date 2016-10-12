
use libc;
use ffi::{Pid};
use prozess_macos::{proc_get_pids};

#[derive(Clone, Debug)]
pub struct Baum
{ pub pid: libc::pid_t,
  pub childs: Vec<Baum>, }

impl Baum
{ pub fn new(pid: libc::pid_t) -> Self
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

pub fn baum_printer(baum: Baum)
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
      for i in baum.childs
      { baum_printer(i); }}
    else
    { nb = 0; }}}

