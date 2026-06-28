import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_markdown/flutter_markdown.dart';
import 'package:google_fonts/google_fonts.dart';

import '../providers/chat_provider.dart';
import '../providers/locale_provider.dart';
import '../widgets/chat/message_bubble.dart';
import '../widgets/chat/chat_input.dart';
import '../widgets/chat/telemetry_dashboard.dart';
import '../widgets/chat/inference_header.dart';
import '../widgets/magi_panel.dart';
import '../core/theme/magi_colors.dart';

class ChatScreen extends ConsumerStatefulWidget {
  final VoidCallback toggleSidebar;
  final bool isSidebarVisible;

  const ChatScreen({
    super.key, 
    required this.toggleSidebar,
    required this.isSidebarVisible,
  });

  @override
  ConsumerState<ChatScreen> createState() => _ChatScreenState();
}

class _ChatScreenState extends ConsumerState<ChatScreen> {
  final TextEditingController _textController = TextEditingController();
  final FocusNode _focusNode = FocusNode();

  @override
  void initState() {
    super.initState();
    _focusNode.onKeyEvent = (node, event) {
      if (event is KeyDownEvent && event.logicalKey == LogicalKeyboardKey.enter) {
        if (!HardwareKeyboard.instance.isShiftPressed) {
          _handleSend();
          return KeyEventResult.handled;
        }
      }
      return KeyEventResult.ignored;
    };
  }

  void _handleSend() {
    final text = _textController.text.trim();
    if (text.isNotEmpty) {
      ref.read(chatProvider.notifier).sendQuery(text);
      _textController.clear();
    }
  }

  @override
  void dispose() {
    _textController.dispose();
    _focusNode.dispose();
    super.dispose();
  }

  void _showHelpDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Row(
          children: [
            const Icon(Icons.help_outline_rounded, color: Colors.indigoAccent),
            const SizedBox(width: 12),
            Text('User Manual', style: GoogleFonts.inter(fontWeight: FontWeight.bold)),
          ],
        ),
        content: SizedBox(
          width: 600,
          height: 500,
          child: SingleChildScrollView(
            child: MarkdownBody(
              data: '''
# MAGI System User Manual
### 다원론적 합의 기반 AI 추론 시스템

**1. 조작 안내**
- **Enter:** 메시지 전송
- **Shift + Enter:** 줄바꿈
- **좌측 사이드바:** 대화 세션 관리 및 기록 확인

**2. 시스템 개요**
MAGI는 Melchior, Balthasar, Casper 등 독립된 추론 유닛들이 협업하여 최적의 결론을 도출하는 시스템입니다.

**3. 주요 상태 표시**
- **IDLE:** 대기 중
- **ACTIVE:** 추론 진행 중
- **CONSENSUS:** 유닛 간 합의 도출 중

---
*본 매뉴얼은 로컬 환경의 `USER_MANUAL.md`를 기반으로 작성되었습니다.*
''',
              styleSheet: MarkdownStyleSheet(
                p: GoogleFonts.inter(height: 1.5),
                h1: GoogleFonts.inter(fontWeight: FontWeight.bold, fontSize: 20),
                h3: GoogleFonts.inter(fontWeight: FontWeight.w600, fontSize: 16),
              ),
            ),
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Close'),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final chatState = ref.watch(chatProvider);

    return Scaffold(
      backgroundColor: Colors.transparent,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        leading: IconButton(
          icon: Icon(widget.isSidebarVisible ? Icons.menu_open_rounded : Icons.menu_rounded),
          onPressed: widget.toggleSidebar,
        ),
        title: Row(
          children: [
            Text('MAGI Research', style: GoogleFonts.inter(fontWeight: FontWeight.w700, fontSize: 18)),
            const SizedBox(width: 8),
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
              decoration: BoxDecoration(
                color: MagiColors.primary.withOpacity(0.1),
                borderRadius: BorderRadius.circular(4),
                border: Border.all(color: MagiColors.primary.withOpacity(0.3)),
              ),
              child: Text('BETA', style: GoogleFonts.inter(color: MagiColors.accent, fontSize: 10, fontWeight: FontWeight.bold)),
            ),
          ],
        ),
        actions: [
          IconButton(
            icon: const Icon(Icons.help_outline_rounded, size: 20),
            onPressed: () => _showHelpDialog(context),
            tooltip: 'Usage Instructions',
          ),
          const Icon(Icons.share_rounded, size: 20, color: MagiColors.textSecondary),
          const SizedBox(width: 16),
        ],
      ),
      body: Column(
        children: [
          InferenceHeader(unitStates: chatState.unitStates),
          
          if (chatState.isProcessing) 
            TelemetryDashboard(state: chatState),

          Expanded(
            child: ListView.builder(
              padding: const EdgeInsets.symmetric(vertical: 20),
              itemCount: chatState.messages.length,
              itemBuilder: (context, index) {
                final msg = chatState.messages[index];
                return MessageBubble(msg: msg);
              },
            ),
          ),
          
          if (chatState.isProcessing && chatState.metrics['documents'] == 0)
            const Padding(
              padding: EdgeInsets.symmetric(horizontal: 40),
              child: LinearProgressIndicator(
                backgroundColor: Colors.transparent,
                minHeight: 1,
              ),
            ),
          
          ChatInput(
            controller: _textController,
            focusNode: _focusNode,
            onSend: _handleSend,
          ),
        ],
      ),
    );
  }
}
