from pydantic import BaseModel

class User(BaseModel):
    id: int
    name: str
    email: str

# mock database
users = [
    User(id=1, name="Alice", email="alice@example.com"),
    User(id=2, name="Bob", email="bob@example.com")
]
