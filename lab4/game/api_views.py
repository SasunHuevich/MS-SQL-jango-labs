from django_filters.rest_framework import DjangoFilterBackend
from rest_framework import viewsets
from rest_framework.filters import OrderingFilter, SearchFilter

from .api_permissions import (
    CommentAPIPermission,
    IsAdminOrModerator,
    ReadOnlyOrAdminModerator,
)
from .auth import get_game_user
from .filters import CommentFilter, GameMapFilter, QuestFilter, TraderFilter
from .models import Comment, GameMap, GameUser, Quest, Trader
from .serializers import (
    CommentSerializer,
    GameMapSerializer,
    GameUserSerializer,
    QuestSerializer,
    TraderSerializer,
)


class QuestViewSet(viewsets.ModelViewSet):
    queryset = Quest.objects.select_related('trader').order_by('id')
    serializer_class = QuestSerializer
    permission_classes = [ReadOnlyOrAdminModerator]
    filterset_class = QuestFilter
    filter_backends = [DjangoFilterBackend, SearchFilter, OrderingFilter]
    search_fields = ('name', 'description')
    ordering_fields = ('name', 'required_level', 'changed_at')


class TraderViewSet(viewsets.ModelViewSet):
    queryset = Trader.objects.all().order_by('id')
    serializer_class = TraderSerializer
    permission_classes = [ReadOnlyOrAdminModerator]
    filterset_class = TraderFilter
    filter_backends = [DjangoFilterBackend, SearchFilter, OrderingFilter]
    search_fields = ('name',)
    ordering_fields = ('name',)


class GameMapViewSet(viewsets.ModelViewSet):
    queryset = GameMap.objects.all().order_by('id')
    serializer_class = GameMapSerializer
    permission_classes = [ReadOnlyOrAdminModerator]
    filterset_class = GameMapFilter
    filter_backends = [DjangoFilterBackend, SearchFilter, OrderingFilter]
    search_fields = ('name', 'description')
    ordering_fields = ('name', 'difficulty')


class CommentViewSet(viewsets.ModelViewSet):
    queryset = Comment.objects.select_related('author', 'quest').order_by('-id')
    serializer_class = CommentSerializer
    permission_classes = [CommentAPIPermission]
    filterset_class = CommentFilter
    filter_backends = [DjangoFilterBackend, OrderingFilter]
    ordering_fields = ('id', 'rating', 'changed_at')

    def perform_create(self, serializer):
        serializer.save(author=get_game_user(self.request))


class GameUserViewSet(viewsets.ReadOnlyModelViewSet):
    queryset = GameUser.objects.select_related('role').order_by('id')
    serializer_class = GameUserSerializer
    permission_classes = [IsAdminOrModerator]
    filter_backends = [SearchFilter, OrderingFilter]
    search_fields = ('username', 'email')
    ordering_fields = ('username', 'registred_at')
