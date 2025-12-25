fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    post.add_text(" and some pickles.");
    post.approve();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    println!("{}", post.content());
}

// ==========================================================================================================

enum Post {
    Draft(PostDraft),
    PendingReview(PostPendingReview),
    Published(PostPublished),
}

impl Post {
    fn new() -> Post {
        Post::Draft(PostDraft::new())
    }

    fn add_text(&mut self, text: &str) {
        if let Post::Draft(draft) = self {
            draft.add_text(text);
        }
    }

    fn request_review(&mut self) {
        if let Post::Draft(state_obj) = self {
            let content = std::mem::take(&mut state_obj.content);
            *self = Post::PendingReview(PostPendingReview::new(content));
        }
    }

    fn approve(&mut self) {
        if let Post::PendingReview(state_obj) = self {
            let content = std::mem::take(&mut state_obj.content);
            *self = Post::Published(PostPublished::new(content));
        }
    }

    fn reject(&mut self) {
        if let Post::PendingReview(state_obj) = self {
            let content = std::mem::take(&mut state_obj.content);
            *self = Post::Draft(PostDraft { content });
        }
    }

    fn content(&self) -> &str {
        match self {
            Post::Published(published) => published.content(),
            _ => "",
        }
    }
}

// ==========================================================================================================

struct PostDraft {
    content: String,
}

impl PostDraft {
    fn new() -> PostDraft {
        PostDraft {
            content: String::new(),
        }
    }

    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

// ==========================================================================================================

struct PostPendingReview {
    content: String,
}

impl PostPendingReview {
    fn new(content: String) -> PostPendingReview {
        PostPendingReview { content: content }
    }
}

// ==========================================================================================================

struct PostPublished {
    content: String,
}

impl PostPublished {
    fn new(content: String) -> PostPublished {
        PostPublished { content: content }
    }

    fn content(&self) -> &str {
        &self.content
    }
}
