from fastapi import APIRouter, Depends
from pydantic import BaseModel

from app.dependencies import get_redis_client
from app.services.redis import RedisClient

router = APIRouter()


class Request(BaseModel):
    post_ids: list[int]


class Data(BaseModel):
    post_id: int
    sent: bool


class Response(BaseModel):
    data: list[Data]


@router.get("/sent")
async def sent(
    request: Request,
    redis_client: RedisClient = Depends(get_redis_client),
):
    response = Response(data=[])
    for post_id in request.post_ids:
        if redis_client.already_sent_post(post_id):
            response.data.append(Data(post_id=post_id, sent=True))
        else:
            response.data.append(Data(post_id=post_id, sent=False))
    return response
