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


struct Salam;

mod Page1 {
    use super::*;
    impl Salam {
        fn goto1(&self) {
            println!("salam");
        }
    }

    #[test]
    fn main() {
        let s = Salam;
        let _x = 5;
        s.goto1();
        // s.goto2();
    }
}

mod Page2 {
    use super::*;
    impl Salam {
        fn goto2(&self) {
            println!("salam");
        }
    }

    #[test]
    fn main() {
        let s = Salam;
        let _x = 5;
        // s.goto1();
        s.goto2();
    }
}
