from pydantic import BaseModel


class Post(BaseModel):
    id: int
    tags: str | None
    created_at: int | None
    updated_at: int | None
    creator_id: int | None
    approver_id: int | None
    author: str | None
    change: int | None
    source: str | None
    score: int | None
    md5: str | None
    file_size: int | None
    file_ext: str | None
    file_url: str | None
    is_shown_in_index: bool | None
    preview_url: str | None
    preview_width: int | None
    preview_height: int | None
    actual_preview_width: int | None
    actual_preview_height: int | None
    sample_url: str
    sample_width: int | None
    sample_height: int | None
    sample_file_size: int | None
    jpeg_url: str | None
    jpeg_width: int | None
    jpeg_height: int | None
    jpeg_file_size: int | None
    rating: str | None
    is_rating_locked: bool | None
    has_children: bool | None
    parent_id: int | None
    status: str | None
    is_pending: bool | None
    width: int | None
    height: int | None
    is_held: bool | None
    frames_pending_string: str | None
    frames_pending: list[int] | None
    frames_string: str | None
    frames: list[int] | None
    is_note_locked: bool | None
    last_noted_at: int | None
    last_commented_at: int | None


class Posts(BaseModel):
    posts: list[Post]
