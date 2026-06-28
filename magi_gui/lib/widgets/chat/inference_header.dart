import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import '../../core/theme/magi_colors.dart';

class InferenceHeader extends StatelessWidget {
  final Map<String, String> unitStates;

  const InferenceHeader({super.key, required this.unitStates});

  @override
  Widget build(BuildContext context) {
    final units = ['Melchior', 'Balthasar', 'Casper', 'Artaban', 'Gushnasaph', 'Kagba'];

    return Container(
      padding: const EdgeInsets.symmetric(vertical: 12, horizontal: 20),
      decoration: const BoxDecoration(
        color: MagiColors.surface,
        border: Border(bottom: BorderSide(color: MagiColors.surfaceVariant, width: 1)),
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: units.map((unit) {
          final state = unitStates[unit] ?? 'IDLE';
          return _buildUnitStatus(unit, state);
        }).toList(),
      ),
    );
  }

  Widget _buildUnitStatus(String name, String state) {
    final bool isActive = state == 'WORKING';
    final bool isDone = state == 'DONE';

    return Column(
      children: [
        Text(
          name[0], // First letter
          style: GoogleFonts.inter(
            fontSize: 10,
            fontWeight: FontWeight.bold,
            color: isActive || isDone ? MagiColors.primary : MagiColors.textMuted,
          ),
        ),
        const SizedBox(height: 4),
        Container(
          width: 40,
          height: 3,
          decoration: BoxDecoration(
            color: isDone 
                ? MagiColors.primary 
                : (isActive ? MagiColors.primary.withOpacity(0.4) : MagiColors.surfaceVariant),
            borderRadius: BorderRadius.circular(2),
            boxShadow: isActive ? [
              BoxShadow(color: MagiColors.primary.withOpacity(0.5), blurRadius: 4, spreadRadius: 1)
            ] : null,
          ),
        ),
      ],
    );
  }
}
