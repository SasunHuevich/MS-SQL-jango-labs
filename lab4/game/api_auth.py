from rest_framework.authentication import BaseAuthentication
from rest_framework.exceptions import AuthenticationFailed

from .auth import SESSION_USER_KEY, user_can_read
from .models import GameUser


class GameUserSessionAuthentication(BaseAuthentication):
    def authenticate(self, request):
        user_id = request.session.get(SESSION_USER_KEY)
        if not user_id:
            return None
        user = GameUser.objects.select_related('role').filter(pk=user_id).first()
        if not user:
            return None
        if not user_can_read(user):
            raise AuthenticationFailed('Роль не имеет доступа к API.')
        return (user, None)
