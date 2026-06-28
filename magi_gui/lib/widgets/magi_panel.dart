import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import '../core/theme/magi_colors.dart';

class MagiStatusPanel extends StatelessWidget {
  final String unitName;
  final String status; 
  final Color accentColor;

  const MagiStatusPanel({
    super.key,
    required this.unitName,
    required this.status,
    required this.accentColor,
  });

  @override
  Widget build(BuildContext context) {
    final bool isActive = status != 'IDLE' && status != 'OFFLINE';

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
      decoration: BoxDecoration(
        color: isActive ? accentColor.withOpacity(0.05) : Colors.transparent,
        borderRadius: BorderRadius.circular(6),
        border: Border.all(
          color: isActive ? accentColor.withOpacity(0.2) : Colors.white.withOpacity(0.05),
        ),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          _buildStatusDot(isActive),
          const SizedBox(width: 8),
          Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                unitName.toUpperCase(),
                style: GoogleFonts.inter(
                  color: isActive ? Colors.white : Colors.white38,
                  fontSize: 10,
                  fontWeight: FontWeight.w600,
                  letterSpacing: 0.5,
                ),
              ),
              if (isActive)
                Text(
                  status,
                  style: GoogleFonts.inter(
                    color: accentColor,
                    fontSize: 8,
                    fontWeight: FontWeight.w500,
                  ),
                ),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildStatusDot(bool isActive) {
    return Container(
      width: 6,
      height: 6,
      decoration: BoxDecoration(
        color: isActive ? accentColor : Colors.white12,
        shape: BoxShape.circle,
        boxShadow: isActive ? [
          BoxShadow(color: accentColor.withOpacity(0.5), blurRadius: 4, spreadRadius: 1)
        ] : [],
      ),
    );
  }
}
