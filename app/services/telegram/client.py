import httpx

from app.configs import TelegramConfig


class TelegramClient:
    def __init__(self, config: TelegramConfig):
        self.token = config.token
        self.chat_id = config.chat_id
        self.channel_id = config.channel_id
        self.session = httpx.AsyncClient()

    async def send_message(self, message: str):
        await self.session.post(
            f"https://api.telegram.org/bot{self.token}/sendMessage",
            json={"chat_id": self.chat_id, "text": message},
        )
