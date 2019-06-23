/**

The State trait defines the behavior shared by different post states, and the Draft, PendingReview, and Published states will all implement the State trait. For now, the trait doesn’t have any methods, and we’ll start by defining just the Draft state because that is the state we want a post to start in.

When we create a new Post, we set its state field to a Some value that holds a Box. This Box points to a new instance of the Draft struct. This ensures whenever we create a new instance of Post, it will start out as a draft. Because the state field of Post is private, there is no way to create a Post in any other state! In the Post::new function, we set the content field to a new, empty String.

 */

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

// The logic related to the rules lives in the state objects rather than being scattered throughout Post.
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
        self.state.as_ref().unwrap().content(&self) // Because the goal is to keep all these rules inside the structs that implement State, we call a content method on the value in state and pass the post instance (that is, self) as an argument. Then we return the value that is returned from using the content method on the state value... We call the as_ref method on the Option because we want a reference to the value inside the Option rather than ownership of the value. Because state is an Option<Box<dyn State>>, when we call as_ref, an Option<&Box<dyn State>> is returned. If we didn’t call as_ref, we would get an error because we can’t move state out of the borrowed &self of the function parameter.
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() { //To consume the old state, the request_review method needs to take ownership of the state value. This is where the Option in the state field of Post comes in: we call the take method to take the Some value out of the state field and leave a None in its place, because Rust doesn’t let us have unpopulated fields in structs. This lets us move the state value out of Post rather than borrowing it... We need to set state to None temporarily rather than setting it directly with code like self.state = self.state.request_review(); to get ownership of the state value. This ensures Post can’t use the old state value after we’ve transformed it into a new state.
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>; // Note that rather than having self, &self, or &mut self as the first parameter of the method, we have self: Box<Self>. This syntax means the method is only valid when called on a Box holding the type. This syntax takes ownership of Box<Self>, invalidating the old state so the state value of the Post can transform into a new state.

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str { // We add a default implementation for the content method that returns an empty string slice. That means we don’t need to implement content on the Draft and PendingReview structs. 
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {}) // The request_review method on Draft needs to return a new, boxed instance of a new PendingReview struct, which represents the state when a post is waiting for a review.
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // The PendingReview struct also implements the request_review method but doesn’t do any transformations. Rather, it returns itself, because when we request a review on a post already in the PendingReview state, it should stay in the PendingReview state.
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // The Published struct will override the content method and return the value in post.content.
    fn content<'a>(&self, post: &'a Post) -> &'a str { // Note that we need lifetime annotations on this method. We’re taking a reference to a post as an argument and returning a reference to part of that post, so the lifetime of the returned reference is related to the lifetime of the post argument.
        &post.content
    }
}

/**
 * HOWEVER!

By implementing the state pattern exactly as it’s defined for object-oriented languages, we’re not taking as full advantage of Rust’s strengths as we could. Let’s look at some changes we can make to the blog crate that can make invalid states and transitions into compile time errors.

But we also have to make some small changes to main. The request_review and approve methods return new instances rather than modifying the struct they’re called on, so we need to add more let post = shadowing assignments to save the returned instances. We also can’t have the assertions about the draft and pending review post’s contents be empty strings, nor do we need them: we can’t compile code that tries to use the content of posts in those states any longer.

fn main() {

    // REFACTOR (REMOVE)
    // let mut post = Post::new();

    // post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content()); // Instead of having this be valid - we could instead just have different types - and the compiler would alert us to the fact that this was still a draft
    
    // REFACTOR (ADD)
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }

// Both the Post and DraftPost structs have a private content field that stores the blog post text. The structs no longer have the state field because we’re moving the encoding of the state to the types of the structs. The Post struct will represent a published post, and it has a content method that returns the content.

We still have a Post::new function, but instead of returning an instance of Post, it returns an instance of DraftPost. Because content is private and there aren’t any functions that return Post, it’s not possible to create an instance of Post right now.

The DraftPost struct has an add_text method, so we can add text to content as before, but note that DraftPost does not have a content method defined! So now the program ensures all posts start as draft posts, and draft posts don’t have their content available for display. Any attempt to get around these constraints will result in a compiler error.

// So how do we get a published post? We want to enforce the rule that a draft post has to be reviewed and approved before it can be published. A post in the pending review state should still not display any content. Let’s implement these constraints by adding another struct, PendingReviewPost, defining the request_review method on DraftPost to return a PendingReviewPost, and defining an approve method on PendingReviewPost to return a Post.

// The request_review and approve methods take ownership of self, thus consuming the DraftPost and PendingReviewPost instances and transforming them into a PendingReviewPost and a published Post, respectively. This way, we won’t have any lingering DraftPost instances after we’ve called request_review on them, and so forth. The PendingReviewPost struct doesn’t have a content method defined on it, so attempting to read its content results in a compiler error, as with DraftPost. Because the only way to get a published Post instance that does have a content method defined is to call the approve method on a PendingReviewPost, and the only way to get a PendingReviewPost is to call the request_review method on a DraftPost, we’ve now encoded the blog post workflow into the type system.
 */