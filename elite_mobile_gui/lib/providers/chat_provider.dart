import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/api_service.dart';

/// State class for the chat
class ChatState {
  final List<ChatMessage> messages;
  final bool isProcessing;
  final Map<String, dynamic>? currentVizData;
  final String? currentSessionId;

  ChatState({
    this.messages = const [],
    this.isProcessing = false,
    this.currentVizData,
    this.currentSessionId,
  });

  ChatState copyWith({
    List<ChatMessage>? messages,
    bool? isProcessing,
    Map<String, dynamic>? currentVizData,
    String? currentSessionId,
  }) {
    return ChatState(
      messages: messages ?? this.messages,
      isProcessing: isProcessing ?? this.isProcessing,
      currentVizData: currentVizData ?? this.currentVizData,
      currentSessionId: currentSessionId ?? this.currentSessionId,
    );
  }
}

/// Provider to manage chat logic
class ChatNotifier extends StateNotifier<ChatState> {
  final EliteApiService _api = EliteApiService();

  ChatNotifier() : super(ChatState());

  Future<void> loadSession(String sessionId) async {
    state = state.copyWith(isProcessing: true, currentSessionId: sessionId);
    try {
      final history = await _api.getHistory(sessionId);
      final messages = (history as List).map((msg) {
        return ChatMessage(
          role: msg['role'],
          content: msg['content'],
          timestamp: msg['timestamp'] ?? DateTime.now().toString(),
        );
      }).toList();
      state = state.copyWith(messages: messages, isProcessing: false);
    } catch (e) {
      state = state.copyWith(isProcessing: false);
    }
  }

  void newSession() {
    state = ChatState();
  }

  Future<void> sendQuery(String query) async {
    if (query.isEmpty || state.isProcessing) return;

    // 1. Add User Message
    final userMsg = ChatMessage(
      role: 'user',
      content: query,
      timestamp: DateTime.now().toString(),
    );

    state = state.copyWith(
      messages: [...state.messages, userMsg],
      isProcessing: true,
    );

    try {
      // 2. Prepare Placeholder AI Message for Streaming
      final aiMsg = ChatMessage(
        role: 'assistant',
        content: '',
        timestamp: DateTime.now().toString(),
      );
      state = state.copyWith(messages: [...state.messages, aiMsg]);

      // 3. Start Streaming
      await for (final event in _api.streamChat(query, sessionId: state.currentSessionId)) {
        final type = event['type'];

        if (type == 'metadata') {
          state = state.copyWith(currentSessionId: event['session_id']);
        } 
        else if (type == 'token') {
          // Update the last message (AI message) with new token
          final updatedMessages = List<ChatMessage>.from(state.messages);
          updatedMessages.last.content += event['content'];
          state = state.copyWith(messages: updatedMessages);
        } 
        else if (type == 'final') {
          // Final consolidation
          final updatedMessages = List<ChatMessage>.from(state.messages);
          updatedMessages.last.content = event['content'];
          state = state.copyWith(
            messages: updatedMessages,
            currentVizData: event['viz_data'],
            isProcessing: false,
          );
        }
      }
    } catch (e) {
      state = state.copyWith(
        isProcessing: false,
        messages: [
          ...state.messages,
          ChatMessage(role: 'assistant', content: 'Error: $e', timestamp: DateTime.now().toString())
        ],
      );
    }
  }
}

final chatProvider = StateNotifierProvider<ChatNotifier, ChatState>((ref) => ChatNotifier());
