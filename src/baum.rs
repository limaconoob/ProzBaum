
use libc;
use ffi::{Pid};
use prozess_macos::{proc_get_pids};

fn pid_in_baum(pid: libc::pid_t, baum: &Baum) -> bool
{ if baum.pid == pid
  { true }
  else if !baum.childs.is_empty() 
  { baum.childs.iter().find(|&b| pid_in_baum(pid, b)).is_some() }
  else
  { false }}

pub trait BaumBenutz
{ fn vergleich(&self, baum: Baum) -> (Vec<libc::pid_t>, Vec<libc::pid_t>);
  fn anzeigt(&self); }

#[derive(Clone, Debug)]
pub struct Baum
{ pub pid: libc::pid_t,
  pub childs: Vec<Baum>, }

impl Baum
{ pub fn new(pid: libc::pid_t) -> Self
  { let pids = proc_get_pids(Pid::PROC_CHILD_PIDS, pid);
    let mut childs = Vec::new();
    match pids
    { Some(mut vec) =>
      { vec.sort_by(|a, b| a.cmp(b));
        {0..vec.len()}.all(|i|
        { childs.push(Baum::new(vec[i]));
          true });
        Baum { pid: pid, childs: childs, }},
      None => Baum { pid: pid, childs: childs, }, }}}

impl Default for Baum
{ fn default() -> Baum
  { Baum { pid: 0, childs: vec![Baum::new(0)], }}}

impl BaumBenutz for Baum
{ fn vergleich(&self, baum: Baum) -> (Vec<libc::pid_t>, Vec<libc::pid_t>)
  { unsafe
    { fn checker(get: &Baum, baum: Baum, pids: &mut Vec<libc::pid_t>)
      { if !pid_in_baum(baum.pid, get)
        { pids.push(baum.pid); }
        if !baum.childs.is_empty()
        { for i in baum.childs
          { checker(get, i, pids); }}}
      let mut in_pids: Vec<libc::pid_t> = Vec::new();
      let mut out_pids: Vec<libc::pid_t> = Vec::new();
      checker(self, baum.clone(), &mut out_pids);
      checker(&baum, self.clone(), &mut in_pids);
      (in_pids, out_pids) }}
  
  fn anzeigt(&self)
  { unsafe
    { static mut nb: u32 = 0;
      static mut pl: char = '|';
      if self.pid > 0
      { {0..nb + 1}.all(|_|
        { print!(" ");
          true });
        print!("{}--= ", pl);
        pl = '|';
        println!("{}", self.pid); }
      if !self.childs.is_empty()
      { if self.pid > 0
        { nb += 1;
          pl = '\\'; }
        {0.. self.childs.len()}.all(|i|
        { self.childs[i].anzeigt();
          true }); }
      else
      { nb = 0; }}}}
