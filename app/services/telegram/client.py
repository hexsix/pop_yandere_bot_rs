import asyncio
from pathlib import Path
from urllib.parse import urlparse

from loguru import logger
from telegram import Bot, InputMediaPhoto
from telegram.constants import ParseMode

from app.configs import TelegramConfig
from app.services.yandere.models import Post


class TelegramClient:
    def __init__(self, config: TelegramConfig):
        self.token = config.token
        self.chat_id = config.chat_id
        self.channel_id = config.channel_id
        self.bot = Bot(token=self.token)

    def _escape_markdown_v2(self, text: str) -> str:
        """Escape special characters for Telegram MarkdownV2 format."""
        escape_chars = [
            "_",
            "*",
            "[",
            "]",
            "(",
            ")",
            "~",
            "`",
            ">",
            "#",
            "+",
            "-",
            "=",
            "|",
            "{",
            "}",
            ".",
            "!",
        ]

        escaped_text = text
        for escape_char in escape_chars:
            escaped_char = f"\\{escape_char}"
            escaped_text = escaped_text.replace(escape_char, escaped_char)

        return escaped_text

    def get_caption(self, post: Post) -> str:
        """Generate caption for a post with proper MarkdownV2 formatting."""
        caption = ""

        if post.source and post.source.strip():
            try:
                parsed_url = urlparse(post.source)

                if "pximg" in post.source:
                    # Handle pixiv URLs
                    source_path = Path(post.source)
                    if source_path.name:
                        base_name = source_path.stem
                        pixiv_id = base_name.split("_")[0]
                        if pixiv_id:
                            caption += f"source: [{self._escape_markdown_v2('www.pixiv.net')}]({self._escape_markdown_v2(f'https://www.pixiv.net/artworks/{pixiv_id}')})\n"
                elif parsed_url.hostname:
                    # Handle other URLs with hostname
                    caption += f"source: [{self._escape_markdown_v2(parsed_url.hostname)}]({self._escape_markdown_v2(post.source)})\n"
                else:
                    # Fallback for URLs without clear hostname
                    caption += f"source: {self._escape_markdown_v2(post.source)}\n"
            except Exception:
                # If URL parsing fails, just escape and show the raw source
                caption += f"source: {self._escape_markdown_v2(post.source)}\n"

        # Add yande.re link
        yande_url = f"https://yande.re/post/show/{post.id}"
        caption += self._escape_markdown_v2(yande_url)

        return caption

    async def send_media_group(self, posts: list[Post]) -> None:
        """Send media group in batches with proper formatting and delays."""
        if not posts:
            return

        # Process posts in chunks of 10 (Telegram's limit for media groups)
        for i in range(0, len(posts), 10):
            batch = posts[i : i + 10]

            # Add 1 second delay between batches (except for the first one)
            if i > 0:
                await asyncio.sleep(1)

            # Prepare media group
            media_group = [
                InputMediaPhoto(
                    media=post.sample_url,
                    caption=self.get_caption(post),
                    parse_mode=ParseMode.MARKDOWN_V2,
                )
                for post in batch
            ]

            # Send the batch with notification disabled
            await self.bot.send_media_group(
                chat_id=self.channel_id,
                media=media_group,
                disable_notification=True,
            )


async def test_new_1121916() -> list[Post]:
    from app.configs import RSSConfig
    from app.services.yandere.client import YandereClient

    client = YandereClient(RSSConfig())
    post = await client.new_post(1121916)
    if post is None:
        return []
    children = await client.get_children(post)
    return children


async def test_send_media_group():
    client = TelegramClient(TelegramConfig())

    posts = await test_new_1121916()
    await client.send_media_group(posts)
    logger.info(posts)


if __name__ == "__main__":
    asyncio.run(test_send_media_group())
