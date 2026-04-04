import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_markdown/flutter_markdown.dart';
import 'package:google_fonts/google_fonts.dart';
import 'providers/chat_provider.dart';
import 'providers/session_provider.dart';

void main() {
  runApp(const ProviderScope(child: EliteAgentApp()));
}

class EliteAgentApp extends StatelessWidget {
  const EliteAgentApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'ELITE Agent Terminal',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        useMaterial3: true,
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.blueGrey,
          brightness: Brightness.light,
        ),
        textTheme: GoogleFonts.interTextTheme(),
      ),
      darkTheme: ThemeData(
        useMaterial3: true,
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.blueGrey,
          brightness: Brightness.dark,
        ),
        textTheme: GoogleFonts.interTextTheme(ThemeData.dark().textTheme),
      ),
      home: const ChatScreen(),
    );
  }
}

class ChatScreen extends ConsumerWidget {
  const ChatScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final chatState = ref.watch(chatProvider);
    final sessionState = ref.watch(sessionProvider);
    final textController = TextEditingController();

    return Scaffold(
      appBar: AppBar(
        title: const Text('ELITE Intelligence', style: TextStyle(fontWeight: FontWeight.bold)),
        actions: [
          IconButton(
            icon: const Icon(Icons.add), 
            tooltip: 'New Session',
            onPressed: () => ref.read(chatProvider.notifier).newSession()
          ),
          IconButton(
            icon: const Icon(Icons.refresh), 
            tooltip: 'Refresh History',
            onPressed: () => ref.read(sessionProvider.notifier).refreshSessions()
          ),
        ],
      ),
      drawer: Drawer(
        child: Column(
          children: [
            DrawerHeader(
              decoration: BoxDecoration(color: Theme.of(context).colorScheme.primary),
              child: const Center(
                child: Text(
                  'Session Archive',
                  style: TextStyle(color: Colors.white, fontSize: 24, fontWeight: FontWeight.bold),
                ),
              ),
            ),
            if (sessionState.isLoading)
              const Center(child: Padding(
                padding: EdgeInsets.all(20.0),
                child: CircularProgressIndicator(),
              ))
            else
              Expanded(
                child: ListView.builder(
                  itemCount: sessionState.sessions.length,
                  itemBuilder: (context, index) {
                    final session = sessionState.sessions[index];
                    final isSelected = session.id == chatState.currentSessionId;
                    return ListTile(
                      leading: const Icon(Icons.chat_bubble_outline),
                      title: Text(session.title, maxLines: 1, overflow: TextOverflow.ellipsis),
                      subtitle: Text(session.updatedAt),
                      selected: isSelected,
                      onTap: () {
                        ref.read(chatProvider.notifier).loadSession(session.id);
                        Navigator.pop(context);
                      },
                    );
                  },
                ),
              ),
          ],
        ),
      ),
      body: Column(
        children: [
          Expanded(
            child: ListView.builder(
              padding: const EdgeInsets.all(16),
              itemCount: chatState.messages.length,
              itemBuilder: (context, index) {
                final msg = chatState.messages[index];
                final isUser = msg.role == 'user';
                return Align(
                  alignment: isUser ? Alignment.centerRight : Alignment.centerLeft,
                  child: Container(
                    margin: const EdgeInsets.symmetric(vertical: 8),
                    padding: const EdgeInsets.all(12),
                    decoration: BoxDecoration(
                      color: isUser 
                        ? Theme.of(context).colorScheme.primaryContainer 
                        : Theme.of(context).colorScheme.surfaceVariant,
                      borderRadius: BorderRadius.circular(12),
                    ),
                    constraints: BoxConstraints(maxWidth: MediaQuery.of(context).size.width * 0.8),
                    child: MarkdownBody(
                      data: msg.content,
                      selectable: true,
                      styleSheet: MarkdownStyleSheet.fromTheme(Theme.of(context)).copyWith(
                        code: const TextStyle(
                          backgroundColor: Colors.transparent,
                          fontFamily: 'monospace',
                          fontSize: 13,
                        ),
                        codeblockDecoration: BoxDecoration(
                          color: Colors.black87,
                          borderRadius: BorderRadius.circular(8),
                        ),
                      ),
                    ),
                  ),
                );
              },
            ),
          ),
          if (chatState.isProcessing)
            const Padding(
              padding: EdgeInsets.all(8.0),
              child: LinearProgressIndicator(),
            ),
          _buildInputArea(context, ref, textController),
        ],
      ),
    );
  }

  Widget _buildInputArea(BuildContext context, WidgetRef ref, TextEditingController controller) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surface,
        boxShadow: [BoxShadow(color: Colors.black12, blurRadius: 4)],
      ),
      child: Row(
        children: [
          Expanded(
            child: TextField(
              controller: controller,
              decoration: const InputDecoration(
                hintText: 'Enter strategic proposition...',
                border: InputBorder.none,
              ),
              onSubmitted: (val) {
                ref.read(chatProvider.notifier).sendQuery(val);
                controller.clear();
              },
            ),
          ),
          IconButton(
            icon: const Icon(Icons.send_rounded, color: Colors.blue),
            onPressed: () {
              ref.read(chatProvider.notifier).sendQuery(controller.text);
              controller.clear();
            },
          ),
        ],
      ),
    );
  }
}
