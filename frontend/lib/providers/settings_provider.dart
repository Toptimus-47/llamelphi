import 'package:flutter_riverpod/flutter_riverpod.dart';

class Settings {
  final String backendUrl;
  final bool darkMode;
  Settings({required this.backendUrl, required this.darkMode});
}

class SettingsNotifier extends StateNotifier<Settings> {
  SettingsNotifier() : super(Settings(backendUrl: 'http://localhost:8000', darkMode: false));

  void setBackendUrl(String url) => state = Settings(backendUrl: url, darkMode: state.darkMode);
  void toggleDarkMode() => state = Settings(backendUrl: state.backendUrl, darkMode: !state.darkMode);
}

final settingsProvider = StateNotifierProvider<SettingsNotifier, Settings>((ref) => SettingsNotifier());
