import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:google_fonts/google_fonts.dart';

import '../../providers/chat_provider.dart';
import '../../providers/session_provider.dart';
import '../../providers/locale_provider.dart';
import '../../core/theme/magi_colors.dart';
import '../../services/magi_api_service.dart';

class MagiSidebar extends ConsumerWidget {
  const MagiSidebar({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final sessionState = ref.watch(sessionProvider);
    final chatState = ref.watch(chatProvider);
    final locale = ref.watch(localeProvider);

    return Container(
      width: 260,
      decoration: const BoxDecoration(
        color: MagiColors.surface,
        border: Border(right: BorderSide(color: MagiColors.surfaceVariant, width: 1)),
      ),
      child: Column(
        children: [
          // Sidebar Header
          Padding(
            padding: const EdgeInsets.all(16.0),
            child: InkWell(
              onTap: () => ref.read(chatProvider.notifier).newSession(),
              child: Container(
                padding: const EdgeInsets.symmetric(vertical: 10, horizontal: 16),
                decoration: BoxDecoration(
                  borderRadius: BorderRadius.circular(8),
                  border: Border.all(color: MagiColors.outline),
                ),
                child: Row(
                  children: [
                    const Icon(Icons.add_rounded, size: 20, color: Colors.white),
                    const SizedBox(width: 12),
                    Text(
                      locale.translate('new_chat'),
                      style: GoogleFonts.inter(fontWeight: FontWeight.w600, color: Colors.white),
                    ),
                  ],
                ),
              ),
            ),
          ),

          // Session List
          Expanded(
            child: ListView.builder(
              padding: const EdgeInsets.symmetric(horizontal: 12),
              itemCount: sessionState.sessions.length,
              itemBuilder: (context, index) {
                final session = sessionState.sessions[index];
                final isSelected = session.id == chatState.currentSessionId;
                return Padding(
                  padding: const EdgeInsets.only(bottom: 4),
                  child: ListTile(
                    onTap: () => ref.read(chatProvider.notifier).loadSession(session.id),
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                    tileColor: isSelected ? MagiColors.surfaceVariant : Colors.transparent,
                    title: Text(
                      session.title,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      style: GoogleFonts.inter(
                        fontSize: 13,
                        color: isSelected ? Colors.white : MagiColors.textSecondary,
                        fontWeight: isSelected ? FontWeight.w600 : FontWeight.normal,
                      ),
                    ),
                    dense: true,
                  ),
                );
              },
            ),
          ),

          const Divider(color: MagiColors.surfaceVariant, height: 1),
          
          // Settings / Model Hot-Swap
          ListTile(
            onTap: () => _showModelSettings(context, ref),
            leading: const Icon(Icons.settings_suggest_rounded, size: 20, color: MagiColors.textSecondary),
            title: Text(
              'Model Settings',
              style: GoogleFonts.inter(fontSize: 13, color: MagiColors.textSecondary),
            ),
            dense: true,
          ),

          const Divider(color: MagiColors.surfaceVariant, height: 1),
          
          // User Profile / Settings
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
            child: Row(
              children: [
                _buildSidebarUserAvatar(),
                const SizedBox(width: 12),
                Text(
                  'Researcher',
                  style: GoogleFonts.inter(fontSize: 14, fontWeight: FontWeight.w600, color: Colors.white),
                ),
                const Spacer(),
                IconButton(
                  icon: const Icon(Icons.language_rounded, size: 18, color: Colors.white38),
                  onPressed: () => ref.read(localeProvider.notifier).toggleLanguage(),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSidebarUserAvatar() {
    return Container(
      width: 32,
      height: 32,
      decoration: BoxDecoration(
        color: MagiColors.surfaceVariant,
        borderRadius: BorderRadius.circular(8),
      ),
      alignment: Alignment.center,
      child: const Icon(Icons.person_rounded, size: 18, color: Colors.white),
    );
  }

  void _showModelSettings(BuildContext context, WidgetRef ref) {
    final units = ['Melchior', 'Balthasar', 'Casper', 'Artaban', 'Gushnasaph', 'Kagba'];
    final api = MagiApiService();

    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        backgroundColor: MagiColors.surface,
        title: Text('Model Hot-Swap', style: GoogleFonts.inter(color: Colors.white)),
        content: SizedBox(
          width: 400,
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: units.map((unit) => ListTile(
              title: Text(unit, style: GoogleFonts.inter(color: Colors.white)),
              subtitle: Text('Current Engine: Candle (Local)', style: GoogleFonts.inter(color: MagiColors.textSecondary, fontSize: 11)),
              trailing: const Icon(Icons.folder_open_rounded, color: MagiColors.primary),
              onTap: () async {
                // In a real app, use file_picker. 
                // Simulation: Toggle between two model names for demo
                final result = await api.hotSwapModel(unit, "models/SmolLM2-1.7B-Instruct-Q4_K_M.gguf");
                if (context.mounted) {
                  ScaffoldMessenger.of(context).showSnackBar(
                    SnackBar(content: Text(result['message'] ?? 'Swap Success'))
                  );
                  Navigator.pop(context);
                }
              },
            )).toList(),
          ),
        ),
      ),
    );
  }
}
