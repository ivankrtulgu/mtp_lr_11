# Способ 1: Docker Compose (deploy с лимитами и резервированием)
# Запуск:
#   docker compose up -d
#
# Блок deploy в docker-compose.yml выше содержит:
#   limits:     memory=512M, cpus=0.5  — жёсткие ограничения
#   reservations: memory=256M, cpus=0.25 — гарантированное резервирование

# --------------------------------------------------------------------------------

# Способ 2: Команда docker run с флагами ограничений ресурсов
# Запуск:
docker run -d \
  --name mtp-limited-app \
  --memory 512m \
  --memory-reservation 256m \
  --cpus 0.5 \
  --cpu-shares 512 \
  -p 8000:8000 \
  -e APP_HOST=0.0.0.0 \
  -e APP_PORT=8000 \
  -e DATABASE_URL=sqlite:///./sqlalchemy.db \
  --restart unless-stopped \
  mtp-lab11-app:latest

# --------------------------------------------------------------------------------

# Проверка лимитов через docker stats:
#   docker stats mtp-limited-app
#
# Вывод покажет:
#   MEM USAGE / LIMIT — текущее использование RAM / 512MiB
#   CPU % — текущая загрузка CPU (не превысит ~50% одного ядра)
#
# Проверка детальной конфигурации:
#   docker inspect mtp-limited-app --format='{{.HostConfig.Memory}}'     → 536870912 (512MB в байтах)
#   docker inspect mtp-limited-app --format='{{.HostConfig.NanoCpus}}'   → 500000000 (0.5 CPU в nanocpus)
