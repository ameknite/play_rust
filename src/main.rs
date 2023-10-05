use std::cell::RefCell;
thread_local! {
    pub static FOO: RefCell<u32> = RefCell::new(1);

    static BAR: RefCell<f32> = RefCell::new(1.0);
}

// fn main() {
//     FOO.with(|foo| assert_eq!(*foo.borrow(), 1));
//     BAR.with(|bar| assert_eq!(*bar.borrow(), 1.0));
// }
fn main() {
    println!("Hello world")
}
