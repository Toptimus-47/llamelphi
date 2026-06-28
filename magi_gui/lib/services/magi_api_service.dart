import 'dart:convert';
import 'package:http/http.dart' as http;
import '../core/constants/app_constants.dart';

/// Represents a message in the chat
class ChatMessage {
  final String role;
  String content;
  final String timestamp;
  final Map<String, dynamic>? vizData;
  final bool isCode;
  final String? unit; // 모델 유닛 이름 (예: SmolLM-Draft)

  ChatMessage({
    required this.role,
    required this.content,
    required this.timestamp,
    this.vizData,
    this.isCode = false,
    this.unit,
  });
}

/// Service to handle communication with MAGI_Core Backend
class MagiApiService {
  final String baseUrl = AppConstants.apiBaseUrl;

  /// Streams chat tokens from the backend
  Stream<Map<String, dynamic>> streamChat(String query, {String? sessionId}) async* {
    final client = http.Client();
    final request = http.Request('POST', Uri.parse("$baseUrl/chat/stream"));
    request.headers['Content-Type'] = 'application/json';
    request.headers['x-magi-token'] = 'MAGI_LOCAL_TOKEN'; // Local Auth Token
    request.body = jsonEncode({
      'query': query,
      'session_id': sessionId,
    });

    final response = await client.send(request);

    if (response.statusCode != 200) {
      throw Exception("Backend Connection Failed: ${response.statusCode}");
    }

    // Process SSE stream from Rust/Axum
    await for (final line in response.stream.transform(utf8.decoder).transform(const LineSplitter())) {
      final trimmedLine = line.trim();
      if (trimmedLine.isEmpty) continue;
      
      // Handle Server-Sent Events (SSE) prefix
      if (trimmedLine.startsWith("data: ")) {
        final data = trimmedLine.substring(6).trim();
        if (data == "[DONE]") continue;
        
        try {
          yield jsonDecode(data);
        } catch (e) {
          print("[Elite UI] Stream Parsing Error: $e on line: $data");
        }
      } else {
        // Fallback for raw JSON lines (legacy or direct NDJSON)
        try {
          yield jsonDecode(trimmedLine);
        } catch (_) {
          // Ignore non-json lines
        }
      }
    }
  }

  Future<List<dynamic>> getSessions() async {
    final resp = await http.get(Uri.parse("$baseUrl/sessions"));
    return jsonDecode(resp.body);
  }

  Future<List<dynamic>> getHistory(String sessionId) async {
    final resp = await http.get(Uri.parse("$baseUrl/sessions/$sessionId/history"));
    return jsonDecode(resp.body);
  }

  Future<void> shutdown() async {
    try {
      await http.post(Uri.parse("$baseUrl/system/shutdown"));
    } catch (e) {
      print("[API] Shutdown request failed (backend may already be closed): $e");
    }
  }

  Future<Map<String, dynamic>> hotSwapModel(String unitName, String modelPath) async {
    final resp = await http.post(
      Uri.parse("$baseUrl/system/hotswap"),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode({
        'unit_name': unitName,
        'model_path': modelPath,
      }),
    );
    return jsonDecode(resp.body);
  }
}
