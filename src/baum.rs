
use libc;
use ffi::{Pid};
use macos::{proc_get_pids, proc_get_info};
use ffi::{proc_bsdinfo};

#[derive(Debug)]
pub enum Status
{ ///Processus sur lequel on est
  Current,
  ///Processus actif
  Active,
  ///Processus suspendu > Ctrl+Z
  Suspend,
  ///Processus zombie > yes 1&>/dev/null &
  Zombie,
  ///Etat non implémenté
  Other, }

pub fn zustand(pid: libc::pid_t) -> Status
{ let (_, _, info, _) = proc_get_info(pid);
  match info
  { 2 =>
      { unsafe 
        { let cur_pid = libc::getpid();
          let cur_baum = Baum::new(cur_pid);
          let baum = Baum::new(pid);
          if !pid_in_baum(pid, &cur_baum) || !baum.childs.is_empty()
          { Status::Active }
          else
          { Status::Current }}},
    4 => Status::Suspend,
    7 => Status::Zombie,
    _ => Status::Other, }}

pub fn pid_in_baum(pid: libc::pid_t, baum: &Baum) -> bool
{ if baum.pid == pid
  { true }
  else if !baum.childs.is_empty() 
  { baum.childs.iter().find(|&b| pid_in_baum(pid, b)).is_some() }
  else
  { false }}

pub fn current_pid() -> libc::pid_t
{ unsafe
  { fn currpid(baum: &Baum, pid: &mut libc::pid_t)
    { match zustand(baum.pid)
      { Status::Current => { *pid = baum.pid; },
        _ => {}, }
      if !baum.childs.is_empty()
      { baum.childs.iter().map(|i|
          { currpid(i, pid);
            true }); }}
      let mut pid: libc::pid_t = 0;
      currpid(&Baum::new(libc::getpid()), &mut pid);
      pid }}

pub trait BaumBenutz
{ fn vergleich(&self, baum: &Baum) -> (Vec<libc::pid_t>, Vec<libc::pid_t>);
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

impl PartialEq for Baum
{ fn eq(&self, baum: &Baum) -> bool
  { if self.pid != baum.pid
    { false }
    else if !baum.childs.is_empty() 
    { !baum.childs.iter().find(|b| self.eq(b)).is_some() }
    else
    { true }}}

impl BaumBenutz for Baum
{ fn vergleich(&self, baum: &Baum) -> (Vec<libc::pid_t>, Vec<libc::pid_t>)
  { fn checker(get: &Baum, baum: &Baum, pids: &mut Vec<libc::pid_t>)
    { if !pid_in_baum(baum.pid, get)
      { pids.push(baum.pid); }
      if !baum.childs.is_empty()
      { baum.childs.iter().all(|x| { checker(get, x, pids) ; true}); }}
    let mut in_pids: Vec<libc::pid_t> = Vec::new();
    let mut out_pids: Vec<libc::pid_t> = Vec::new();
    checker(self, baum, &mut out_pids);
    checker(baum, self, &mut in_pids);
    (in_pids, out_pids) }

  fn anzeigt(&self)
  { unsafe
    { static mut nb: u32 = 0;
      static mut pl: char = '|';
      static mut tmp: u32 = 0;
      if self.pid > 0
      { {0..nb + 1}.all(|_|
        { print!(" ");
          true });
        let c = if !self.childs.is_empty() {'+'} else {'-'};
        print!("{}{}-= ", pl, c);
        pl = '|';
        println!("{} {:?}", self.pid, zustand(self.pid)); }
      if !self.childs.is_empty()
      { if self.pid > 0
        { nb += 1;
          tmp = nb; }
        {0.. self.childs.len()}.all(|i|
        { if i == self.childs.len() - 1
          { pl = '\\'; }
          self.childs[i].anzeigt();
          true }); }
      else
      { nb = 0; }}}}
