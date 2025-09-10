import redis.asyncio as redis
from loguru import logger

from app.configs import RedisConfig


class RedisClient:
    def __init__(self, config: RedisConfig):
        pool = redis.ConnectionPool.from_url(config.url)
        self.client = redis.Redis.from_pool(pool)
        self.expiration_time = 60 * 60 * 24 * 30  # 30 days

    async def ping(self):
        logger.debug("Pinging Redis")
        return await self.client.ping()

    async def close(self):
        logger.debug("Closing Redis connection")
        await self.client.aclose()

    async def already_sent_post(self, post_id: int) -> bool:
        logger.debug(f"Checking if post {post_id} has already been sent")
        return await self.client.get(f"post:{post_id}") is not None

    async def already_sent_posts(self, posts_ids: list[int]) -> bool:
        logger.debug(f"Checking if posts {posts_ids} have already been sent")
        for post_id in posts_ids:
            if not await self.already_sent_post(post_id):
                return False
        return True

    async def set_post_as_sent(self, post_id: int):
        try:
            logger.debug(f"Setting post {post_id} as sent")
            await self.client.set(f"post:{post_id}", "1", ex=self.expiration_time)
        except Exception as e:
            logger.error(f"Error setting post: {e}")

    async def set_posts_as_sent(self, posts_ids: list[int]):
        try:
            logger.debug(f"Setting posts {posts_ids} as sent")
            async with self.client.pipeline() as pipe:
                for post_id in posts_ids:
                    await pipe.set(f"post:{post_id}", "1", ex=self.expiration_time)
                await pipe.execute()
        except Exception as e:
            logger.error(f"Error setting posts: {e}")


async def test_already_sent():
    post_ids = [1242294, 1242295]
    client = RedisClient(RedisConfig())
    logger.info(await client.ping())
    await client.set_posts_as_sent(post_ids)
    result = await client.already_sent_posts(post_ids)
    assert result == True


if __name__ == "__main__":
    import asyncio

    asyncio.run(test_already_sent())

