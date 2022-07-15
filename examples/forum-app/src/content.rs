use crate::types::*;
use crate::Msg;
use sauron::prelude::*;

#[derive(Debug, derive_more::From)]
pub enum Content {
    Posts(Vec<Post>),
    PostDetail(PostDetails),
}

impl Content {
    pub fn view(&self) -> Node<Msg> {
        match self {
            Content::Posts(posts) => self.view_posts(posts),
            Content::PostDetail(post_detail) => self.view_post_detail(post_detail),
        }
    }

    fn view_posts(&self, posts: &[Post]) -> Node<Msg> {
        ol([], posts.into_iter().map(|post| self.view_post(post)))
    }

    fn view_post(&self, post: &Post) -> Node<Msg> {
        let post_id = post.post_id;
        li(
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
        )
    }

    fn view_post_detail(&self, post_detail: &PostDetails) -> Node<Msg> {
        div(
            [],
            [
                self.view_post(&post_detail.post),
                ul(
                    [],
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
            [],
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
        li([], [text(comment.content())])
    }
}
