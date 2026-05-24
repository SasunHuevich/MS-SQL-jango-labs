from django.core.management.base import BaseCommand, CommandError

from game.models import GameUser, Role


class Command(BaseCommand):
    help = 'Создаёт пользователя в таблице users (для входа в приложение).'

    def add_arguments(self, parser):
        parser.add_argument('username', type=str)
        parser.add_argument('password', type=str)
        parser.add_argument(
            '--role',
            type=str,
            default='user',
            help='Роль: admin, moderator, user, premium',
        )

    def handle(self, *args, **options):
        role_name = options['role']
        try:
            role = Role.objects.get(name=role_name)
        except Role.DoesNotExist as exc:
            raise CommandError(f'Роль "{role_name}" не найдена в таблице roles.') from exc

        user, created = GameUser.objects.update_or_create(
            username=options['username'],
            defaults={
                'password': options['password'],
                'role': role,
            },
        )
        action = 'Создан' if created else 'Обновлён'
        self.stdout.write(self.style.SUCCESS(
            f'{action} пользователь {user.username} (роль: {role_name})',
        ))
