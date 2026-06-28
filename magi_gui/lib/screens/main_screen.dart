import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:window_manager/window_manager.dart';
import 'package:google_fonts/google_fonts.dart';

import '../providers/chat_provider.dart';
import '../providers/session_provider.dart';
import '../providers/locale_provider.dart';
import '../providers/ui_provider.dart';
import '../services/magi_api_service.dart';
import '../core/theme/magi_colors.dart';
import '../widgets/common/pdf_viewer_panel.dart';
import '../widgets/sidebar/magi_sidebar.dart';
import 'chat_screen.dart';

class MainScreen extends ConsumerStatefulWidget {
  const MainScreen({super.key});

  @override
  ConsumerState<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends ConsumerState<MainScreen> with WindowListener {
  final MagiApiService _api = MagiApiService();

  @override
  void initState() {
    super.initState();
    windowManager.addListener(this);
    _initWindowManager();
  }

  void _initWindowManager() async {
    await windowManager.setPreventClose(true);
  }

  @override
  void dispose() {
    windowManager.removeListener(this);
    super.dispose();
  }

  @override
  void onWindowClose() async {
    bool isPreventClose = await windowManager.isPreventClose();
    if (isPreventClose) {
      await _api.shutdown();
      await windowManager.destroy();
    }
  }

  @override
  Widget build(BuildContext context) {
    final uiState = ref.watch(uiProvider);
    final chatState = ref.watch(chatProvider);
    final sessionState = ref.watch(sessionProvider);
    final isWide = MediaQuery.of(context).size.width > 1200;

    return Scaffold(
      backgroundColor: MagiColors.background,
      body: Row(
        children: [
          // 1. Sidebar
          if (uiState.isSidebarVisible)
            const MagiSidebar(),
          
          // 2. Chat Main
          Expanded(
            child: ChatScreen(
              toggleSidebar: () => ref.read(uiProvider.notifier).toggleSidebar(),
              isSidebarVisible: uiState.isSidebarVisible,
            ),
          ),

          // 3. Context Panel (PDF Viewer)
          if (uiState.isContextPanelVisible)
            PdfViewerPanel(
              pdfPath: uiState.selectedPdfPath,
              title: uiState.selectedPdfTitle ?? 'Reference Document',
              onClose: () => ref.read(uiProvider.notifier).closeContextPanel(),
            ),
        ],
      ),
    );
  }
}
