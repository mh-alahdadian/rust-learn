#[test]
fn destruct_by_pub_fields() -> () {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        // Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        Point { x, .. } => println!("x is {}", x),
    };
}


#[test]
fn early_return_none() -> () {
    let x = Some(4);

    let value = match x {
        Some(i) => i,
        None => return (),
    };

    println!("{}", value);
}
