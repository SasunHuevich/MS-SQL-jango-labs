import django_filters

from .models import Comment, GameMap, Quest, Trader


class QuestFilter(django_filters.FilterSet):
    name = django_filters.CharFilter(lookup_expr='icontains', label='Название')
    required_level_min = django_filters.NumberFilter(
        field_name='required_level',
        lookup_expr='gte',
        label='Уровень от',
    )
    required_level_max = django_filters.NumberFilter(
        field_name='required_level',
        lookup_expr='lte',
        label='Уровень до',
    )
    trader = django_filters.NumberFilter(field_name='trader_id', label='ID торговца')

    class Meta:
        model = Quest
        fields = ['name', 'trader', 'required_level_min', 'required_level_max']


class TraderFilter(django_filters.FilterSet):
    name = django_filters.CharFilter(lookup_expr='icontains', label='Имя')

    class Meta:
        model = Trader
        fields = ['name']


class GameMapFilter(django_filters.FilterSet):
    name = django_filters.CharFilter(lookup_expr='icontains', label='Название')
    difficulty_min = django_filters.NumberFilter(
        field_name='difficulty',
        lookup_expr='gte',
        label='Сложность от',
    )
    difficulty_max = django_filters.NumberFilter(
        field_name='difficulty',
        lookup_expr='lte',
        label='Сложность до',
    )

    class Meta:
        model = GameMap
        fields = ['name', 'difficulty_min', 'difficulty_max']


class CommentFilter(django_filters.FilterSet):
    quest = django_filters.NumberFilter(field_name='quest_id', label='ID квеста')
    rating_min = django_filters.NumberFilter(
        field_name='rating',
        lookup_expr='gte',
        label='Рейтинг от',
    )

    class Meta:
        model = Comment
        fields = ['quest', 'rating_min']
