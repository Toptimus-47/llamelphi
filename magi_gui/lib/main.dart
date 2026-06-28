import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:window_manager/window_manager.dart';

import 'core/theme/magi_theme.dart';
import 'core/constants/app_constants.dart';
import 'core/ffi/magi_ffi.dart';
import 'screens/main_screen.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await windowManager.ensureInitialized();

  // --- Start MAGI Backend via Binary FFI ---
  MagiFfi.init();
  final result = MagiFfi.startBackend();
  print("[MAGI System] Backend Start Result: $result");

  WindowOptions windowOptions = const WindowOptions(
    size: Size(1280, 800),
    center: true,
    backgroundColor: Colors.transparent,
    skipTaskbar: false,
    titleBarStyle: TitleBarStyle.normal,
    title: AppConstants.appTitle,
  );
  
  windowManager.waitUntilReadyToShow(windowOptions, () async {
    await windowManager.show();
    await windowManager.focus();
  });

  runApp(const ProviderScope(child: MagiApp()));
}

class MagiApp extends StatelessWidget {
  const MagiApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: AppConstants.appTitle,
      debugShowCheckedModeBanner: false,
      theme: MagiTheme.darkTheme,
      home: const MainScreen(),
    );
  }
}
