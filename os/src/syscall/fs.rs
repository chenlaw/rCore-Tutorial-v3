const FD_STDOUT: usize = 1;
use crate::batch::is_good_address;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let address = buf as usize;
    if is_good_address(address, len) != 0 {
        return -1;
    }
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        },
        _ => {
            -1
            // panic!("Unsupported fd in sys_write!");
        }
    }
}