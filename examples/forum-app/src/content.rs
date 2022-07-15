use crate::types::*;
use crate::Msg;
use sauron::prelude::*;

#[derive(Debug, derive_more::From)]
pub enum Content {
    Posts(Vec<Post>),
    PostDetail(PostDetails),
    Errored(crate::Error),
}

impl Content {
    pub fn view(&self) -> Node<Msg> {
        match self {
            Content::Posts(posts) => self.view_posts(posts),
            Content::PostDetail(post_detail) => self.view_post_detail(post_detail),
            Content::Errored(error) => self.view_error(error),
        }
    }

    fn view_error(&self, error: &crate::Error) -> Node<Msg> {
        div(
            [class("error")],
            [text!("Something went wrong: {:#?}", error)],
        )
    }

    fn view_posts(&self, posts: &[Post]) -> Node<Msg> {
        if posts.is_empty() {
            div([class("empty-posts")], [text("There are no posts yet!")])
        } else {
            ol(
                [class("posts")],
                posts.into_iter().map(|post| self.view_post(post)),
            )
        }
    }

    fn view_post(&self, post: &Post) -> Node<Msg> {
        let post_id = post.post_id;
        li(
            [class("post")],
            [h2(
                [],
                [a(
                    [
                        href(format!("/item/{}", post_id)),
                        on_click(move |e| {
                            e.prevent_default();
                            Msg::ShowPost(post_id)
                        }),
                    ],
                    [text(post.content())],
                )],
            )],
        )
    }

    fn view_post_detail(&self, post_detail: &PostDetails) -> Node<Msg> {
        div(
            [class("post-detail")],
            [
                self.view_post(&post_detail.post),
                ul(
                    [class("comment-details")],
                    post_detail
                        .comments
                        .iter()
                        .map(|comment| self.view_comment_details(comment)),
                ),
            ],
        )
    }

    fn view_comment_details(&self, comment_detail: &CommentDetails) -> Node<Msg> {
        li(
            [class("comment-detail")],
            [
                self.view_comment(&comment_detail.comment),
                ul(
                    [],
                    comment_detail
                        .kids
                        .iter()
                        .map(|comment| self.view_comment_details(comment)),
                ),
            ],
        )
    }

    fn view_comment(&self, comment: &Comment) -> Node<Msg> {
        li([class("comment")], [text(comment.content())])
    }
}
