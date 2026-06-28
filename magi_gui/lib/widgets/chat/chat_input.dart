import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import '../../core/theme/magi_colors.dart';

class ChatInput extends StatelessWidget {
  final TextEditingController controller;
  final FocusNode focusNode;
  final VoidCallback onSend;

  const ChatInput({
    super.key,
    required this.controller,
    required this.focusNode,
    required this.onSend,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.fromLTRB(20, 0, 20, 24),
      alignment: Alignment.center,
      child: Container(
        constraints: const BoxConstraints(maxWidth: 800),
        decoration: BoxDecoration(
          color: MagiColors.surface,
          borderRadius: BorderRadius.circular(16),
          border: Border.all(color: MagiColors.surfaceVariant),
          boxShadow: [
            BoxShadow(
              color: Colors.black.withOpacity(0.2),
              blurRadius: 20,
              offset: const Offset(0, 10),
            ),
          ],
        ),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              controller: controller,
              focusNode: focusNode,
              maxLines: 5,
              minLines: 1,
              style: GoogleFonts.inter(fontSize: 15),
              decoration: InputDecoration(
                hintText: 'Message MAGI...',
                hintStyle: GoogleFonts.inter(color: MagiColors.textMuted),
                contentPadding: const EdgeInsets.all(16),
                border: InputBorder.none,
              ),
            ),
            Padding(
              padding: const EdgeInsets.fromLTRB(12, 0, 12, 12),
              child: Row(
                children: [
                  const Icon(Icons.attachment_rounded, color: MagiColors.textMuted, size: 20),
                  const SizedBox(width: 16),
                  const Icon(Icons.language_rounded, color: MagiColors.textMuted, size: 20),
                  const Spacer(),
                  IconButton.filled(
                    onPressed: onSend,
                    icon: const Icon(Icons.arrow_upward_rounded, size: 18),
                    style: IconButton.styleFrom(
                      backgroundColor: Colors.white,
                      foregroundColor: Colors.black,
                      minimumSize: const Size(32, 32),
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
