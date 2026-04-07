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


## Задание 9: Ограничить ресурсы (CPU, память) для контейнеров.

### Промпт 1
**Инструмент:** Qwen Code
**Промпт:**
```
Выполняй задание в новой папке @Task9/ Действуй как эксперт по Docker. У меня есть задание по контейнеризации Python-приложения (@Task1/). Мне необходимо внедрить жесткие ограничения на потребление ресурсов для этого контейнера. 

Требования:
Ограничь использование оперативной памяти (RAM) до 512MB.
Ограничь использование процессора (CPU) до 0.5 (50% одного ядра).

Реализуй это двумя способами:
— Обнови мой docker-compose.yml, добавив блок deploy с лимитами и резервированием.
— Напиши готовую команду docker run со всеми необходимыми флагами.

Напиши только код и краткое пояснение, как проверить примененные лимиты через docker stats.
```

**Результат:** Создана конфигурация контейнеризации с жёсткими ограничениями ресурсов для Python/FastAPI приложения. Реализовано два способа: обновлённый docker-compose.yml с блоком deploy.resources (лимиты: 512M RAM, 0.5 CPU; резервирование: 256M RAM, 0.25 CPU) и готовая команда docker run с флагами --memory, --memory-reservation, --cpus. Проверка лимитов выполняется через docker stats и docker inspect.

### Итого
- **Количество промптов:** 1
- **Что пришлось исправлять вручную:** Пришлось билдить контейнер docker compose up -d, затем вручную с помощью команды docker stats mtp-limited-app смотреть статистику работы контейнера и потребление ресурсов, были получены следующие результаты:
```
CONTAINER ID   NAME              CPU %     MEM USAGE / LIMIT   MEM %     NET I/O         BLOCK I/O         PIDS
2f2d86978ba6   mtp-limited-app   0.17%     64.1MiB / 512MiB    12.52%    1.75kB / 126B   49.9MB / 65.5kB   1
```
Далее, с помощью команд docker inspect mtp-limited-app --format='{{.HostConfig.Memory}}' и docker inspect mtp-limited-app --format='{{.HostConfig.NanoCpus}}', были получены следующие результаты:
```
(venv) PS D:\Workspace\MTP_lab11\proj\Task9> docker inspect mtp-limited-app --format='{{.HostConfig.Memory}}' 
536870912
(venv) PS D:\Workspace\MTP_lab11\proj\Task9> docker inspect mtp-limited-app --format='{{.HostConfig.NanoCpus}}'
500000000
(venv) PS D:\Workspace\MTP_lab11\proj\Task9> 
```
- **Время:** ~10 мин


## Задание 1(Hard): Собрать Go-приложение с поддержкой статической компиляции и запустить в scratch образе.

### Промпт 1
**Инструмент:** Qwen Code
**Промпт:**
```
Техническое задание: Собрать Go-приложение с поддержкой статической компиляции и запустить в scratch образе. 

Выполнять в папке @Task_hard1/ . Необходимо реализовать микросервис на языке Go, упакованный в максимально оптимизированный Docker-контейнер.

Требования к коду (Go):
1. Функционал: Простой HTTP-сервер с эндпоинтами `/health` (возвращает JSON `{"status": "ok"}`).
2. Конфигурация: Все настройки (порт, уровень логирования) должны считываться ИСКЛЮЧИТЕЛЬНО из переменных окружения (используй пакет `os`).
3. Тестирование: Напиши unit-тесты в файле `main_test.go`, проверяющие корректность работы HTTP-обработчиков.

Требования к Docker (Multi-stage + Scratch):
1. Этап 1 (Builder):
   - Использовать `golang:1.22-alpine`.
   - Рабочая директория `/build`.
   - Обязательно: Запуск тестов командой `RUN go test -v ./...` внутри Dockerfile. Если тесты не прошли, сборка должна прерваться.
   - Статическая компиляция: Сборка бинарника с флагами `CGO_ENABLED=0 GOOS=linux` и флагами линковщика `-s -w` (для удаления отладочной информации и уменьшения размера).
2. Этап 2 (Final):
   - Использовать пустой образ `scratch`.
   - Скопировать из билдера только скомпилированный бинарный файл и CA-сертификаты (`/etc/ssl/certs/ca-certificates.crt`).
   - Настроить `USER` (создать не-root пользователя в билдере и перенести его), чтобы приложение не запускалось от имени суперпользователя.
   - Указать `ENTRYPOINT`.

Ожидаемый результат:
1. Исходный код `main.go` и `main_test.go`.
2. Оптимизированный `Dockerfile`.
3. Файл `.env` с примерами настроек.
4. Команда для сборки и команда для запуска с пробросом переменных окружения через `--env-file`.
5. Инструкция, как проверить размер итогового образа (должен быть < 15MB).
```

**Результат:** Создан полноценный, готовый к эксплуатации Go-микросервис с многоэтапной сборкой Docker (scratch образ), статической компиляцией (CGO_ENABLED=0, -ldflags="-s -w"), запуском от non-root пользователя и unit-тестами. Все 2 теста прошли успешно. Размер статического бинарника составил ~5.7 MB.

### Итого
- **Количество промптов:** 1
- **Что пришлось исправлять вручную:** Unit-тесты были проверены командой go test -v (все 2 прошли), статический бинарник собран вручную через set CGO_ENABLED=0 && set GOOS=linux && go build -ldflags="-s -w" -o app.exe ./main.go — получен размер 5 968 034 байт (~5.7 MB). После сборки образа была выполнена команда docker images go-microservice для проверки информации о Docker-образе, были получены следующие результаты:
```
PS D:\Workspace\MTP_lab11\proj\Task_hard1> docker images go-microservice
                                                                                                                                        i Info →   U  In Use
IMAGE                    ID             DISK USAGE   CONTENT SIZE   EXTRA
go-microservice:latest   5b43c31cd940       7.48MB         2.29MB    U 
PS D:\Workspace\MTP_lab11\proj\Task_hard1> 
```
- **Время:** ~25 мин


## Задание 3(Hard): Настроить CI/CD, который собирает и пушит образы для всех трёх языков.

### Промпт 1
**Инструмент:** Qwen Code
**Промпт:**
```
Техническое задание: Настройка CI/CD пайплайна для мультиязычного монорепозитория (Python, Go, Rust)

Роль: Ты — Lead DevOps инженер.
Задача: Создать единый GitHub Actions Workflow (`main.yml`), который автоматически собирает, тестирует и пушит Docker-образы в Docker Hub для трёх разных сервисов.

Структура проекта:
- `@Task1/` (Python/FastAPI)
- `@Task_hard1/` (Go - статическая сборка в scratch)
- `@Task3/` (Rust - оптимизированная сборка)

Требования к Pipeline (CI/CD):
1. Триггеры: Запуск при каждом `push` в ветку `main` или при создании `pull_request`.
2. Безопасность (Secrets): Использовать GitHub Secrets для хранения `DOCKERHUB_USERNAME` и `DOCKERHUB_TOKEN`. Никаких открытых данных в коде.
3. Оптимизация (Matrix Strategy): Реализовать сборку через `strategy: matrix`, чтобы минимизировать дублирование кода и запускать сборки параллельно.
4. Этапы для каждого языка:
   - Lint/Test: Перед сборкой образа запустить тесты (pytest для Python, go test для Go, cargo test для Rust). Если тесты упали — пайплайн стопится.
   - Docker Build & Push: Использовать `docker/build-push-action@v5`. 
   - Тегирование: Использовать два тега: `latest` и короткий SHA коммита (`${{ github.sha }}`).
5. Специфика сборки:
   - Для Go: Убедиться, что билд идет в контексте multi-stage.
   - Для Rust: Настроить кэширование зависимостей (`actions/cache`), чтобы сборка не занимала 20 минут каждый раз.
   - Для Python: Проверить наличие `.dockerignore`, чтобы не копировать `venv` в образ.

Ожидаемый результат:
1. Полный код файла `.github/workflows/main.yml`.
2. Список секретов, которые нужно добавить в настройки GitHub (Settings -> Secrets).
3. Инструкция, как проверить статус сборки и где найти готовые образы в Docker Hub.
4. Краткое пояснение: как этот пайплайн помогает соблюсти "чистоту" кода и гарантировать, что в продакшн (Docker Hub) попадет только протестированный бинарник.

Формат вывода: Только структурированный YAML код и краткий чек-лист по настройке.
```

**Результат:** Создан единый GitHub Actions Workflow (.github/workflows/main.yml) для мультиязычного монорепозитория с матричной стратегией (strategy:  matrix), обеспечивающий параллельную сборку, тестирование и публикацию Docker-образов в Docker Hub для трёх сервисов: Python/FastAPI (Task1/), Go (Task_hard1/), Rust (Task3/). Реализованы этапы lint/test перед сборкой (flake8 / go vet / cargo clippy, pytest / go test -race / cargo test), при провале любого теста пайплайн останавливается (fail-fast: true). Docker-образы пушатся с двумя тегами: latest и короткий SHA коммита (${{ github.sha }}  | cut -c1-7). Для Go подтверждена multi-stage сборка в scratch-образ, для Rust настроено кэширование зависимостей через actions/cache с ключом по Cargo.lock, для Python проверено наличие .dockerignore (исключает __pycache__, .pytest_cache, *.db). Безопасность обеспечена через GitHub Secrets (DOCKERHUB_USERNAME, DOCKERHUB_TOKEN) — никаких открытых кредов в коде. Push образов происходит только при push в main; при pull_request выполняются только lint/test/build без push.

### Итого
- **Количество промптов:** 1
- **Что пришлось исправлять вручную:** 
1. Добавил components: clippy в шаг установки Rust.
2. Была выполнена настройка GitHub Secrets в репозитории: Settings → Secrets and variables → Actions → New repository secret. Добавлены два секрета:
  DOCKERHUB_USERNAME (имя пользователя Docker Hub) и DOCKERHUB_TOKEN (Personal Access Token, сгенерированный на Docker Hub → Account Settings → Security → Access Tokens → Generate New Token).
3. Был выполнен пуш изменений в репозиторий командой git add . && git commit -m "add CI/CD pipeline" && git push origin main, после чего в GitHub → вкладка Actions была запущена сборка пайплайна CI/CD Pipeline.
4. Визуально подтверждено, что все три матричных джоба (python, go, rust) запустились параллельно и прошли успешно: lint (flake8 / go vet / cargo clippy), тесты (pytest / go test -race / cargo test), Docker Build & Push.
5. После добавления секретов был запущен тестовый пуш в ветку main для проверки корректности авторизации docker/login-action@v3 — сборка прошла успешно, образы запушены в Docker Hub.
6. На вкладке Docker Hub → Repositories проверено наличие образов с тегами latest и коротким SHA коммита.
- **Время:** ~35 мин
