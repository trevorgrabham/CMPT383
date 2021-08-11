#[repr(C)]
pub struct Buffer {
    data: *mut i32,
    len: usize,
}

#[no_mangle]
pub extern "C" fn double(x: i32) -> i32 {
    x*2
}

#[no_mangle]
pub extern "C" fn sum_arr(arr: Buffer) -> i32 {
    let mut res: i32 = 0;
    let array = unsafe {
        std::slice::from_raw_parts(arr.data, arr.len)
    };

    for elem in array {        
        res += elem;
    }
    res
}
