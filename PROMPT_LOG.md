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


## Задание 3: Написать Dockerfile для Rust-приложения.

### Промпт 1
**Инструмент:** Qwen Code
**Промпт:**
```
Выполнять в папке @Task3/ . Сгенерируй полностью рабочий, готовый к запуску проект на Rust с Docker-контейнеризацией. Запрещены плейсхолдеры, `// TODO`, пропуски логики или комментарии вида "здесь вставьте код". Код должен компилироваться и работать сразу после копирования в пустую директорию.

1. Приложение (Rust)
- Стек: `axum` + `tokio` + `serde`/`serde_json` + `tracing`/`tracing-subscriber`.
- Эндпоинты:
  - `GET /` → `200 OK`, JSON: `{"message": "ok", "version": "0.1.0"}`
  - `GET /health` → `200 OK`, JSON: `{"status": "healthy"}`
- Конфигурация: `APP_HOST` (дефолт `0.0.0.0`), `APP_PORT` (дефолт `3000`), `RUST_LOG` (дефолт `info`). Парсинг через `std::env`, валидация с выводом понятной ошибки при старте.
- Логирование: `tracing` с JSON-форматтером. Все запросы логируются с методом, путём и статус-кодом (middleware или `tower-http::trace`).
- Жизненный цикл: перехват `SIGTERM`/`SIGINT` через `tokio::signal`, graceful shutdown с ожиданием завершения активных запросов (≤10s).
- Обработка ошибок: `Result` вместо `unwrap()`/`expect()` в runtime-пути. Ошибки маппятся в `axum::Response`.
- Версии: зафиксируй совместимые stable-версии зависимостей. `edition = "2021"`.

2. Тесты (обязательно)
- Unit-тесты: проверка парсинга конфигурации и сериализации/десериализации ответов.
- Integration-тесты: запуск `axum::Router` без реальной сети через `tower::ServiceExt::call`. Проверка статус-кодов, `Content-Type: application/json` и структуры тела для `/` и `/health`.
- Все тесты асинхронные (`#[tokio::test]`), запускаются через `cargo test`, не требуют внешних зависимостей или моков.

3. Docker (упрощённый, но production-aligned)
- Multi-stage:
  - `builder`: компиляция в `release`, кэширование зависимостей (сначала `COPY Cargo.toml Cargo.lock ./`, затем `cargo build --locked`, потом `COPY src/ ./src`).
  - `runtime`: `gcr.io/distroless/cc-debian12` или `alpine:3.20`. Копирование только скомпилированного бинарника.
- Безопасность: создание non-root пользователя (`USER 10001:10001`), `HEALTHCHECK --interval=10s --timeout=3s --start-period=5s --retries=3 CMD ["/app", "health"]` (или `curl`/`wget` если образ поддерживает).
- `.dockerignore`: исключи `target/`, `.git/`, `.idea/`, `*.md`, `.env`, `tests/` (если не нужны в сборке).
- Без лишних скриптов, `ENTRYPOINT` wrapper-ов, CI-хуков или сложных `docker-compose`. Минимум, но по стандартам CIS Docker Benchmark.

4. Формат вывода
Предоставь полные листинги файлов:
1. `Cargo.toml`
2. `src/main.rs` (вся логика, роутинг, graceful shutdown, конфигурация, логирование)
3. `tests/integration.rs` (HTTP-тесты)
4. `Dockerfile`
5. `.dockerignore`
6. Команды для:
   - локального запуска тестов
   - сборки Docker-образа
   - запуска контейнера
   - проверки эндпоинтов через `curl`
   - просмотра логов и graceful shutdown
7. Краткий технический комментарий (3-4 пункта): как работает кэширование слоёв, почему выбран distroless/alpine, как реализован graceful shutdown, как тесты изолированы от сети.

Запрещено: `COPY . .` без `.dockerignore`, запуск от `root`, `unwrap()` в runtime, динамическая линковка без явного `RUN` копирования либ, устаревшие флаги Docker, заглушки. Код должен быть самодостаточным, компилироваться на `stable` Rust и запускаться в контейнере без дополнительных шагов.
```
**Результат:** Создан полноценный, готовый к эксплуатации проект на Rust (axum + tokio) с JSON-логированием, graceful shutdown через обработку     SIGTERM/SIGINT, многоэтапной сборкой Docker с использованием distroless-образа и запуском от non-root пользователя (UID 10001), а также полным покрытием тестами. Все 12 тестов прошли успешно: 9 unit-тестов (валидация конфигурации, сериализация/десериализация ответов) и 3 интеграционных теста (проверка эндпоинтов /, /health и обработки 404 через tower::ServiceExt без сетевого стека). В процессе выполнения было зафиксировано 1  предупреждение (warning) о неиспользуемом импорте Span в lib.rs, которое было устранено. Проект компилируется без ошибок в release-профиле и готов к развёртыванию.

### Итого
- **Количество промптов:** 1
- **Что пришлось исправлять вручную:** Пришлось билдить контейнер docker run -d --name axum-service -p 3000:3000 rust-axum-app:latest и проверять запросы к сервису командами curl http://localhost:3000/ curl http://localhost:3000/health.
- **Время:** ~15 мин