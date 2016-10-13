
extern crate baum;
extern crate libc;

use baum::baum::current_pid;

fn main() {
    println!("Current pid: {}", current_pid());
}
