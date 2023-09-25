use std::thread;
use std::time::Duration;
use teloxide::prelude::*;
use teloxide::types::{InputFile, InputMedia, InputMediaPhoto, ParseMode};
use url::Url;

use crate::yandere::Post;
use crate::{BOT, CONFIG};

pub async fn send_media_group(posts: &[Post]) -> Result<(), ()> {
    thread::sleep(Duration::from_secs(3));
    let post_ids: Vec<i32> = posts.iter().map(|m| m.get_id()).collect();
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

    if let Err(e) = BOT
        .send_media_group(CONFIG.telegram.channel_id.clone(), media_group)
        .disable_notification(true)
        .await
    {
        error!("error(send_msg), post_ids = {:?}, error = {}", post_ids, e);
        return Err(());
    }

    info!("ok(send_media_group), posts = {:?}", post_ids);
    Ok(())
}
