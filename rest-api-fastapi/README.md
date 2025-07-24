# 🚀 REST API with FastAPI

```markdown
This project is a lightweight, production-ready REST API built using **FastAPI** and **Docker**, following modern best practices. It's ideal for freelance projects requiring clean, scalable, and testable backend APIs.
```

## 📦 Features

- ✅ CRUD-ready architecture
- ✅ Modular folder structure
- ✅ Dockerized for easy deployment
- ✅ Built-in database mock (can be extended to PostgreSQL)
- ✅ Includes JWT-ready structure (easily pluggable)

## 🧰 Tech Stack

- **Python 3.11**
- **FastAPI**
- **Pydantic**
- **Uvicorn**
- **Docker**

## 🗂️ Project Structure

rest-api-fastapi/
├── app/
│ ├── main.py # Entry point
│ ├── routes.py # API routes
│ ├── models.py # Pydantic models and mock DB
│ └── database.py # DB setup (placeholder)
├── Dockerfile
├── requirements.txt
└── README.md

## 🚀 Getting Started

### ▶️ Run Locally (With Python)

```bash
# Create virtual environment (optional)
python -m venv venv
source venv/bin/activate   # On Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Run the API
uvicorn main:app --reload
```

- Visit: http://localhost:8000/docs for Swagger UI

### 🐳 Run with Docker

```bash
docker build -t fastapi-app .
docker run -p 8000:8000 fastapi-app
```

### 🔐 Sample Endpoint

```bash
GET /users
```

#### Response:

```json
[
  {
    "id": 1,
    "name": "Alice",
    "email": "alice@example.com"
  },
  {
    "id": 2,
    "name": "Bob",
    "email": "bob@example.com"
  }
]
```

## 📂 Ideal For

- Freelance API projects
- Microservice boilerplate
- Rapid prototyping of backend services
- Integration with PostgreSQL, Redis, or Auth layers

## 🧑‍💻 Author

- Arpit Srivastava
- Senior Python/Golang Developer | Cloud & Data Platforms
- 📧 <arpitusrivastava@hotmail.com>
- 🌐 [Freelancer.com Profile](https://www.freelancer.com/u/arpitusrivastava?sb=t)
- 🌐 [Fiverr.com Profile](https://www.fiverr.com/s/bdaYvGY)
- 🌐 [Upwork.com Profile](https://www.upwork.com/freelancers/~01bdfb5647cd44913c?mp_source=share)

## 📝 License

This project is open source under the MIT License.
