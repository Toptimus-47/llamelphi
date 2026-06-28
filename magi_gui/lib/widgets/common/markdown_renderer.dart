import 'package:flutter/material.dart';
import 'package:flutter_markdown/flutter_markdown.dart';
import 'package:markdown/markdown.dart' as md;
import 'package:flutter_math_fork/flutter_math.dart';
import 'package:google_fonts/google_fonts.dart';

import '../../core/theme/magi_colors.dart';

class MarkdownRenderer extends StatelessWidget {
  final String data;
  final void Function(String text, String? href, String title)? onTapLink;

  const MarkdownRenderer({
    super.key,
    required this.data,
    this.onTapLink,
  });

  @override
  Widget build(BuildContext context) {
    return MarkdownBody(
      data: _preprocessMarkdown(data),
      selectable: true,
      onTapLink: onTapLink,
      styleSheet: MarkdownStyleSheet(
        p: GoogleFonts.inter(color: MagiColors.textPrimary, fontSize: 15, height: 1.6),
        code: GoogleFonts.firaCode(
          backgroundColor: MagiColors.background,
          color: MagiColors.accent,
          fontSize: 13,
        ),
        codeblockDecoration: BoxDecoration(
          color: MagiColors.background,
          borderRadius: BorderRadius.circular(8),
          border: Border.all(color: MagiColors.outline),
        ),
        blockquote: GoogleFonts.inter(color: MagiColors.textSecondary, fontStyle: FontStyle.italic),
        blockquoteDecoration: const BoxDecoration(
          border: Border(left: BorderSide(color: MagiColors.primary, width: 4)),
        ),
        h1: GoogleFonts.inter(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 24),
        h2: GoogleFonts.inter(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 20),
        h3: GoogleFonts.inter(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 18),
      ),
      builders: {
        'latex': LatexBuilder(),
      },
    );
  }

  static String _preprocessMarkdown(String input) {
    // Convert $$...$$ to <latex displayMode="true">...</latex> for display mode
    String result = input.replaceAllMapped(RegExp(r'\$\$(.*?)\$\$', dotAll: true), (match) {
      return '<latex displayMode="true">${match.group(1)}</latex>';
    });
    // Convert $...$ to <latex>...</latex> for inline mode
    result = result.replaceAllMapped(RegExp(r'\$(.*?)\$'), (match) {
      return '<latex>${match.group(1)}</latex>';
    });
    return result;
  }
}

class LatexBuilder extends MarkdownElementBuilder {
  @override
  Widget? visitElementAfter(md.Element element, TextStyle? preferredStyle) {
    final String text = element.textContent;
    final bool isDisplayMode = element.attributes['displayMode'] == 'true';

    return Math.tex(
      text,
      mathStyle: isDisplayMode ? MathStyle.display : MathStyle.text,
      textStyle: preferredStyle?.copyWith(color: MagiColors.accent),
      onErrorFallback: (err) => Text(
        text,
        style: preferredStyle?.copyWith(color: Colors.redAccent),
      ),
    );
  }
}
