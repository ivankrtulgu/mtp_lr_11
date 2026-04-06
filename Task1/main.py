import os
import signal
import logging
import sys
from contextlib import asynccontextmanager
from fastapi import FastAPI, Depends, HTTPException
from fastapi.responses import JSONResponse
from sqlalchemy import create_engine, Column, Integer, String
from sqlalchemy.orm import sessionmaker, declarative_base, Session
from pythonjsonlogger.json import JsonFormatter
from starlette.middleware.base import BaseHTTPMiddleware
from starlette.requests import Request
from starlette.responses import Response
import uvicorn

# Configuration from environment
APP_HOST = os.getenv("APP_HOST", "0.0.0.0")
APP_PORT = int(os.getenv("APP_PORT", "8000"))
DATABASE_URL = os.getenv("DATABASE_URL", "sqlite:///./sqlalchemy.db")

# Setup JSON logger
logger = logging.getLogger("app")
logger.setLevel(logging.INFO)
log_handler = logging.StreamHandler(sys.stdout)
formatter = JsonFormatter(
    "%(asctime)s %(levelname)s %(message)s %(name)s",
    datefmt="%Y-%m-%dT%H:%M:%S"
)
log_handler.setFormatter(formatter)
logger.addHandler(log_handler)

# SQLAlchemy setup
engine = create_engine(
    DATABASE_URL,
    connect_args={"check_same_thread": False}
)
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
Base = declarative_base()

class User(Base):
    __tablename__ = "users"
    id = Column(Integer, primary_key=True, index=True)
    name = Column(String, unique=True, index=True, nullable=False)

# JSON Logging Middleware
class LoggingMiddleware(BaseHTTPMiddleware):
    async def dispatch(self, request: Request, call_next) -> Response:
        response = await call_next(request)
        logger.info(
            "HTTP Request",
            extra={
                "method": request.method,
                "path": request.url.path,
                "status_code": response.status_code,
            }
        )
        return response

def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()

@asynccontextmanager
async def lifespan(app: FastAPI):
    # Startup
    logger.info("Application starting up")
    Base.metadata.create_all(bind=engine)
    logger.info("Database tables created")
    
    # Setup signal handlers for graceful shutdown
    loop = None
    try:
        import asyncio
        loop = asyncio.get_event_loop()
    except RuntimeError:
        loop = asyncio.new_event_loop()
        asyncio.set_event_loop(loop)
    
    def handle_signal(signum, frame):
        logger.info(f"Received signal {signum}")
        raise SystemExit(0)
    
    signal.signal(signal.SIGTERM, handle_signal)
    signal.signal(signal.SIGINT, handle_signal)
    
    yield
    
    # Shutdown
    logger.info("Shutting down gracefully")
    SessionLocal.close_all()
    logger.info("Shutdown complete")

app = FastAPI(
    title="FastAPI SQLAlchemy App",
    version="0.1.0",
    lifespan=lifespan
)

app.add_middleware(LoggingMiddleware)

@app.get("/")
async def root():
    return {"message": "ok", "version": "0.1.0"}

@app.get("/health")
async def health():
    return {"status": "healthy"}

@app.get("/users")
async def read_users(db: Session = Depends(get_db)):
    try:
        users = db.query(User).all()
        return [{"id": u.id, "name": u.name} for u in users]
    except Exception as e:
        logger.error(f"Error reading users: {str(e)}")
        raise HTTPException(status_code=500, detail="Internal server error")

@app.post("/users/{name}")
async def create_user(name: str, db: Session = Depends(get_db)):
    try:
        existing_user = db.query(User).filter(User.name == name).first()
        if existing_user:
            raise HTTPException(status_code=400, detail="User already exists")
        
        user = User(name=name)
        db.add(user)
        db.commit()
        db.refresh(user)
        return {"id": user.id, "name": user.name}
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Error creating user: {str(e)}")
        raise HTTPException(status_code=500, detail="Internal server error")

if __name__ == "__main__":
    uvicorn.run(
        "main:app",
        host=APP_HOST,
        port=APP_PORT,
        log_level="info"
    )
