from .auth import get_game_user, user_can_write


def game_user(request):
    user = get_game_user(request)
    return {
        'current_user': user,
        'can_write': user_can_write(user) if user else False,
    }
