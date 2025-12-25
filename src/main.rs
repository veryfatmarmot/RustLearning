fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    post.add_text(" and some pickles.");    
    let post = post.request_review();

    let mut post = post.reject();    
    post.add_text(" And drunk some woter.");    
    let post = post.request_review();

    let post = post.approve();

    println!("{}", post.content());
}


// ==========================================================================================================

type Post = PostDraft;

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

    fn request_review(self) -> PostPendingReview {
        PostPendingReview {
            content: self.content,
        }
    }
}

// ==========================================================================================================

struct PostPendingReview {
    content: String,
}

impl PostPendingReview {
    fn approve(self) -> PostPublished {
        PostPublished {
            content: self.content,
        }
    }

    fn reject(self) -> PostDraft {
        PostDraft {
            content: self.content,
        }
    }
}

// ==========================================================================================================

//#[derive(Debug)]
struct PostPublished {
    content: String,
}

impl PostPublished {
    fn content(&self) -> &str {
        &self.content
    }
}