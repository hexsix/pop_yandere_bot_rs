from fastapi import APIRouter, Depends

from app.dependencies import get_redis_client, get_telegram_client, get_yandere_client
from app.services.redis import RedisClient
from app.services.telegram import TelegramClient
from app.services.yandere import YandereClient

router = APIRouter()


@router.get("/rss")
async def rss(
    redis_client: RedisClient = Depends(get_redis_client),
    telegram_client: TelegramClient = Depends(get_telegram_client),
    yandere_client: YandereClient = Depends(get_yandere_client),
):
    posts = await yandere_client.rss()

    for post in posts:
        if post.score < yandere_client.threshold:
            continue
        if post.parent_id is not None:
            parent_post = await yandere_client.new_post(post.parent_id)
            if parent_post is None:
                continue
            post = parent_post
        if post.has_children:
            children = await yandere_client.get_children(post)
        else:
            children = [post]
        if await redis_client.already_sent_posts([child.id for child in children]):
            continue
        await telegram_client.send_media_group(children)
        await redis_client.set_posts_as_sent([child.id for child in children])

    return "ok"
