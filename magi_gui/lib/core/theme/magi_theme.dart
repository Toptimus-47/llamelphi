import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import 'magi_colors.dart';

class MagiTheme {
  static ThemeData get darkTheme {
    final baseTextTheme = GoogleFonts.interTextTheme(ThemeData.dark().textTheme);
    
    return ThemeData(
      useMaterial3: true,
      brightness: Brightness.dark,
      scaffoldBackgroundColor: MagiColors.background,
      colorScheme: ColorScheme.fromSeed(
        seedColor: const Color(0xFF10B981),
        brightness: Brightness.dark,
        primary: MagiColors.primary,
        surface: MagiColors.surface,
        surfaceVariant: MagiColors.surfaceVariant,
        outline: MagiColors.outline,
      ),
      textTheme: baseTextTheme.copyWith(
        displayLarge: GoogleFonts.inter(fontWeight: FontWeight.w700, letterSpacing: -1),
        titleMedium: GoogleFonts.inter(fontWeight: FontWeight.w600, color: Colors.white),
        bodyMedium: baseTextTheme.bodyMedium?.copyWith(
          color: MagiColors.textPrimary,
          height: 1.6,
          fontSize: 15,
        ),
      ),
      cardTheme: CardThemeData(
        color: MagiColors.surface,
        elevation: 0,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: const BorderSide(color: MagiColors.surfaceVariant),
        ),
      ),
    );
  }
}
