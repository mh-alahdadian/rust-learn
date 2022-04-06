#[test]
fn borrows1() {
    let mut x = String::from("hello");
    {
        let y = &mut x;
        // let z = &mut x;
        *y = String::from("world");
        println!("{}", y);
    }
    {
        let y = &mut x;
        // let z = &mut x;
        y.push_str("hi");
        println!("{}", y);
    }
    println!("{}", x);
}

#[test]
fn borrows2() {
    let s1 = String::from("hello");

    let len = {
        let s = &s1;
        s.len()
    };

    println!("The length of '{}' is {}.", s1, len);
}

#[test]
fn borrows3() {
    fn first_word<'a, 'b>(s: &'a String) -> &'b str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return "i as u32";
            }
        }
        "s.len() as u32"
    }
    fn first_word2<'a, 'b>(s: &'a String) -> &'a str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    let mut s = String::from("hello world");
    let word = first_word(&s);
    // let word2 = first_word2(&s);

    s.clear(); // this empties the String, making it equal to ""

    println!("the first word is: {}", word /* word2 */);
}
#[test]
fn borrows4() {
    let mut s1 = String::from("hello");

    let len = {
        let mut s = &mut s1;
        *s = String::from("world");
        s = &mut String::from("world2");
        5
    };

    println!("The length of '{}' is {}.", s1, len);
}

#[test]
fn borrows5() {
    let x = 5;
    let arr = [Some(5); 1024];
    let a = arr[2];

    let vec = vec![Some(5)];
    let b = vec[0];
    let c = &a;
    let d = "";

    // println!("{:?}, {:?}, {:?}, {:?}", a, b, vec[0], &vec[0]);
    println!(
        "{d} {:p}, {salam:?}, {:p}, {:p}, {:p}, {:p}",
        &c,
        c,
        &b,
        &vec[0],
        &vec[0],
        salam = 12
    );
}

#[test]
fn borrows6() {
    let arr = [Some(5); 1024];
    let a = arr[2];

    let vec = vec![Some(5)];
    let b = vec[0];
    let c = &a;
    let d = "";

    // println!("{:?}, {:?}, {:?}, {:?}", a, b, vec[0], &vec[0]);
    println!(
        "{d} {:p}, {salam:?}, {:p}, {:p}, {:p}, {:p}",
        &c,
        c,
        &b,
        &vec[0],
        &vec[0],
        salam = 12
    );
}

// pointers

#[test]
fn pointer1() {
    let m = Box::new(String::from("Rust"));
    let x: &str = &**m;
    let x: &str = &*m;
    let x: &str = &m;

    println!("pointer1: Box: {}", x);
}

#[test]
fn pointer2() {
    use std::ptr::eq;
    use std::rc::Rc;
    let a = Rc::new(String::from("Rust"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));
    drop(a);
    println!("count after droping a = {}", Rc::strong_count(&b));
    {
        let c = Rc::clone(&b);
        println!("count after creating c = {}", Rc::strong_count(&c));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&b));
    println!("");
    let a = Rc::clone(&b);
    println!("a is not equal to b: {}", eq(&a, &b));
    println!("*a is equal to *b: {}", eq(&*a, &*b));
}
