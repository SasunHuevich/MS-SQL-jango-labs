from django.contrib import admin

from .models import (
    Comment,
    GameMap,
    GameUser,
    Item,
    MapMarker,
    Picture,
    PictureType,
    Quest,
    QuestMap,
    QuestRef,
    QuestReward,
    Role,
    Trader,
    UserCompleteQuest,
    UserOAuth,
)


class QuestRewardInline(admin.TabularInline):
    model = QuestReward
    extra = 0
    autocomplete_fields = ('item', 'trader')


class CommentInline(admin.TabularInline):
    model = Comment
    extra = 0
    readonly_fields = ('changed_at',)
    autocomplete_fields = ('author',)


@admin.register(Quest)
class QuestAdmin(admin.ModelAdmin):
    list_display = ('name', 'trader', 'required_level', 'changed_at')
    search_fields = ('name', 'description')
    list_filter = ('required_level',)
    list_select_related = ('trader',)
    show_full_result_count = False
    autocomplete_fields = ('trader', 'picture')
    inlines = [QuestRewardInline, CommentInline]


class MapMarkerInline(admin.TabularInline):
    model = MapMarker
    extra = 0
    autocomplete_fields = ('picture',)


@admin.register(GameMap)
class GameMapAdmin(admin.ModelAdmin):
    list_display = ('name', 'difficulty')
    search_fields = ('name', 'description')
    list_filter = ('difficulty',)
    autocomplete_fields = ('picture',)
    inlines = [MapMarkerInline]


@admin.register(Trader)
class TraderAdmin(admin.ModelAdmin):
    list_display = ('name', 'picture')
    search_fields = ('name',)
    autocomplete_fields = ('picture',)


@admin.register(Item)
class ItemAdmin(admin.ModelAdmin):
    list_display = ('name', 'picture')
    search_fields = ('name',)
    autocomplete_fields = ('picture',)


@admin.register(GameUser)
class GameUserAdmin(admin.ModelAdmin):
    list_display = ('username', 'email', 'role', 'registred_at')
    search_fields = ('username', 'email')
    list_filter = ('role',)
    list_select_related = ('role',)
    autocomplete_fields = ('avatar', 'role')


@admin.register(Comment)
class CommentAdmin(admin.ModelAdmin):
    list_display = ('id', 'quest', 'author', 'rating', 'changed_at')
    list_filter = ('rating',)
    search_fields = ('text',)
    list_select_related = ('quest', 'author')
    autocomplete_fields = ('quest', 'author')


@admin.register(Role)
class RoleAdmin(admin.ModelAdmin):
    list_display = ('name', 'description')
    search_fields = ('name',)


@admin.register(PictureType)
class PictureTypeAdmin(admin.ModelAdmin):
    list_display = ('name',)
    search_fields = ('name',)


@admin.register(Picture)
class PictureAdmin(admin.ModelAdmin):
    list_display = ('url', 'type')
    list_filter = ('type',)
    search_fields = ('url',)
    autocomplete_fields = ('type',)


@admin.register(QuestReward)
class QuestRewardAdmin(admin.ModelAdmin):
    list_display = ('quest', 'type', 'item', 'amount', 'reputation_amount')
    list_filter = ('type',)
    list_select_related = ('quest', 'item', 'trader')
    autocomplete_fields = ('quest', 'item', 'trader')


@admin.register(MapMarker)
class MapMarkerAdmin(admin.ModelAdmin):
    list_display = ('map', 'type', 'x', 'y')
    list_filter = ('type',)
    list_select_related = ('map',)
    autocomplete_fields = ('map', 'picture')


@admin.register(UserOAuth)
class UserOAuthAdmin(admin.ModelAdmin):
    list_display = ('user', 'provider', 'provider_user_id')
    list_filter = ('provider',)
    autocomplete_fields = ('user',)


@admin.register(QuestRef)
class QuestRefAdmin(admin.ModelAdmin):
    list_display = ('quest', 'required_quest')
    autocomplete_fields = ('quest', 'required_quest')


@admin.register(UserCompleteQuest)
class UserCompleteQuestAdmin(admin.ModelAdmin):
    list_display = ('user', 'quest')
    autocomplete_fields = ('user', 'quest')


@admin.register(QuestMap)
class QuestMapAdmin(admin.ModelAdmin):
    list_display = ('quest', 'map')
    autocomplete_fields = ('quest', 'map')
