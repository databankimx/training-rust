pub mod foo {
    // bar must be pub for code outside foo to reach it. Here, a() reaches it from inside foo.
    pub mod bar {
        pub fn b() {
            println!("b");
        }
    }

    pub fn a() {
        bar::b();
    }
}

fn main() {
    foo::a();

    // Try: remove pub from `mod bar`, and call foo::bar::b() from main. Which line does the compiler reject?
}
