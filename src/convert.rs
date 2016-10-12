
use libc;

pub fn u8_to_u64(c: &[u8], k: usize) -> libc::uint64_t
{ let mut ret: u64 = 0;
  {0..8}.all(|i|
  { ret |= (c[i + k] as u64) << (i * 8);
    true });
  ret }

pub fn u8_to_u32(c: &[u8], k: usize) -> libc::uint32_t
{ let mut ret: u32 = 0;
  {0..4}.all(|i|
  { ret |= (c[i + k] as u32) << (i * 8);
    true });
  ret }

pub fn u8_to_string(c: &[u8], size: usize) -> String
{ let mut ret: String = String::with_capacity(size);
  {0..size}.all(|i|
  { if c[i] >= 32
    { ret.push(c[i] as char);
      true }
    else
    { false }});
  ret }
