from django.urls import path

from . import views

urlpatterns = [
    path('', views.HomeView.as_view(), name='home'),
    path('login/', views.LoginView.as_view(), name='login'),
    path('register/', views.RegisterView.as_view(), name='register'),
    path('logout/', views.LogoutView.as_view(), name='logout'),
    path('quests/', views.QuestListView.as_view(), name='quest_list'),
    path('quests/new/', views.QuestCreateView.as_view(), name='quest_create'),
    path('quests/<int:pk>/', views.QuestDetailView.as_view(), name='quest_detail'),
    path('quests/<int:pk>/edit/', views.QuestUpdateView.as_view(), name='quest_update'),
    path('quests/<int:pk>/delete/', views.QuestDeleteView.as_view(), name='quest_delete'),
    path('maps/', views.GameMapListView.as_view(), name='map_list'),
    path('maps/<int:pk>/', views.GameMapDetailView.as_view(), name='map_detail'),
    path('traders/', views.TraderListView.as_view(), name='trader_list'),
    path('traders/new/', views.TraderCreateView.as_view(), name='trader_create'),
    path('traders/<int:pk>/edit/', views.TraderUpdateView.as_view(), name='trader_update'),
    path('traders/<int:pk>/delete/', views.TraderDeleteView.as_view(), name='trader_delete'),
    path('comments/', views.CommentListView.as_view(), name='comment_list'),
    path('comments/new/', views.CommentCreateView.as_view(), name='comment_create'),
    path('comments/<int:pk>/edit/', views.CommentUpdateView.as_view(), name='comment_update'),
    path('comments/<int:pk>/delete/', views.CommentDeleteView.as_view(), name='comment_delete'),
]
