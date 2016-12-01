
use libc;
use ffi::{proc_bsdinfo};
use std::ops::Not;
use baum::{Baum, BaumBenutz, current_pid};

#[derive(Debug)]
pub enum Event
{ /// If the current pid is modified
  NewCurrentPid(libc::pid_t),
  /// If the tree from the current process is modified
  /// (created_pids, deleted_pids)
  ModifiedTree((Vec<libc::pid_t>, Vec<libc::pid_t>)), }

#[derive(Clone, Debug)]
pub struct Proc
{ pub current_pid: libc::pid_t,
  pub baum: Baum, }

impl Proc
{ pub fn new() -> Self
  { unsafe { Proc {current_pid: current_pid(), baum: Baum::new(libc::getpid())} }}}

impl Iterator for Proc
{ type Item = Event;
  fn next(&mut self) -> Option<Event>
  { if self.current_pid.eq(&current_pid()).not()
    { self.current_pid = current_pid();
      Some(Event::NewCurrentPid(self.current_pid)) }
    else
    { let baum = unsafe { Baum::new(libc::getpid()) };
      if self.baum.eq(&baum).not()
      { let ref old_baum = self.baum;
        Some(Event::ModifiedTree(baum.vergleich(old_baum))) }
      else
      { None }}}}

impl Default for Proc
{ fn default() -> Proc
  { unsafe { Proc {current_pid: current_pid(), baum: Baum::new(libc::getpid())} }}}
