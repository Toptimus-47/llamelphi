import 'package:flutter_riverpod/flutter_riverpod.dart';

// State representing a single chat message.
class ChatMessage {
  final String role; // "user" or "assistant"
  final String content;
  ChatMessage({required this.role, required this.content});
}

// StateNotifier to manage the list of chat messages.
class ChatNotifier extends StateNotifier<List<ChatMessage>> {
  ChatNotifier() : super([]);

  void addMessage(ChatMessage msg) => state = [...state, msg];

  void clear() => state = [];
}

// Riverpod provider exposing the ChatNotifier.
final chatProvider = StateNotifierProvider<ChatNotifier, List<ChatMessage>>((ref) {
  return ChatNotifier();
});
