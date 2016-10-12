
extern crate top;
extern crate libc;

use self::top::convert::{u8_to_u32, u8_to_u64, u8_to_string};

#[test]
fn conversions()
{ //u8_to_u32(c: &[u8], k: usize) -> libc::uint32_t
  //u8_to_u32 converts a &[u8] (= [u8; 4]) into a u32
  //c.len() has to be >= 4
  //k is the beginning of conversion and has to be <= c.len() - 4
  assert_eq!(u8_to_u32(&[1, 1, 1, 1], 0), 0x01010101);
  assert_eq!(u8_to_u32(&[128, 128, 128, 128], 0), 0x80808080);
  assert_eq!(u8_to_u32(&[255, 255, 255, 255], 0), 0xFFFFFFFF);
  assert_eq!(u8_to_u32(&[255, 255, 255, 255, 0], 1), 0x00FFFFFF);

  //u8_to_u64(c: &[u8], k: usize) -> libc::uint64_t
  //u8_to_u64 converts a &[u8] (= [u8; 8]) into a u64
  //c.len() has to be >= 8
  //k is the beginning of conversion and  has to be <= c.len() - 8
  assert_eq!(u8_to_u64(&[1, 1, 1, 1, 1, 1, 1, 1], 0), 0x0101010101010101);
  assert_eq!(u8_to_u64(&[128, 128, 128, 128, 128, 128, 128, 128], 0), 0x8080808080808080);
  assert_eq!(u8_to_u64(&[255, 255, 255, 255, 255, 255, 255, 255], 0), 0xFFFFFFFFFFFFFFFF);
  assert_eq!(u8_to_u64(&[255, 255, 255, 255, 255, 255, 255, 255, 0], 1), 0x00FFFFFFFFFFFFFF);

  //u8_to_string(c: &[u8], k: usize, size: usize) -> String
  //u8_to_string converts a &[u8] (= [u8; size]) into a String
  //c.len() has to be >= size
  //k is the beginning of conversion and has to be <= c.len() - size
  assert_eq!(u8_to_string(&[66, 111, 110, 106, 111, 117, 114], 7), "Bonjour".to_string());
  assert_eq!(u8_to_string(&[67, 111, 110, 110, 97, 114, 100, 0, 0, 0], 10), "Connard".to_string());
  assert_eq!(u8_to_string(&[67, 111, 117, 99, 111, 117, 0], 7), "Coucou".to_string()); }
