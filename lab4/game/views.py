from django.contrib import messages
from django.shortcuts import redirect, render
from django.urls import reverse_lazy
from django.views import View
from django.views.generic import (
    CreateView,
    DeleteView,
    DetailView,
    ListView,
    TemplateView,
    UpdateView,
)
from django_filters.views import FilterView

from .auth import (
    GameLoginRequiredMixin,
    GameWritePermissionMixin,
    authenticate_game_user,
    get_game_user,
    login_game_user,
    logout_game_user,
    user_can_write,
)
from .filters import CommentFilter, GameMapFilter, QuestFilter, TraderFilter
from .forms import CommentForm, QuestForm, RegistrationForm, TraderForm
from .models import Comment, GameMap, GameUser, Quest, Trader


class HomeView(GameLoginRequiredMixin, TemplateView):
    template_name = 'game/home.html'

    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        context['quest_count'] = Quest.objects.count()
        context['map_count'] = GameMap.objects.count()
        context['trader_count'] = Trader.objects.count()
        context['current_user'] = get_game_user(self.request)
        context['can_write'] = user_can_write(context['current_user'])
        return context


class LoginView(View):
    template_name = 'game/login.html'

    def _redirect_after_login(self, request):
        next_url = (request.POST.get('next') or request.GET.get('next') or '').strip()
        if next_url.startswith('/') and not next_url.startswith('//'):
            return redirect(next_url)
        return redirect('home')

    def get(self, request):
        if get_game_user(request):
            return redirect('home')
        return render(request, self.template_name, {
            'next': request.GET.get('next', ''),
        })

    def post(self, request):
        username = request.POST.get('username', '')
        password = request.POST.get('password', '')
        user = authenticate_game_user(username, password)
        if user:
            login_game_user(request, user)
            messages.success(request, f'Добро пожаловать, {user.username}!')
            return self._redirect_after_login(request)
        messages.error(request, 'Неверный логин или пароль.')
        return render(request, self.template_name, {
            'next': request.POST.get('next', ''),
        })


class RegisterView(View):
    template_name = 'game/register.html'

    def get(self, request):
        if get_game_user(request):
            return redirect('home')
        return render(request, self.template_name, {'form': RegistrationForm()})

    def post(self, request):
        if get_game_user(request):
            return redirect('home')
        form = RegistrationForm(request.POST)
        if form.is_valid():
            user = form.save()
            login_game_user(request, user)
            messages.success(request, f'Аккаунт {user.username} создан.')
            return redirect('home')
        return render(request, self.template_name, {'form': form})


class LogoutView(View):
    def post(self, request):
        logout_game_user(request)
        messages.info(request, 'Вы вышли из системы.')
        return redirect('login')

    def get(self, request):
        return self.post(request)


class QuestListView(GameLoginRequiredMixin, FilterView):
    model = Quest
    template_name = 'game/quest_list.html'
    context_object_name = 'quests'
    paginate_by = 20
    filterset_class = QuestFilter

    def get_queryset(self):
        return Quest.objects.select_related('trader', 'picture').order_by('name')

    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        context['can_write'] = user_can_write(get_game_user(self.request))
        return context


class QuestDetailView(GameLoginRequiredMixin, DetailView):
    model = Quest
    template_name = 'game/quest_detail.html'
    context_object_name = 'quest'

    def get_queryset(self):
        return Quest.objects.select_related('trader', 'picture').prefetch_related(
            'rewards__item',
            'rewards__trader',
            'comments__author',
            'quest_refs__required_quest',
            'quest_maps__map',
        )

    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        context['can_write'] = user_can_write(get_game_user(self.request))
        return context


class QuestCreateView(GameWritePermissionMixin, CreateView):
    model = Quest
    form_class = QuestForm
    template_name = 'game/quest_form.html'
    success_url = reverse_lazy('quest_list')


class QuestUpdateView(GameWritePermissionMixin, UpdateView):
    model = Quest
    form_class = QuestForm
    template_name = 'game/quest_form.html'
    success_url = reverse_lazy('quest_list')


class QuestDeleteView(GameWritePermissionMixin, DeleteView):
    model = Quest
    template_name = 'game/confirm_delete.html'
    success_url = reverse_lazy('quest_list')

    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        context['object_label'] = 'квест'
        return context


class GameMapListView(GameLoginRequiredMixin, FilterView):
    model = GameMap
    template_name = 'game/map_list.html'
    context_object_name = 'maps'
    paginate_by = 20
    filterset_class = GameMapFilter

    def get_queryset(self):
        return GameMap.objects.select_related('picture').order_by('name')


class GameMapDetailView(GameLoginRequiredMixin, DetailView):
    model = GameMap
    template_name = 'game/map_detail.html'
    context_object_name = 'game_map'

    def get_queryset(self):
        return GameMap.objects.select_related('picture').prefetch_related(
            'markers__picture',
            'quest_maps__quest',
        )


class TraderListView(GameLoginRequiredMixin, FilterView):
    model = Trader
    template_name = 'game/trader_list.html'
    context_object_name = 'traders'
    paginate_by = 20
    filterset_class = TraderFilter

    def get_queryset(self):
        return Trader.objects.select_related('picture').order_by('name')

    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        context['can_write'] = user_can_write(get_game_user(self.request))
        return context


class TraderCreateView(GameWritePermissionMixin, CreateView):
    model = Trader
    form_class = TraderForm
    template_name = 'game/trader_form.html'
    success_url = reverse_lazy('trader_list')


class TraderUpdateView(GameWritePermissionMixin, UpdateView):
    model = Trader
    form_class = TraderForm
    template_name = 'game/trader_form.html'
    success_url = reverse_lazy('trader_list')


class TraderDeleteView(GameWritePermissionMixin, DeleteView):
    model = Trader
    template_name = 'game/confirm_delete.html'
    success_url = reverse_lazy('trader_list')

    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        context['object_label'] = 'торговца'
        return context


class CommentListView(GameLoginRequiredMixin, FilterView):
    model = Comment
    template_name = 'game/comment_list.html'
    context_object_name = 'comments'
    paginate_by = 20
    filterset_class = CommentFilter

    def get_queryset(self):
        return Comment.objects.select_related('quest', 'author').order_by('-id')

    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        context['can_write'] = user_can_write(get_game_user(self.request))
        return context


class CommentCreateView(GameLoginRequiredMixin, CreateView):
    model = Comment
    form_class = CommentForm
    template_name = 'game/comment_form.html'
    success_url = reverse_lazy('comment_list')

    def form_valid(self, form):
        form.instance.author = get_game_user(self.request)
        return super().form_valid(form)


class CommentUpdateView(GameWritePermissionMixin, UpdateView):
    model = Comment
    form_class = CommentForm
    template_name = 'game/comment_form.html'
    success_url = reverse_lazy('comment_list')


class CommentDeleteView(GameWritePermissionMixin, DeleteView):
    model = Comment
    template_name = 'game/confirm_delete.html'
    success_url = reverse_lazy('comment_list')

    def get_context_data(self, **kwargs):
        context = super().get_context_data(**kwargs)
        context['object_label'] = 'комментарий'
        return context
