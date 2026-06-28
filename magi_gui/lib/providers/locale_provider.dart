import 'package:flutter_riverpod/flutter_riverpod.dart';

enum Language { ko, en }

class LocaleState {
  final Language language;
  
  LocaleState({this.language = Language.ko});

  LocaleState copyWith({Language? language}) {
    return LocaleState(language: language ?? this.language);
  }

  String translate(String key) {
    final translations = {
      'ko': {
        'app_title': 'MAGI 리서치',
        'new_chat': '새 대화',
        'user_account': '사용자 계정',
        'message_hint': 'MAGI에게 메시지 보내기...',
        'help_title': '사용자 매뉴얼',
        'close': '닫기',
        'send': '전송',
      },
      'en': {
        'app_title': 'MAGI Research',
        'new_chat': 'New Chat',
        'user_account': 'User Account',
        'message_hint': 'Message MAGI...',
        'help_title': 'User Manual',
        'close': 'Close',
        'send': 'Send',
      }
    };
    return translations[language.name]?[key] ?? key;
  }
}

class LocaleNotifier extends StateNotifier<LocaleState> {
  LocaleNotifier() : super(LocaleState());

  void toggleLanguage() {
    state = state.copyWith(
      language: state.language == Language.ko ? Language.en : Language.ko,
    );
  }
}

final localeProvider = StateNotifierProvider<LocaleNotifier, LocaleState>((ref) {
  return LocaleNotifier();
});
