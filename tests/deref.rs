use std::ops::Deref;

struct Person {
    pub name: String,
}

struct Student {
    pub person: Person,
}

impl Deref for Student {
    type Target = Person;

    fn deref(&self) -> &Person {
        &self.person
    }
}

#[test]
fn auto_deref_student() {
    let s = Student {
        person: Person {
            name: "me".to_string(),
        },
    };
    println!("{:?}", s.name);
}
