from django.urls import include, path
from rest_framework.routers import DefaultRouter

from .api_views import (
    CommentViewSet,
    GameMapViewSet,
    GameUserViewSet,
    QuestViewSet,
    TraderViewSet,
)

router = DefaultRouter()
router.register('quests', QuestViewSet, basename='api-quest')
router.register('traders', TraderViewSet, basename='api-trader')
router.register('maps', GameMapViewSet, basename='api-map')
router.register('comments', CommentViewSet, basename='api-comment')
router.register('users', GameUserViewSet, basename='api-user')

urlpatterns = [
    path('', include(router.urls)),
]
