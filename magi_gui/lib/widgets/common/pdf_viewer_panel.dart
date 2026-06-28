import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import 'package:pdfrx/pdfrx.dart';

import '../../core/theme/magi_colors.dart';

class PdfViewerPanel extends StatelessWidget {
  final String? pdfPath;
  final String title;
  final VoidCallback onClose;

  const PdfViewerPanel({
    super.key,
    this.pdfPath,
    required this.title,
    required this.onClose,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 400,
      decoration: const BoxDecoration(
        color: MagiColors.surface,
        border: Border(left: BorderSide(color: MagiColors.surfaceVariant, width: 1)),
      ),
      child: Column(
        children: [
          // Header
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
            decoration: const BoxDecoration(
              border: Border(bottom: BorderSide(color: MagiColors.surfaceVariant, width: 1)),
            ),
            child: Row(
              children: [
                const Icon(Icons.description_rounded, size: 20, color: MagiColors.primary),
                const SizedBox(width: 12),
                Expanded(
                  child: Text(
                    title,
                    style: GoogleFonts.inter(
                      fontSize: 14,
                      fontWeight: FontWeight.w600,
                      color: Colors.white,
                    ),
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.close_rounded, size: 20, color: MagiColors.textMuted),
                  onPressed: onClose,
                ),
              ],
            ),
          ),
          
          // PDF Content or Placeholder
          Expanded(
            child: pdfPath != null
                ? PdfViewer.file(pdfPath!)
                : _buildPlaceholder(),
          ),
        ],
      ),
    );
  }

  Widget _buildPlaceholder() {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(Icons.auto_stories_rounded, size: 48, color: MagiColors.surfaceVariant),

          const SizedBox(height: 16),
          Text(
            'No Document Selected',
            style: GoogleFonts.inter(color: MagiColors.textMuted, fontSize: 14),
          ),
        ],
      ),
    );
  }
}
