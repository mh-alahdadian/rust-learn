enum PostState {
    Draft,
    Approval(u32),
    Published,
}

pub struct Post {
    state: PostState,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: PostState::Draft,
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        match &self.state {
            PostState::Published => &self.content,
            ـ => "",
        }
    }
    pub fn request_review(&mut self) {
        match self.state {
            PostState::Draft => self.state = PostState::Approval(0),
            _ => {}
        }
    }
    pub fn approve(&mut self) {
        match self.state {
            PostState::Approval(n) if n == 1 => self.state = PostState::Published,
            PostState::Approval(n) => self.state = PostState::Approval(n + 1),
            _ => {}
        }
    }
}

#[test]
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
