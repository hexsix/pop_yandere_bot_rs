import uvicorn
from fastapi import FastAPI
from loguru import logger

from app.api import rss
from app.dependencies import get_config
from app.services.redis import RedisClient
from app.services.telegram import TelegramClient
from app.services.yandere import YandereClient


async def lifespan(app: FastAPI):
    config = get_config()
    logger.level(config.log_level)
    logger.info("Starting up...")
    logger.info(f"Config: {config}")

    app.state.redis_client = RedisClient(config.redis)
    app.state.telegram_client = TelegramClient(config.telegram)
    app.state.yandere_client = YandereClient(config.rss)

    yield

    logger.info("Shutting down...")
    await app.state.redis_client.close()
    await app.state.yandere_client.close()


app = FastAPI(lifespan=lifespan)
app.include_router(rss.router)


@app.get("/health")
async def health():
    return "ok"


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
