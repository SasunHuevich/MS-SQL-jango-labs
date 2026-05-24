from django.db import models


class Role(models.Model):
    id = models.AutoField(primary_key=True)
    name = models.CharField(max_length=50, unique=True)
    description = models.CharField(max_length=255, blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'roles'
        verbose_name = 'Роль'
        verbose_name_plural = 'Роли'

    def __str__(self):
        return self.name


class PictureType(models.Model):
    id = models.AutoField(primary_key=True)
    name = models.CharField(max_length=50, unique=True)

    class Meta:
        managed = False
        db_table = 'picture_type'
        verbose_name = 'Тип изображения'
        verbose_name_plural = 'Типы изображений'

    def __str__(self):
        return self.name


class Picture(models.Model):
    id = models.AutoField(primary_key=True)
    url = models.CharField(max_length=500)
    type = models.ForeignKey(
        PictureType,
        on_delete=models.DO_NOTHING,
        db_column='type_id',
    )

    class Meta:
        managed = False
        db_table = 'picture'
        verbose_name = 'Изображение'
        verbose_name_plural = 'Изображения'

    def __str__(self):
        return self.url


class Trader(models.Model):
    id = models.AutoField(primary_key=True)
    name = models.CharField(max_length=100)
    description = models.CharField(max_length=500, blank=True, null=True)
    picture = models.ForeignKey(
        Picture,
        on_delete=models.DO_NOTHING,
        db_column='picture_id',
        blank=True,
        null=True,
    )

    class Meta:
        managed = False
        db_table = 'trader'
        verbose_name = 'Торговец'
        verbose_name_plural = 'Торговцы'

    def __str__(self):
        return self.name


class Item(models.Model):
    id = models.AutoField(primary_key=True)
    name = models.CharField(max_length=100)
    description = models.CharField(max_length=500, blank=True, null=True)
    picture = models.ForeignKey(
        Picture,
        on_delete=models.DO_NOTHING,
        db_column='picture_id',
        blank=True,
        null=True,
        related_name='items',
    )

    class Meta:
        managed = False
        db_table = 'item'
        verbose_name = 'Предмет'
        verbose_name_plural = 'Предметы'

    def __str__(self):
        return self.name


class Quest(models.Model):
    id = models.AutoField(primary_key=True)
    name = models.CharField(max_length=150)
    description = models.TextField(blank=True, null=True)
    picture = models.ForeignKey(
        Picture,
        on_delete=models.DO_NOTHING,
        db_column='picture_id',
        blank=True,
        null=True,
        related_name='quests',
    )
    trader = models.ForeignKey(
        Trader,
        on_delete=models.DO_NOTHING,
        db_column='trader_id',
        blank=True,
        null=True,
    )
    required_level = models.IntegerField()
    changed_at = models.DateTimeField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'quest'
        verbose_name = 'Квест'
        verbose_name_plural = 'Квесты'

    def __str__(self):
        return self.name


class QuestReward(models.Model):
    id = models.AutoField(primary_key=True)
    quest = models.ForeignKey(
        Quest,
        on_delete=models.DO_NOTHING,
        db_column='quest_id',
        related_name='rewards',
    )
    type = models.CharField(max_length=50, db_column='type')
    item = models.ForeignKey(
        Item,
        on_delete=models.DO_NOTHING,
        db_column='item_id',
        blank=True,
        null=True,
    )
    amount = models.IntegerField(default=1, blank=True, null=True)
    reputation_amount = models.IntegerField(blank=True, null=True)
    trader = models.ForeignKey(
        Trader,
        on_delete=models.DO_NOTHING,
        db_column='trader_id',
        blank=True,
        null=True,
        related_name='quest_rewards',
    )

    class Meta:
        managed = False
        db_table = 'quest_reward'
        verbose_name = 'Награда квеста'
        verbose_name_plural = 'Награды квестов'

    def __str__(self):
        return f'{self.quest} — {self.type}'


class GameUser(models.Model):
    id = models.AutoField(primary_key=True)
    username = models.CharField(max_length=100, unique=True)
    password = models.CharField(max_length=255)
    email = models.CharField(max_length=150, blank=True, null=True)
    avatar = models.ForeignKey(
        Picture,
        on_delete=models.DO_NOTHING,
        db_column='avatar_id',
        blank=True,
        null=True,
        related_name='users',
    )
    role = models.ForeignKey(
        Role,
        on_delete=models.DO_NOTHING,
        db_column='role_id',
        blank=True,
        null=True,
    )
    registred_at = models.DateTimeField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'users'
        verbose_name = 'Пользователь'
        verbose_name_plural = 'Пользователи'

    def __str__(self):
        return self.username


class UserOAuth(models.Model):
    id = models.AutoField(primary_key=True)
    user = models.ForeignKey(
        GameUser,
        on_delete=models.DO_NOTHING,
        db_column='user_id',
        related_name='oauth_accounts',
    )
    provider = models.CharField(max_length=50)
    provider_user_id = models.CharField(max_length=100)

    class Meta:
        managed = False
        db_table = 'user_oauth'
        verbose_name = 'OAuth'
        verbose_name_plural = 'OAuth'

    def __str__(self):
        return f'{self.provider}:{self.provider_user_id}'


class GameMap(models.Model):
    id = models.AutoField(primary_key=True)
    name = models.CharField(max_length=100)
    description = models.CharField(max_length=500, blank=True, null=True)
    picture = models.ForeignKey(
        Picture,
        on_delete=models.DO_NOTHING,
        db_column='picture_id',
        blank=True,
        null=True,
        related_name='maps',
    )
    difficulty = models.IntegerField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'map'
        verbose_name = 'Карта'
        verbose_name_plural = 'Карты'

    def __str__(self):
        return self.name


class Comment(models.Model):
    id = models.AutoField(primary_key=True)
    text = models.TextField()
    author = models.ForeignKey(
        GameUser,
        on_delete=models.DO_NOTHING,
        db_column='author_id',
        related_name='comments',
    )
    rating = models.IntegerField(blank=True, null=True)
    quest = models.ForeignKey(
        Quest,
        on_delete=models.DO_NOTHING,
        db_column='quest_id',
        related_name='comments',
    )
    changed_at = models.DateTimeField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'comment'
        verbose_name = 'Комментарий'
        verbose_name_plural = 'Комментарии'

    def __str__(self):
        return f'Комментарий #{self.id}'


class MapMarker(models.Model):
    id = models.AutoField(primary_key=True)
    map = models.ForeignKey(
        GameMap,
        on_delete=models.DO_NOTHING,
        db_column='map_id',
        related_name='markers',
    )
    type = models.CharField(max_length=50, blank=True, null=True)
    description = models.CharField(max_length=500, blank=True, null=True)
    access_rule = models.CharField(max_length=100, blank=True, null=True)
    picture = models.ForeignKey(
        Picture,
        on_delete=models.DO_NOTHING,
        db_column='picture_id',
        blank=True,
        null=True,
        related_name='markers',
    )
    x = models.FloatField()
    y = models.FloatField()

    class Meta:
        managed = False
        db_table = 'map_marker'
        verbose_name = 'Маркер карты'
        verbose_name_plural = 'Маркеры карт'

    def __str__(self):
        return f'Маркер #{self.id} ({self.x}, {self.y})'


class QuestRef(models.Model):
    quest = models.ForeignKey(
        Quest,
        on_delete=models.DO_NOTHING,
        db_column='quest_id',
        related_name='quest_refs',
        primary_key=True,
    )
    required_quest = models.ForeignKey(
        Quest,
        on_delete=models.DO_NOTHING,
        db_column='required_quest_id',
        related_name='required_by_refs',
    )

    class Meta:
        managed = False
        db_table = 'quest_ref'
        unique_together = (('quest', 'required_quest'),)
        verbose_name = 'Зависимость квеста'
        verbose_name_plural = 'Зависимости квестов'

    def __str__(self):
        return f'{self.quest} требует {self.required_quest}'


class UserCompleteQuest(models.Model):
    user = models.ForeignKey(
        GameUser,
        on_delete=models.DO_NOTHING,
        db_column='user_id',
        related_name='completed_quests',
        primary_key=True,
    )
    quest = models.ForeignKey(
        Quest,
        on_delete=models.DO_NOTHING,
        db_column='quest_id',
        related_name='completed_by_users',
    )

    class Meta:
        managed = False
        db_table = 'user_complete_quest'
        unique_together = (('user', 'quest'),)
        verbose_name = 'Выполненный квест'
        verbose_name_plural = 'Выполненные квесты'

    def __str__(self):
        return f'{self.user} — {self.quest}'


class QuestMap(models.Model):
    quest = models.ForeignKey(
        Quest,
        on_delete=models.DO_NOTHING,
        db_column='quest_id',
        related_name='quest_maps',
        primary_key=True,
    )
    map = models.ForeignKey(
        GameMap,
        on_delete=models.DO_NOTHING,
        db_column='map_id',
        related_name='quest_maps',
    )

    class Meta:
        managed = False
        db_table = 'quest_map'
        unique_together = (('quest', 'map'),)
        verbose_name = 'Квест на карте'
        verbose_name_plural = 'Квесты на картах'

    def __str__(self):
        return f'{self.quest} — {self.map}'
