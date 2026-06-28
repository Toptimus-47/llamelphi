import 'package:flutter/material.dart';
import 'package:fl_chart/fl_chart.dart';
import 'package:google_fonts/google_fonts.dart';

class VisualizationChart extends StatelessWidget {
  final Map<String, dynamic> data;

  const VisualizationChart({super.key, required this.data});

  @override
  Widget build(BuildContext context) {
    final String type = data['type'] ?? 'bar';
    final List<dynamic> labels = data['labels'] ?? [];
    final List<dynamic> values = data['values'] ?? [];

    return Container(
      height: 320,
      padding: const EdgeInsets.all(24),
      decoration: BoxDecoration(
        color: const Color(0xFF18181B), // Zinc-900
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: const Color(0xFF27272A)), // Zinc-800
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              const Icon(Icons.bar_chart_rounded, size: 18, color: Color(0xFF6366F1)),
              const SizedBox(width: 10),
              Text(
                data['title'] ?? 'Analysis Result',
                style: GoogleFonts.inter(
                  color: Colors.white, 
                  fontWeight: FontWeight.w600,
                  fontSize: 14,
                ),
              ),
            ],
          ),
          const SizedBox(height: 24),
          Expanded(
            child: type == 'bar' ? _buildBarChart(labels, values) : _buildLineChart(labels, values),
          ),
        ],
      ),
    );
  }

  Widget _buildBarChart(List<dynamic> labels, List<dynamic> values) {
    return BarChart(
      BarChartData(
        gridData: const FlGridData(show: false),
        borderData: FlBorderData(show: false),
        barGroups: List.generate(values.length, (index) {
          return BarChartGroupData(
            x: index,
            barRods: [
              BarChartRodData(
                toY: (values[index] as num).toDouble(),
                color: const Color(0xFF6366F1),
                width: 20,
                borderRadius: const BorderRadius.vertical(top: Radius.circular(4)),
                backDrawRodData: BackgroundBarChartRodData(
                  show: true,
                  toY: 100, // Assuming 100 is max for simplicity or normalize
                  color: const Color(0xFF27272A),
                ),
              ),
            ],
          );
        }),
        titlesData: FlTitlesData(
          leftTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
          rightTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
          topTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
          bottomTitles: AxisTitles(
            sideTitles: SideTitles(
              showTitles: true,
              getTitlesWidget: (value, meta) {
                if (value.toInt() < labels.length) {
                  return Padding(
                    padding: const EdgeInsets.only(top: 10.0),
                    child: Text(
                      labels[value.toInt()].toString(),
                      style: GoogleFonts.inter(color: const Color(0xFF71717A), fontSize: 10),
                    ),
                  );
                }
                return const Text('');
              },
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildLineChart(List<dynamic> labels, List<dynamic> values) {
    return LineChart(
      LineChartData(
        gridData: const FlGridData(show: false),
        borderData: FlBorderData(show: false),
        lineBarsData: [
          LineChartBarData(
            spots: List.generate(values.length, (index) {
              return FlSpot(index.toDouble(), (values[index] as num).toDouble());
            }),
            isCurved: true,
            color: const Color(0xFF6366F1),
            barWidth: 4,
            isStrokeCapRound: true,
            dotData: const FlDotData(show: false),
            belowBarData: BarAreaData(
              show: true,
              color: const Color(0xFF6366F1).withOpacity(0.1),
            ),
          ),
        ],
        titlesData: FlTitlesData(
          leftTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
          rightTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
          topTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
          bottomTitles: AxisTitles(
            sideTitles: SideTitles(
              showTitles: true,
              getTitlesWidget: (value, meta) {
                if (value.toInt() < labels.length) {
                  return Padding(
                    padding: const EdgeInsets.only(top: 10.0),
                    child: Text(
                      labels[value.toInt()].toString(),
                      style: GoogleFonts.inter(color: const Color(0xFF71717A), fontSize: 10),
                    ),
                  );
                }
                return const Text('');
              },
            ),
          ),
        ),
      ),
    );
  }
}
