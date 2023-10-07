use std::thread;
use std::time::Duration;
use teloxide::prelude::*;
use teloxide::types::{InputFile, InputMedia, InputMediaPhoto, ParseMode};
use teloxide::RequestError;
use url::Url;

use crate::yandere::Post;
use crate::{BOT, CONFIG};

pub async fn send_media_group(
    posts: &[Post],
) -> Result<Vec<Message>, RequestError> {
    thread::sleep(Duration::from_secs(1));
    let media_group: Vec<InputMedia> = posts
        .iter()
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

    for media_group in media_group.chunks(10) {
        BOT.send_media_group(
            CONFIG.telegram.channel_id.clone(),
            media_group.to_vec(),
        )
        .disable_notification(true)
        .await?;
    }

    Ok(vec![])
}
