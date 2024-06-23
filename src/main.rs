use std::error::Error;

fn up_or_down(other: Option<*const i8>) -> Result<bool, Box<dyn Error>> {
    let x: i8 = 0;
    if other.is_none() {
        return up_or_down(Some(&x));
    } else {
        return Ok(std::ptr::addr_of!(x) > other.ok_or("Error").unwrap());
    }
}

fn main() -> () {
    let res: Result<bool, Box<dyn Error>> = up_or_down(None);
    if res.is_ok() {
        println!("{}", if res.unwrap() { "UP" } else { "DOWN" });
    }
}
