import json
import re

import httpx
from loguru import logger

from app.configs import RSSConfig

from .models import Post, Posts


class YandereClient:
    def __init__(self, config: RSSConfig):
        self.url = config.url
        self.threshold = config.threshold
        self.session = httpx.AsyncClient(follow_redirects=True)

    async def new_post(self, id: int) -> Post | None:
        logger.info(f"Getting new post {id}")
        response = await self.session.get(
            f"{self.url}/post.json?api_version=2&tags=id:{id}"
        )
        response.raise_for_status()
        result = Posts.model_validate(response.json())
        if len(result.posts) == 0:
            return None
        return result.posts[0]

    async def get_children(self, post: Post) -> list[Post]:
        logger.info(f"Getting children of post {post.id}")
        holds_response = await self.session.get(
            f"{self.url}/post.json?api_version=2&tags=parent:{post.id}%20holds:true"
        )
        holds_response.raise_for_status()

        no_holds_response = await self.session.get(
            f"{self.url}/post.json?api_version=2&tags=parent:{post.id}"
        )
        no_holds_response.raise_for_status()

        holds_result = Posts.model_validate(holds_response.json())
        result = Posts.model_validate(no_holds_response.json())
        result.posts.extend(holds_result.posts)
        return result.posts

    async def rss(self) -> list[Post]:
        logger.info("Getting RSS")
        response = await self.session.get(f"{self.url}/post/popular_recent")
        response.raise_for_status()
        posts = await self.parse_pop_recent(response.text)
        return posts

    async def extract_post(self, html: str) -> list[str]:
        pattern = r"Post\.register\((?P<json>\{.*?\})\)"
        matches = re.finditer(pattern, html)
        return [match.group("json") for match in matches]

    async def parse_pop_recent(self, html: str) -> list[Post]:
        posts = []
        json_strings = await self.extract_post(html)

        for json_str in json_strings:
            try:
                post_data = json.loads(json_str)
                post = Post.model_validate(post_data)
                posts.append(post)
            except (json.JSONDecodeError, ValueError) as e:
                # Skip invalid JSON data and continue processing
                logger.warning(f"Failed to parse post JSON: {e}")
                continue

        return posts


async def test_rss():
    client = YandereClient(RSSConfig())
    posts = await client.rss()
    logger.info(f"Found {len(posts)} posts")
    logger.info(f"Posts: {posts}")


async def test_new_1121916():
    client = YandereClient(RSSConfig())
    post = await client.new_post(1121916)
    logger.info(f"Post: {post}")

    assert post.has_children
    assert post.created_at == 1695383691
    assert post.rating == "q"

    children = await client.get_children(post)
    logger.info(f"Children: {children}")
    assert len(children) == 2
    assert children[0].id == 1121917


if __name__ == "__main__":
    import asyncio

    asyncio.run(test_rss())
