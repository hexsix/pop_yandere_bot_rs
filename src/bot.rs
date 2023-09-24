use teloxide::prelude::*;
use teloxide::types::{InputFile, InputMedia, InputMediaPhoto, ParseMode};
use url::Url;

use yandere::Post;

use crate::yandere;

pub async fn send_message(bot: &Bot, chat_id: String, post: &Post) {
    if let Err(response) = bot
        .send_photo(
            chat_id,
            InputFile::url(Url::parse(post.get_sample_url()).unwrap()),
        )
        .caption(post.get_caption())
        .parse_mode(ParseMode::MarkdownV2)
        .disable_notification(true)
        .await
    {
        warn!(
            "Something went wrong with post = {}, response = {}",
            post.get_id(),
            response
        );
    }
}

pub async fn send_media_group(bot: &Bot, chat_id: String, children: Vec<Post>) {
    let media_group: Vec<InputMedia> = children
        .iter()
        .rev()
        .map(|child| {
            InputMedia::Photo(
                InputMediaPhoto::new(InputFile::url(
                    Url::parse(child.get_sample_url()).unwrap(),
                ))
                .caption(child.get_caption())
                .parse_mode(ParseMode::MarkdownV2),
            )
        })
        .collect();
    if let Err(response) = bot
        .send_media_group(chat_id, media_group)
        .disable_notification(true)
        .await
    {
        warn!("Something went wrong todo",);
    }
}
