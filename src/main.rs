static mut CURRENT_LEVEL: i32 = 1;
static MAX_DEPTH: i32 = 10;
const ARR: &[&str] = &["A", "B", "C"];

fn main() {
    unsafe {
        while CURRENT_LEVEL <= MAX_DEPTH {
            let start: String = "".to_string();
            printer(start, 1);
            CURRENT_LEVEL += 1;
        }
    }
}

fn printer(parent_string: String, my_level: i32) {
    for x in ARR {
        unsafe {
            if my_level == CURRENT_LEVEL {
                println!("{}{}", parent_string, x)
            } else {
                let new_string = format!("{}{}", parent_string, x);
                printer(new_string, my_level + 1)
            }
        }
    }
}
