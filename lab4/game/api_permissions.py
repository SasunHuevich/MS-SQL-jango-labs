from rest_framework.permissions import BasePermission, SAFE_METHODS

from .auth import user_can_write


class IsGameUserAuthenticated(BasePermission):
    def has_permission(self, request, view):
        user = getattr(request, 'user', None)
        return user is not None and hasattr(user, 'role')


class IsAdminOrModerator(BasePermission):
    def has_permission(self, request, view):
        user = getattr(request, 'user', None)
        return user is not None and user_can_write(user)


class ReadOnlyOrAdminModerator(BasePermission):
    def has_permission(self, request, view):
        user = getattr(request, 'user', None)
        if user is None or not hasattr(user, 'role'):
            return False
        if request.method in SAFE_METHODS:
            return True
        return user_can_write(user)


class CommentAPIPermission(BasePermission):
    def has_permission(self, request, view):
        user = getattr(request, 'user', None)
        if user is None or not hasattr(user, 'role'):
            return False
        if request.method in SAFE_METHODS or request.method == 'POST':
            return True
        return user_can_write(user)

    def has_object_permission(self, request, view, obj):
        if request.method in SAFE_METHODS:
            return True
        if user_can_write(request.user):
            return True
        return obj.author_id == request.user.pk
