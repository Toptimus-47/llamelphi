import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:google_fonts/google_fonts.dart';
import '../../providers/chat_provider.dart';
import '../../providers/ui_provider.dart';
import '../../core/theme/magi_colors.dart';
import '../common/markdown_renderer.dart';

class MessageBubble extends ConsumerStatefulWidget {
  final ChatMessage msg;

  const MessageBubble({super.key, required this.msg});

  @override
  ConsumerState<MessageBubble> createState() => _MessageBubbleState();
}

class _MessageBubbleState extends ConsumerState<MessageBubble> {
  bool isReasoningExpanded = false;

  @override
  Widget build(BuildContext context) {
    final bool isUser = widget.msg.role == 'user';
    final hasReasoning = widget.msg.reasoningLog != null && widget.msg.reasoningLog!.isNotEmpty;
    
    return Align(
      alignment: Alignment.center,
      child: Container(
        constraints: const BoxConstraints(maxWidth: 800),
        padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 16),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            _buildAvatar(isUser, widget.msg.unit),
            const SizedBox(width: 16),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Text(
                        isUser ? 'You' : (widget.msg.unit ?? 'MAGI'),
                        style: GoogleFonts.inter(fontWeight: FontWeight.w700, fontSize: 14, color: Colors.white),
                      ),
                      if (!isUser && widget.msg.unit != null) ...[
                        const SizedBox(width: 8),
                        _buildUnitBadge(widget.msg.unit!),
                      ],
                    ],
                  ),
                  const SizedBox(height: 8),
                  
                  if (hasReasoning) ...[
                    _buildReasoningSection(widget.msg.reasoningLog!),
                    const SizedBox(height: 12),
                  ],

                  if (widget.msg.content.isNotEmpty)
                    MarkdownRenderer(
                      data: widget.msg.content,
                      onTapLink: (text, href, title) {
                        // Handle citation clicks: e.g., href could be "pdf://path/to/file.pdf"
                        if (href != null && href.startsWith('pdf://')) {
                          final path = href.replaceFirst('pdf://', '');
                          ref.read(uiProvider.notifier).toggleContextPanel(
                            path: path,
                            title: text,
                          );
                        } else {
                          // Default behavior for other links
                        }
                      },
                    ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildReasoningSection(String log) {
    return Container(
      decoration: BoxDecoration(
        color: MagiColors.background,
        borderRadius: BorderRadius.circular(10),
        border: Border.all(color: MagiColors.surfaceVariant),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          InkWell(
            onTap: () => setState(() => isReasoningExpanded = !isReasoningExpanded),
            borderRadius: BorderRadius.circular(10),
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
              child: Row(
                children: [
                  const Icon(Icons.psychology_outlined, size: 16, color: Colors.white38),
                  const SizedBox(width: 8),
                  Text(
                    'Thought Process (DeepSeek-R2)',
                    style: GoogleFonts.inter(fontSize: 12, color: Colors.white38, fontWeight: FontWeight.w500),
                  ),
                  const Spacer(),
                  Icon(
                    isReasoningExpanded ? Icons.keyboard_arrow_up_rounded : Icons.keyboard_arrow_down_rounded,
                    size: 16,
                    color: Colors.white38,
                  ),
                ],
              ),
            ),
          ),
          if (isReasoningExpanded)
            Padding(
              padding: const EdgeInsets.fromLTRB(12, 0, 12, 12),
              child: Text(
                log,
                style: GoogleFonts.inter(fontSize: 13, color: MagiColors.textSecondary, fontStyle: FontStyle.italic, height: 1.5),
              ),
            ),
        ],
      ),
    );
  }

  Widget _buildAvatar(bool isUser, String? unit) {
    if (isUser) {
      return CircleAvatar(
        radius: 16,
        backgroundColor: Colors.indigo.shade700,
        child: const Icon(Icons.person_rounded, size: 18, color: Colors.white),
      );
    }
    
    return Container(
      width: 32,
      height: 32,
      decoration: BoxDecoration(
        color: _getUnitColor(unit).withOpacity(0.1),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: _getUnitColor(unit).withOpacity(0.4)),
      ),
      child: Icon(
        unit != null && unit.contains('Critic') ? Icons.gavel_rounded : Icons.auto_awesome_rounded,
        size: 18, 
        color: _getUnitColor(unit),
      ),
    );
  }

  Widget _buildUnitBadge(String unit) {
    final bool isCritic = unit.contains('Critic');
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 1),
      decoration: BoxDecoration(
        color: isCritic ? Colors.red.withOpacity(0.1) : _getUnitColor(unit).withOpacity(0.1),
        borderRadius: BorderRadius.circular(4),
        border: Border.all(color: isCritic ? Colors.red.withOpacity(0.3) : _getUnitColor(unit).withOpacity(0.3)),
      ),
      child: Text(
        isCritic ? 'CRITIC' : 'UNIT',
        style: GoogleFonts.inter(
          color: isCritic ? Colors.redAccent : _getUnitColor(unit),
          fontSize: 8,
          fontWeight: FontWeight.bold,
        ),
      ),
    );
  }

  Color _getUnitColor(String? unit) {
    if (unit == null) return Colors.indigoAccent;
    if (unit.contains('Melchior')) return Colors.orangeAccent;
    if (unit.contains('Balthasar')) return Colors.greenAccent;
    if (unit.contains('Casper')) return Colors.blueAccent;
    if (unit.contains('Artaban')) return Colors.purpleAccent;
    if (unit.contains('Gushnasaph')) return Colors.cyanAccent;
    if (unit.contains('Kagba')) return Colors.redAccent;
    if (unit.contains('Orchestrator')) return Colors.indigoAccent;
    return Colors.indigoAccent;
  }
}
