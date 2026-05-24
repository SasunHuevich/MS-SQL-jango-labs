from django.http import JsonResponse
from django.shortcuts import redirect
from django.urls import reverse

from .auth import get_game_user

EXEMPT_PREFIXES = (
    '/login/',
    '/logout/',
    '/register/',
    '/admin/',
    '/static/',
)


class GameLoginRequiredMiddleware:
    """Редирект неавторизованных пользователей на страницу входа."""

    def __init__(self, get_response):
        self.get_response = get_response

    def __call__(self, request):
        path = request.path

        if any(path.startswith(prefix) for prefix in EXEMPT_PREFIXES):
            return self.get_response(request)

        if get_game_user(request):
            return self.get_response(request)

        login_url = reverse('login')
        next_path = request.get_full_path()

        if path.startswith('/api/'):
            return JsonResponse(
                {'detail': 'Требуется авторизация. Войдите через /login/'},
                status=401,
            )

        if next_path and next_path != login_url:
            return redirect(f'{login_url}?next={next_path}')

        return redirect(login_url)
