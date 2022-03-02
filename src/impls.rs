use std::fmt::Display;

pub trait Foo {
    fn foo(&self) -> String {
        format!("default impl")
    }
}

impl<T: Display> Foo for T {
    fn foo(&self) -> String {
        format!("self is {}, impl1", &self)
    }
}

// impl<T> Foo for T {
//     fn foo(&self) -> String {
//         format!("self is not displayable, impl2")
//     }
// }

#[test]
fn foo() {
    struct A {}
    println!("{}", 5.foo());
    // println!("{}", (A {}).foo());
}
