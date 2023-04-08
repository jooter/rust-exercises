
#[no_mangle]
pub extern "C" 
fn add_one(x: i32) -> i32 {
    x +  1
}


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

#[no_mangle]
pub extern "C" 
fn get_points() -> [i32; 2] {
    [9; 2]
}