use fail_on_ci::fail_on_ci;

fn main() {
    fail_on_ci!{ 
        println!("Hello, world!");
    }
}
