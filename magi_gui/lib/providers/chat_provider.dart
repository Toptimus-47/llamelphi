import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/magi_api_service.dart';

/// Representation of a message in the chat
class ChatMessage {
  final String role;
  String content;
  String? unit;
  String? reasoningLog; // New for 2026: Capture DeepSeek-R2 CoT
  final String timestamp;

  ChatMessage({
    required this.role,
    required this.content,
    this.unit,
    this.reasoningLog,
    required this.timestamp,
  });
}

/// State class for the chat
class ChatState {
  final List<ChatMessage> messages;
  final bool isProcessing;
  final Map<String, dynamic>? currentVizData;
  final String? currentSessionId;
  final Map<String, String> unitStates;
  final List<String> searchKeywords; // New 2026: Store deliberated keywords
  final Map<String, dynamic> metrics; // New 2026: Live telemetry (docs, size, tokens)

  ChatState({
    this.messages = const [],
    this.isProcessing = false,
    this.currentVizData,
    this.currentSessionId,
    this.unitStates = const {
      'Melchior': 'IDLE',
      'Balthasar': 'IDLE',
      'Casper': 'IDLE',
      'Artaban': 'IDLE',
      'Gushnasaph': 'IDLE',
      'Kagba': 'IDLE',
    },
    this.searchKeywords = const [],
    this.metrics = const {
      'documents': 0,
      'size_kb': 0,
      'est_tokens': 0,
      'current_query': '',
    },
  });

  ChatState copyWith({
    List<ChatMessage>? messages,
    bool? isProcessing,
    Map<String, dynamic>? currentVizData,
    String? currentSessionId,
    Map<String, String>? unitStates,
    List<String>? searchKeywords,
    Map<String, dynamic>? metrics,
  }) {
    return ChatState(
      messages: messages ?? this.messages,
      isProcessing: isProcessing ?? this.isProcessing,
      currentVizData: currentVizData ?? this.currentVizData,
      currentSessionId: currentSessionId ?? this.currentSessionId,
      unitStates: unitStates ?? this.unitStates,
      searchKeywords: searchKeywords ?? this.searchKeywords,
      metrics: metrics ?? this.metrics,
    );
  }
}

/// Provider to manage chat logic
class ChatNotifier extends StateNotifier<ChatState> {
  final MagiApiService _api = MagiApiService();

  ChatNotifier() : super(ChatState()) {
    _initialize();
  }

  Future<void> _initialize() async {
    try {
      await _api.getSessions();
    } catch (e) {
      print("Waiting for MAGI Core...");
    }
  }

  Future<void> loadSession(String sessionId) async {
    state = state.copyWith(isProcessing: true, currentSessionId: sessionId);
    try {
      final history = await _api.getHistory(sessionId);
      final messages = (history as List).map((msg) {
        return ChatMessage(
          role: msg['role'],
          content: msg['content'],
          unit: msg['unit'],
          reasoningLog: msg['reasoning_log'],
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

    final userMsg = ChatMessage(
      role: 'user',
      content: query,
      timestamp: DateTime.now().toString(),
    );

    state = state.copyWith(
      messages: [...state.messages, userMsg],
      isProcessing: true,
    );

    // Initial persistence (User message)
    final Map<String, dynamic> userEntry = {
      'role': 'user',
      'content': query,
      'timestamp': userMsg.timestamp,
      'session_id': state.currentSessionId,
    };
    // Note: We'd typically call a separate API for history persistence if needed, 
    // but the backend is currently set up to save the ASSISTANT response in the stream handler.
    // To be consistent, let's assume the backend handles assistant responses.

    try {
      await for (final event in _api.streamChat(query, sessionId: state.currentSessionId)) {
        final type = event['type'];

        if (type == 'metadata') {
          state = state.copyWith(currentSessionId: event['session_id']);
        } 
        else if (type == 'search_strategy') {
          // Aligned 2026 protocol: SearchStrategy event
          final List<String> keywords = List<String>.from(event['queries']);
          state = state.copyWith(searchKeywords: keywords);
        }
        else if (type == 'telemetry') {
          // Aligned 2026 protocol: Telemetry metrics
          state = state.copyWith(metrics: Map<String, dynamic>.from(event['metrics']));
        }
        else if (type == 'status') {
          final content = event['content'] as String;
          final updatedUnits = Map<String, String>.from(state.unitStates);
          
          final List<String> unitKeys = ['Melchior', 'Balthasar', 'Casper', 'Artaban', 'Gushnasaph', 'Kagba'];
          
          bool matched = false;
          for (var key in unitKeys) {
            if (content.contains(key)) {
              if (content.contains('Critique')) {
                updatedUnits[key] = 'CRITIQUE';
              } else {
                updatedUnits[key] = 'ACTIVE';
              }
              matched = true;
            }
          }

          if (content.toLowerCase().contains('consensus')) {
            for (var key in unitKeys) {
              updatedUnits[key] = 'CONSENSUS';
            }
            matched = true;
          }

          if (matched) {
            state = state.copyWith(unitStates: updatedUnits);
          }
        }
        else if (type == 'reasoning') {
          final unitName = event['unit'] as String? ?? 'Orchestrator';
          final reasoning = event['content'] as String;
          
          final updatedMessages = List<ChatMessage>.from(state.messages);
          int msgIndex = updatedMessages.lastIndexWhere((m) => m.role == 'assistant');
          
          if (msgIndex != -1) {
            final m = updatedMessages[msgIndex];
            m.reasoningLog = (m.reasoningLog ?? "") + reasoning;
          } else {
            updatedMessages.add(ChatMessage(
              role: 'assistant',
              content: '',
              unit: unitName,
              reasoningLog: reasoning,
              timestamp: DateTime.now().toString(),
            ));
          }
          state = state.copyWith(messages: updatedMessages);
        }
        else if (type == 'token') {
          final unitName = event['unit'] as String? ?? 'MAGI';
          final token = event['content'] as String;
          
          final updatedMessages = List<ChatMessage>.from(state.messages);
          int unitMsgIndex = updatedMessages.lastIndexWhere((m) => 
            m.unit == unitName && m.role == 'assistant'
          );
          
          if (unitMsgIndex != -1) {
            updatedMessages[unitMsgIndex].content += token;
          } else {
            updatedMessages.add(ChatMessage(
              role: 'assistant',
              content: token,
              unit: unitName,
              timestamp: DateTime.now().toString(),
            ));
          }
          state = state.copyWith(messages: updatedMessages);
        }
        else if (type == 'error') {
          final errorMessage = event['content'] as String;
          final updatedMessages = List<ChatMessage>.from(state.messages);
          updatedMessages.add(ChatMessage(
            role: 'assistant',
            content: '### [SYSTEM_ERROR]\n$errorMessage',
            timestamp: DateTime.now().toString(),
          ));
          state = state.copyWith(messages: updatedMessages, isProcessing: false);
        }
        else if (type == 'final') {
          final finalContent = event['content'] as String;
          final updatedMessages = List<ChatMessage>.from(state.messages);
          
          // Replace content with masked final answer if available
          int lastAssistantIndex = updatedMessages.lastIndexWhere((m) => m.role == 'assistant');
          if (lastAssistantIndex != -1) {
            updatedMessages[lastAssistantIndex].content = finalContent;
          }

          state = state.copyWith(
            messages: updatedMessages,
            currentVizData: event['viz_data'],
            isProcessing: false,
            unitStates: {
              'Melchior': 'IDLE', 'Balthasar': 'IDLE', 'Casper': 'IDLE',
              'Artaban': 'IDLE', 'Gushnasaph': 'IDLE', 'Kagba': 'IDLE',
            },
          );
        }
      }
    } catch (e) {
      state = state.copyWith(isProcessing: false);
    }
  }
}

final chatProvider = StateNotifierProvider<ChatNotifier, ChatState>((ref) => ChatNotifier());
