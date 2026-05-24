from functools import wraps

from django.shortcuts import redirect

from .models import GameUser

SESSION_USER_KEY = 'game_user_id'

ROLE_ADMIN = 'admin'
ROLE_MODERATOR = 'moderator'
ROLE_PREMIUM = 'premium'
ROLE_USER = 'user'

WRITE_ROLES = {ROLE_ADMIN, ROLE_MODERATOR}
READ_ROLES = WRITE_ROLES | {ROLE_PREMIUM, ROLE_USER}


def get_game_user(request):
    user_id = request.session.get(SESSION_USER_KEY)
    if not user_id:
        return None
    return GameUser.objects.select_related('role').filter(pk=user_id).first()


def login_game_user(request, user):
    request.session[SESSION_USER_KEY] = user.pk
    request.session.save()


def logout_game_user(request):
    request.session.flush()


def user_role_name(user):
    if user and user.role:
        return user.role.name
    return None


def user_can_write(user):
    return user_role_name(user) in WRITE_ROLES


def user_can_read(user):
    if not user:
        return False
    role = user_role_name(user)
    if role is None:
        return True
    return role in READ_ROLES


def authenticate_game_user(username, password):
    if not username or not password:
        return None
    return (
        GameUser.objects.select_related('role')
        .filter(username__iexact=username.strip(), password=password)
        .first()
    )


def game_login_required(view_func):
    @wraps(view_func)
    def wrapper(request, *args, **kwargs):
        if not get_game_user(request):
            return redirect('login')
        return view_func(request, *args, **kwargs)

    return wrapper


class GameLoginRequiredMixin:
    def dispatch(self, request, *args, **kwargs):
        if not get_game_user(request):
            return redirect('login')
        return super().dispatch(request, *args, **kwargs)

    def get_game_user(self):
        return get_game_user(self.request)


class GameWritePermissionMixin(GameLoginRequiredMixin):
    def dispatch(self, request, *args, **kwargs):
        if not get_game_user(request):
            return redirect('login')
        if not user_can_write(get_game_user(request)):
            from django.core.exceptions import PermissionDenied
            raise PermissionDenied('Недостаточно прав для изменения данных.')
        return super(GameLoginRequiredMixin, self).dispatch(request, *args, **kwargs)
