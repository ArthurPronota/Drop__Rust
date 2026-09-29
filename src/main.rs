struct Guard ;
use std::mem::drop ;

impl Drop for Guard {
    fn drop(&mut self) {
        println!("Drop data!");
    }
}

fn main() {
    let g = Guard ;

    drop(g);
}
