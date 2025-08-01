use std::marker::PhantomData;

use crate::post::{Body, Deleted, Title, Unmoderated};

mod post {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Title(pub String);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Body(pub String);

    // A series of zero-sized types representing the state of the post
    pub struct New;
    pub struct Unmoderated;
    pub struct Deleted;
    pub struct Published;
}
mod user {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);
}

#[derive(Clone)]
struct Post<S> {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
    state: PhantomData<S>,
}

impl Post<post::New> {
    pub fn new(
        id: post::Id,
        user_id: user::Id,
        title: post::Title,
        body: post::Body,
    ) -> Post<post::New> {
        Post {
            id,
            user_id,
            title,
            body,
            state: PhantomData,
        }
    }

    pub fn publish(self) -> Post<post::Unmoderated> {
        Post {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            state: std::marker::PhantomData,
        }
    }
}

// these methods can be called only when post's state is Unmoderated
impl Post<post::Unmoderated> {
    pub fn allow(self) -> Post<post::Published> {
        Post {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            state: PhantomData,
        }
    }

    pub fn deny(self) -> Post<post::Deleted> {
        Post {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            state: PhantomData,
        }
    }
}

impl Post<post::Published> {
    pub fn delete(self) -> Post<post::Deleted> {
        Post {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            state: PhantomData,
        }
    }
}
fn main() {
       let new_post = Post::new(
        post::Id(1),
        user::Id(1),
        post::Title("Title".to_string()),
        post::Body("Content".to_string()),
    );

    let res = new_post.publish().allow().delete();

    // res.deny(); // wouldn't compile.
    
    // let unmoderated = new_post.allow();  // wouldn't compile
    // new_post.delete();  // wouldn't compile
}
