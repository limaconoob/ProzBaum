extern crate top;
extern crate libc;

use top::baum::{Baum, BaumBenutz};

///Affiche la liste de tous les pids avec leur status.
fn main()
{ let baum: Baum = Baum::default();
  baum.anzeigt(); }
