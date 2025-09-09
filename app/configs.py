import os

from dotenv import load_dotenv

load_dotenv()


class RedisConfig:
    def __init__(self):
        self.url = os.getenv("REDIS_URL", "redis://localhost:6379")


class TelegramConfig:
    def __init__(self):
        self.token = os.getenv("TELEGRAM_TOKEN")
        self.chat_id = os.getenv("TELEGRAM_CHAT_ID")
        self.channel_id = os.getenv("TELEGRAM_CHANNEL_ID")


class RSSConfig:
    def __init__(self):
        self.url = os.getenv("RSS_URL", "https://yande.re")
        self.threshold = int(os.getenv("RSS_THRESHOLD", 40))


class Config:
    def __init__(self):
        self.log_level = os.getenv("LOG_LEVEL", "INFO")
        self.redis = RedisConfig()
        self.telegram = TelegramConfig()
        self.rss = RSSConfig()
