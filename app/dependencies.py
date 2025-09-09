from functools import lru_cache

from fastapi import Request

from app.configs import Config
from app.services.redis import RedisClient
from app.services.telegram import TelegramClient
from app.services.yandere import YandereClient


@lru_cache
def get_config():
    return Config()


def get_redis_client(request: Request) -> RedisClient:
    return request.app.state.redis_client


def get_telegram_client(request: Request) -> TelegramClient:
    return request.app.state.telegram_client


def get_yandere_client(request: Request) -> YandereClient:
    return request.app.state.yandere_client
