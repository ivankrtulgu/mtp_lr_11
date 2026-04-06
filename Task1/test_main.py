import pytest
from fastapi.testclient import TestClient
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker
from sqlalchemy.pool import StaticPool

from main import app, Base, get_db

# Test database setup
SQLALCHEMY_DATABASE_URL = "sqlite://"

engine = create_engine(
    SQLALCHEMY_DATABASE_URL,
    connect_args={"check_same_thread": False},
    poolclass=StaticPool,
)

TestingSessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)


def override_get_db():
    try:
        db = TestingSessionLocal()
        yield db
    finally:
        db.close()


app.dependency_overrides[get_db] = override_get_db


@pytest.fixture(autouse=True)
def setup_database():
    Base.metadata.create_all(bind=engine)
    yield
    Base.metadata.drop_all(bind=engine)


client = TestClient(app)


class TestRootEndpoint:
    def test_root_returns_correct_response(self):
        response = client.get("/")
        assert response.status_code == 200
        data = response.json()
        assert data["message"] == "ok"
        assert data["version"] == "0.1.0"


class TestHealthEndpoint:
    def test_health_returns_healthy_status(self):
        response = client.get("/health")
        assert response.status_code == 200
        data = response.json()
        assert data["status"] == "healthy"


class TestUsersEndpoints:
    def test_get_users_empty_list(self):
        response = client.get("/users")
        assert response.status_code == 200
        data = response.json()
        assert isinstance(data, list)
        assert len(data) == 0

    def test_create_user_success(self):
        response = client.post("/users/Alice")
        assert response.status_code == 200
        data = response.json()
        assert data["name"] == "Alice"
        assert "id" in data
        assert isinstance(data["id"], int)

    def test_create_duplicate_user_fails(self):
        response1 = client.post("/users/Bob")
        assert response1.status_code == 200

        response2 = client.post("/users/Bob")
        assert response2.status_code == 400
        data = response2.json()
        assert "detail" in data

    def test_get_users_after_creation(self):
        client.post("/users/Charlie")
        client.post("/users/Diana")

        response = client.get("/users")
        assert response.status_code == 200
        data = response.json()
        assert len(data) == 2
        names = [user["name"] for user in data]
        assert "Charlie" in names
        assert "Diana" in names

    def test_create_user_with_empty_name(self):
        # POST /users/ redirects to /users (307) which returns 405 
        # because the route expects a {name} parameter
        response = client.post("/users/")
        assert response.status_code in [307, 405]
