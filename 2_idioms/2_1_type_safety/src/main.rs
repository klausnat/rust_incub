mod post {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(u64);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Title(String);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Body(String);

    // A series of zero-sized types representing the state of the post
    struct ReadyForModeration;
    struct Unmoderated;
    struct Deleted;
    struct Published;

    
    #[derive(Clone, Debug, PartialEq)]
    pub enum State {
        ReadyForModeration,
        Unmoderated,
        Deleted,
        Published,
    }
}
mod user {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(u64);
}

#[derive(Clone)]
struct Post {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
    state: post::State,
}

fn main() {}
