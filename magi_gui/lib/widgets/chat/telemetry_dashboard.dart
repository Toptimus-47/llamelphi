import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import '../../providers/chat_provider.dart';
import '../../core/theme/magi_colors.dart';

class TelemetryDashboard extends StatelessWidget {
  final ChatState state;

  const TelemetryDashboard({super.key, required this.state});

  @override
  Widget build(BuildContext context) {
    final metrics = state.metrics;
    final keywords = state.searchKeywords;

    return Container(
      width: double.infinity,
      margin: const EdgeInsets.symmetric(horizontal: 20, vertical: 8),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: MagiColors.surface,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: MagiColors.surfaceVariant),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (keywords.isNotEmpty) ...[
            Row(
              children: [
                const Icon(Icons.manage_search_rounded, size: 16, color: Colors.indigoAccent),
                const SizedBox(width: 8),
                Text('Active Search Vectors', style: GoogleFonts.inter(fontSize: 12, fontWeight: FontWeight.bold, color: Colors.white70)),
              ],
            ),
            const SizedBox(height: 8),
            Wrap(
              spacing: 8,
              runSpacing: 8,
              children: keywords.map((k) => Container(
                padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
                decoration: BoxDecoration(
                  color: Colors.indigo.withOpacity(0.1),
                  borderRadius: BorderRadius.circular(20),
                  border: Border.all(color: Colors.indigo.withOpacity(0.3)),
                ),
                child: Text(k, style: GoogleFonts.inter(fontSize: 11, color: Colors.indigoAccent)),
              )).toList(),
            ),
            const Divider(height: 24, color: MagiColors.surfaceVariant),
          ],
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: [
              _buildMetricItem('DOCUMENTS', '${metrics['documents']}', Icons.article_outlined),
              _buildMetricItem('DATA SIZE', '${metrics['size_kb']} KB', Icons.data_usage_rounded),
              _buildMetricItem('EST. TOKENS', '${metrics['est_tokens']}', Icons.generating_tokens_outlined),
            ],
          ),
          if (metrics['current_query'] != '') ...[
            const SizedBox(height: 12),
            Text(
              'Currently scraping: ${metrics['current_query']}',
              style: GoogleFonts.inter(fontSize: 10, color: Colors.white38, fontStyle: FontStyle.italic),
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildMetricItem(String label, String value, IconData icon) {
    return Column(
      children: [
        Icon(icon, size: 18, color: Colors.white38),
        const SizedBox(height: 4),
        Text(value, style: GoogleFonts.firaCode(fontSize: 16, fontWeight: FontWeight.bold, color: Colors.white)),
        Text(label, style: GoogleFonts.inter(fontSize: 9, color: Colors.white38, letterSpacing: 1)),
      ],
    );
  }
}
