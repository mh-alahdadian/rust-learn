mod result {
    fn x() -> Result<String, String> {
        // Ok("5".to_string())
        Err("5".to_string())
    }

    #[test]
    fn unwrap() {
        let v = x().unwrap();

        println!("value is: {}", v);
    }
}

mod option {
    fn y() -> Option<String> {
        // Some("5".to_string())
        None
    }

    #[test]
    fn unwrap2() {
        let v = y().unwrap_or_default();

        println!("value is: {}", v);
    }
}
