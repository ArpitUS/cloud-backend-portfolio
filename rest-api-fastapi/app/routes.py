from fastapi import APIRouter, HTTPException
from app.models import User, users

router = APIRouter()

@router.get("/users", response_model=list[User])
def get_users():
    return users
