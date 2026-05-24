from django import forms
from django.core.exceptions import ValidationError

from .models import Comment, GameUser, Quest, Role, Trader


class RegistrationForm(forms.Form):
    username = forms.CharField(max_length=100, label='Логин')
    email = forms.EmailField(required=False, label='Email')
    password = forms.CharField(widget=forms.PasswordInput, label='Пароль')
    password_confirm = forms.CharField(widget=forms.PasswordInput, label='Повтор пароля')

    def clean_username(self):
        username = self.cleaned_data['username'].strip()
        if not username:
            raise ValidationError('Введите логин.')
        if GameUser.objects.filter(username__iexact=username).exists():
            raise ValidationError('Пользователь с таким логином уже существует.')
        return username

    def clean(self):
        cleaned = super().clean()
        password = cleaned.get('password')
        password_confirm = cleaned.get('password_confirm')
        if password and password_confirm and password != password_confirm:
            raise ValidationError('Пароли не совпадают.')
        return cleaned

    def save(self):
        role = Role.objects.get(name='user')
        return GameUser.objects.create(
            username=self.cleaned_data['username'],
            password=self.cleaned_data['password'],
            email=self.cleaned_data.get('email') or None,
            role=role,
        )

class QuestForm(forms.ModelForm):
    class Meta:
        model = Quest
        fields = ('name', 'description', 'trader', 'picture', 'required_level')
        widgets = {
            'description': forms.Textarea(attrs={'rows': 4}),
            'required_level': forms.NumberInput(attrs={'min': 1}),
        }


class TraderForm(forms.ModelForm):
    class Meta:
        model = Trader
        fields = ('name', 'description', 'picture')
        widgets = {
            'description': forms.Textarea(attrs={'rows': 3}),
        }


class CommentForm(forms.ModelForm):
    class Meta:
        model = Comment
        fields = ('text', 'rating', 'quest')
        widgets = {
            'text': forms.Textarea(attrs={'rows': 3}),
            'rating': forms.NumberInput(attrs={'min': 1, 'max': 5}),
        }
