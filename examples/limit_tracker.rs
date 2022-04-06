pub trait Messenger {
    fn send(&mut self, msg: &str);
}
pub struct LimitTracker<T: Messenger> {
    pub messenger: T,
    value: usize,
    max: usize,
}

impl<T: Messenger> LimitTracker<T> {
    pub fn new(messenger: T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

struct MockMessenger {
    sent_messages: Vec<String>,
}
impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: vec![],
        }
    }
}
impl Messenger for MockMessenger {
    fn send(&mut self, message: &str) {
        self.sent_messages.push(String::from(message));
    }
}

fn main() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(mock_messenger, 100);

    limit_tracker.set_value(80);

    assert_eq!(limit_tracker.messenger.sent_messages.len(), 1);
}
