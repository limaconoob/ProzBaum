extern crate baum;
extern crate libc;

use baum::baum::{Baum, BaumBenutz};

#[test]
///Compare deux arbres de pids
fn compare()
{
  let alt_baum: Baum = Baum {pid: 10, childs: vec![Baum {pid: 20, childs: Vec::new()}, Baum {pid: 30, childs: vec![Baum {pid: 32, childs: Vec::new()}, Baum {pid: 35, childs: vec![Baum {pid: 37, childs: Vec::new()}, Baum {pid: 38, childs: Vec::new()}]}, Baum {pid: 40, childs: Vec::new()}]}, Baum {pid: 50, childs: Vec::new()}]};

  let neu_baum: Baum = Baum {pid: 10, childs: vec![Baum {pid: 20, childs: Vec::new()}, Baum {pid: 30, childs: vec![Baum {pid: 32, childs: Vec::new()}, Baum {pid: 35, childs: vec![Baum {pid: 38, childs: Vec::new()}, Baum {pid: 39, childs: Vec::new()}]}, Baum {pid: 40, childs: Vec::new()}]}]};


  if alt_baum == neu_baum
  { println!("These are equivalent"); }
  else
  { println!("They are not equivalent"); }

  println!("");

  let (in_pids, out_pids) = neu_baum.vergleich(alt_baum);
  println!("In::{:?} | Out::{:?}", in_pids, out_pids);
}
