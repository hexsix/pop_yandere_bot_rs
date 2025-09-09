import uvicorn
from fastapi import FastAPI
from loguru import logger

from app.dependencies import get_config


async def lifespan(app: FastAPI):
    config = get_config()
    logger.level(config.log_level)
    logger.info("Starting up...")
    logger.info(f"Config: {config}")
    yield


app = FastAPI(lifespan=lifespan)


@app.get("/health")
async def health():
    return "ok"


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
