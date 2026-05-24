# Lab 4 — Django-клиент для MSSQL

Клиент-серверное приложение (паттерн MTV) для базы данных из `init-db/init.sql`.

## Первый запуск

Выполнять из **корня репозитория** (где лежит `docker-compose.yaml`).

### 1. Запуск контейнеров

```bash
bash lab4/run.sh
```

Скрипт:
- собирает образы `db` и `django`;
- поднимает MSSQL;
- применяет `init.sql` (схема и роли);
- запускает Django и выполняет `migrate` (служебные таблицы Django).

### 2. Создание администратора приложения

Регистрация на сайте всегда создаёт пользователя с ролью **`user`** (только просмотр и комментарии).

Первого **admin** нужно создать командой:

```bash
docker compose exec django python manage.py create_game_user ВАШ_ЛОГИН ВАШ_ПАРОЛЬ --role=admin
```

Пример:

```bash
docker compose exec django python manage.py create_game_user admin mypassword --role=admin
```

Другие роли: `moderator`, `premium`, `user`.

### 3. Вход на сайт

- Приложение: http://localhost:8000/login/
- Регистрация (только роль `user`): http://localhost:8000/register/

### 4. Django Admin (опционально)

Отдельная учётная запись для `/admin/` (таблица `auth_user`, не `users`):

```bash
docker compose exec django python manage.py createsuperuser
```

Затем: http://localhost:8000/admin/

## Повседневный запуск

Если контейнеры уже настроены:

```bash
docker compose up -d db django
```

## Роли и права

| Роль        | Просмотр | Комментарии | Создание / изменение / удаление |
|-------------|----------|-------------|----------------------------------|
| user        | да       | да          | нет                              |
| premium     | да       | да          | нет                              |
| moderator   | да       | да          | да                               |
| admin       | да       | да          | да                               |

## REST API

Эндпоинты доступны по префиксу `/api/` (нужна авторизация через сессию: сначала войти на `/login/` в том же браузере).

- `/api/quests/`
- `/api/traders/`
- `/api/maps/`
- `/api/comments/`
- `/api/users/` — только admin и moderator

## Структура

```
lab4/
├── manage.py
├── django_back/     # настройки проекта
├── game/            # приложение (модели, views, API)
├── templates/       # HTML-шаблоны
├── Dockerfile
└── run.sh           # скрипт первого/полного запуска
```
