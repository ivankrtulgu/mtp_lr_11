# Prompt Log

## Задание 1: Написать Dockerfile для Python-приложения (из ЛР №6).

### Промпт 1
**Инструмент:** Qwen Code
**Промпт:**

Задание выполнять в папке @Task1/ Модифицируй существующий проект FastAPI+SQLAlchemy+Docker (из лабораторной работы №6) из файла @Task1/09_hard.py без плейсхолдеров и //TODO. Код должен быть готов к запуску.

1. Приложение (main.py):
- Стек: FastAPI, Uvicorn, SQLAlchemy (SQLite), python-json-logger.
- Эндпоинты: GET / (message: ok, v0.1.0), GE T /health (healthy), GET /users, POST /users/{name}.
- Конфиг: APP_HOST, APP_PORT(int), DATABASE_URL через os.getenv.
- Логи: JSON-формат для всех запросов (метод, путь, статус).
- Lifespan: корректный SIGTERM/SIGINT, закрытие сессий БД, лог "Shutdown complete".
- БД: SQLite (check_same_thread: False), автосоздание таблиц при старте.

2. Тесты (test_main.py):
- Pytest + httpx. Интеграционная проверка всех эндпоинтов через TestClient. Статус-коды, структура JSON.

3. Docker:
- Multi-stage (builder + runtime на python:3.12-slim). 
- Builder: COPY requirements.txt -> RUN pip install --prefix=/install.
- Runtime: COPY /install, USER 10001:10001, создание sqlalchemy.db с chown.
- Healthcheck: через urllib.request (GET /health).
- .dockerignore: __pycache__, .git, .env, *.db, Dockerfile.

4. Вывод:
Листинги: requirements.txt (fixed versions), main.py, test_main.py, Dockerfile, .dockerignore. 
Инструкции: команды для pytest, docker build, docker run, curl.
Комментарий: кэширование слоев, non-root безопасность, graceful shutdown.

ЗАПРЕЩЕНО: COPY . ., запуск от root, пропуски логики.

**Результат:** Создан полноценный, готовый к эксплуатации проект на FastAPI с JSON-логированием, интеграцией SQLAlchemy, многоэтапной сборкой Docker с защитой non-root и полным покрытием тестами. Тем не менее, тест test_create_user_with_empty_name завершился ошибкой из-за некорректной обработки слеша в URL и возврата статуса 405 Method Not Allowed вместо ожидаемого 422. Также в процессе выполнения было зафиксировано 1 предупреждение (warning), связанное с использованием устаревшего пути импорта в библиотеке python-json-logger.

### Промт 2
**Инструмент:** Qwen Code
**Промпт:**
Требуется исправить проект, чтобы ошибки все тесты проходили без ошибок. Ниже представлен лог ошибок:
```
(venv) PS D:\Workspace\MTP_lab11\proj\Task1> pytest test_main.py -v
================================================================= test session starts =================================================================
platform win32 -- Python 3.11.0, pytest-8.3.4, pluggy-1.6.0 -- D:\Workspace\MTP_lab11\proj\Task1\venv\Scripts\python.exe
cachedir: .pytest_cache
rootdir: D:\Workspace\MTP_lab11\proj\Task1
plugins: anyio-4.13.0
collected 7 items                                                                                                                                      

test_main.py::TestRootEndpoint::test_root_returns_correct_response PASSED                                                                        [ 14%]
test_main.py::TestHealthEndpoint::test_health_returns_healthy_status PASSED                                                                      [ 28%] 
test_main.py::TestUsersEndpoints::test_get_users_empty_list PASSED                                                                               [ 42%]
test_main.py::TestUsersEndpoints::test_create_user_success PASSED                                                                                [ 57%] 
test_main.py::TestUsersEndpoints::test_create_duplicate_user_fails PASSED                                                                        [ 71%]
test_main.py::TestUsersEndpoints::test_get_users_after_creation PASSED                                                                           [ 85%]
test_main.py::TestUsersEndpoints::test_create_user_with_empty_name FAILED                                                                        [100%]

====================================================================== FAILURES ======================================================================= 
_________________________________________________ TestUsersEndpoints.test_create_user_with_empty_name _________________________________________________ 

self = <test_main.TestUsersEndpoints object at 0x000001BDC3E6D390>

    def test_create_user_with_empty_name(self):
        response = client.post("/users/")
>       assert response.status_code in [200, 422]
E       assert 405 in [200, 422]
E        +  where 405 = <Response [405 Method Not Allowed]>.status_code

test_main.py:98: AssertionError
---------------------------------------------------------------- Captured stdout call ----------------------------------------------------------------- 
{"asctime": "2026-04-06T21:01:31", "levelname": "INFO", "message": "HTTP Request", "name": "app", "method": "POST", "path": "/users/", "status_code": 307}
{"asctime": "2026-04-06T21:01:31", "levelname": "INFO", "message": "HTTP Request", "name": "app", "method": "POST", "path": "/users", "status_code": 405}
------------------------------------------------------------------ Captured log call ------------------------------------------------------------------ 
INFO     app:main.py:49 HTTP Request
INFO     app:main.py:49 HTTP Request
================================================================== warnings summary =================================================================== 
venv\Lib\site-packages\pythonjsonlogger\jsonlogger.py:11
  D:\Workspace\MTP_lab11\proj\Task1\venv\Lib\site-packages\pythonjsonlogger\jsonlogger.py:11: DeprecationWarning: pythonjsonlogger.jsonlogger has been moved to pythonjsonlogger.json
    warnings.warn(

-- Docs: https://docs.pytest.org/en/stable/how-to/capture-warnings.html
=============================================================== short test summary info =============================================================== 
FAILED test_main.py::TestUsersEndpoints::test_create_user_with_empty_name - assert 405 in [200, 422]
======================================================= 1 failed, 6 passed, 1 warning in 1.02s ========================================================
```

**Результат:** В результате исправлений ошибка 405 была устранена путем синхронизации путей в тестах и роутах FastAPI, а предупреждение библиотеки логирования удалено через обновление путей импорта. Теперь проект проходит все тесты с корректным статусом 422 и демонстрирует чистый вывод логов без системных предупреждений.

### Итого
- **Количество промптов:** 2
- **Что пришлось исправлять вручную:** Пришлось убирать из файла .dockerignore ошибочно добавленный requirements.txt, из-за которого возникала ошибка при сборке контейнера командой docker build -t fastapi-sqlalchemy-app . , код исправленной ошибки: CopyIgnoredFile: Attempting to Copy file "requirements.txt" that is excluded by .dockerignore (line 6).
- **Время:** ~12 мин