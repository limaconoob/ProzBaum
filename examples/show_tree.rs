extern crate baum;
extern crate libc;

use baum::baum::{Baum, BaumBenutz};

///Affiche la liste de tous les pids avec leur status.
fn main()
{ let baum: Baum = Baum::default();
  baum.anzeigt(); }
