import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/api_service.dart';

class SessionInfo {
  final String id;
  final String title;
  final String updatedAt;

  SessionInfo({required this.id, required this.title, required this.updatedAt});

  factory SessionInfo.fromJson(Map<String, dynamic> json) {
    return SessionInfo(
      id: json['id'],
      title: json['title'],
      updatedAt: json['updated_at'],
    );
  }
}

class SessionState {
  final List<SessionInfo> sessions;
  final bool isLoading;

  SessionState({this.sessions = const [], this.isLoading = false});

  SessionState copyWith({List<SessionInfo>? sessions, bool? isLoading}) {
    return SessionState(
      sessions: sessions ?? this.sessions,
      isLoading: isLoading ?? this.isLoading,
    );
  }
}

class SessionNotifier extends StateNotifier<SessionState> {
  final EliteApiService _api = EliteApiService();

  SessionNotifier() : super(SessionState()) {
    refreshSessions();
  }

  Future<void> refreshSessions() async {
    state = state.copyWith(isLoading: true);
    try {
      final data = await _api.getSessions();
      final sessions = data.map((e) => SessionInfo.fromJson(e)).toList();
      state = state.copyWith(sessions: sessions, isLoading: false);
    } catch (e) {
      state = state.copyWith(isLoading: false);
    }
  }
}

final sessionProvider = StateNotifierProvider<SessionNotifier, SessionState>((ref) => SessionNotifier());
