extern crate top;
extern crate libc;

use top::baum::{Baum, pid_in_baum};

///Constate que le pid courant est dans la liste de tous les pids
fn main()
{ unsafe 
  { let pid = libc::getpid();
    println!("{} is the process? {}", pid, pid_in_baum(pid, &Baum::default())); }}
