from rest_framework import serializers

from .models import Comment, GameMap, GameUser, Quest, Trader


class TraderSerializer(serializers.ModelSerializer):
    class Meta:
        model = Trader
        fields = ('id', 'name', 'description', 'picture_id')


class QuestSerializer(serializers.ModelSerializer):
    trader_name = serializers.CharField(source='trader.name', read_only=True)

    class Meta:
        model = Quest
        fields = (
            'id',
            'name',
            'description',
            'picture_id',
            'trader_id',
            'trader_name',
            'required_level',
            'changed_at',
        )
        read_only_fields = ('changed_at',)


class GameMapSerializer(serializers.ModelSerializer):
    class Meta:
        model = GameMap
        fields = ('id', 'name', 'description', 'picture_id', 'difficulty')


class CommentSerializer(serializers.ModelSerializer):
    author_name = serializers.CharField(source='author.username', read_only=True)
    quest_name = serializers.CharField(source='quest.name', read_only=True)

    class Meta:
        model = Comment
        fields = (
            'id',
            'text',
            'author_id',
            'author_name',
            'rating',
            'quest_id',
            'quest_name',
            'changed_at',
        )
        read_only_fields = ('author_id', 'changed_at')


class GameUserSerializer(serializers.ModelSerializer):
    role_name = serializers.CharField(source='role.name', read_only=True)

    class Meta:
        model = GameUser
        fields = ('id', 'username', 'email', 'role_id', 'role_name', 'registred_at')
        extra_kwargs = {'password': {'write_only': True}}
