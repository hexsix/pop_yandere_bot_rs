use std::thread;
use std::time::Duration;
use teloxide::prelude::*;
use teloxide::types::{InputFile, InputMedia, InputMediaPhoto, ParseMode};
use url::Url;

use crate::yandere;
use yandere::Post;

pub async fn send_message(
    bot: &Bot,
    chat_id: String,
    post: &Post,
) -> Result<(), ()> {
    thread::sleep(Duration::from_secs(3));
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
            "oh, something went wrong with post = {}, response = {}",
            post.get_id(),
            response
        );
        return Err(());
    }
    info!("ok, send the post = {}", post.get_id());
    Ok(())
}

pub async fn send_media_group(
    bot: &Bot,
    chat_id: String,
    posts: &Vec<Post>,
) -> Result<(), ()> {
    thread::sleep(Duration::from_secs(3));
    let post_ids: Vec<i32> = posts.iter().map(|m| m.get_id()).collect();
    let media_group: Vec<InputMedia> = posts
        .iter()
        .rev()
        .map(|post| {
            InputMedia::Photo(
                InputMediaPhoto::new(InputFile::url(
                    Url::parse(post.get_sample_url()).unwrap(),
                ))
                .caption(post.get_caption())
                .parse_mode(ParseMode::MarkdownV2),
            )
        })
        .collect();

    if let Err(response) = bot
        .send_media_group(chat_id, media_group)
        .disable_notification(true)
        .await
    {
        warn!("oh, something went wrong in send_media_group, post_ids = {:?}, response = {}", post_ids, response);
        return Err(());
    }

    info!("ok, send all the posts = {:?}", post_ids);
    Ok(())
}
