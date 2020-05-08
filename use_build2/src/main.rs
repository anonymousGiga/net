extern { fn hello(); }

fn main() {
    unsafe {
        hello();
    }
}
