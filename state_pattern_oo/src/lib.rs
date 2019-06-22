/**

The State trait defines the behavior shared by different post states, and the Draft, PendingReview, and Published states will all implement the State trait. For now, the trait doesn’t have any methods, and we’ll start by defining just the Draft state because that is the state we want a post to start in.

When we create a new Post, we set its state field to a Some value that holds a Box. This Box points to a new instance of the Draft struct. This ensures whenever we create a new instance of Post, it will start out as a draft. Because the state field of Post is private, there is no way to create a Post in any other state! In the Post::new function, we set the content field to a new, empty String.

 */

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // Even after we’ve called add_text and added some content to our post, we still want the content method to return an empty string slice because the post is still in the draft state

    pub fn content(&self) -> &str {
        ""
    }
}

trait State {}

struct Draft {}

impl State for Draft {}