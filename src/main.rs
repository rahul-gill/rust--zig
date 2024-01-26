#[link(name = "ziglib")]
extern "C" {
    fn adderfunc(a: i32, b: i32) -> i32;
}


fn main() {
    let r = unsafe { adderfunc(10, 20) };
    println!("Hello, world! sum is: {}", r);
}
