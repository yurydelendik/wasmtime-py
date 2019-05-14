
extern "C" {
    fn callback(s: *const u8, s_len: u32);
}

#[no_mangle]
pub extern fn test() {
  let msg = "Hello, world!";
  unsafe {
    callback(msg.as_ptr(), msg.len() as u32);
  }
}
