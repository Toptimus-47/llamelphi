import 'package:flutter/material.dart';
import 'package:fl_chart/fl_chart.dart';

class VisualizationChart extends StatefulWidget {
  final Map<String, dynamic> data;

  const VisualizationChart({super.key, required this.data});

  @override
  State<VisualizationChart> createState() => _VisualizationChartState();
}

class _VisualizationChartState extends State<VisualizationChart> {
  int touchedIndex = -1;

  @override
  Widget build(BuildContext context) {
    final type = widget.data['type'] ?? 'line';
    final labels = List<String>.from(widget.data['labels'] ?? []);
    final values = List<double>.from((widget.data['values'] ?? []).map((v) => v.toDouble()));
    final title = widget.data['title'] ?? 'Strategic Data Insight';

    if (labels.isEmpty || values.isEmpty) return const SizedBox.shrink();

    return Container(
      height: 300,
      margin: const EdgeInsets.symmetric(vertical: 12),
      padding: const EdgeInsets.all(20),
      decoration: BoxDecoration(
        gradient: LinearGradient(
          colors: [
            Theme.of(context).colorScheme.surfaceVariant.withOpacity(0.3),
            Theme.of(context).colorScheme.surface.withOpacity(0.7),
          ],
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
        ),
        borderRadius: BorderRadius.circular(24),
        border: Border.all(color: Theme.of(context).colorScheme.outlineVariant.withOpacity(0.5)),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.05),
            blurRadius: 10,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text(
                title,
                style: Theme.of(context).textTheme.titleSmall?.copyWith(
                  fontWeight: FontWeight.bold,
                  letterSpacing: 0.5,
                  color: Theme.of(context).colorScheme.primary,
                ),
              ),
              Icon(
                type == 'pie' ? Icons.pie_chart : (type == 'bar' ? Icons.bar_chart : Icons.show_chart),
                size: 16,
                color: Theme.of(context).colorScheme.primary.withOpacity(0.5),
              ),
            ],
          ),
          const SizedBox(height: 24),
          Expanded(
            child: _buildChart(type, labels, values),
          ),
        ],
      ),
    );
  }

  Widget _buildChart(String type, List<String> labels, List<double> values) {
    switch (type) {
      case 'pie':
        return _buildPieChart(labels, values);
      case 'bar':
        return _buildBarChart(labels, values);
      default:
        return _buildLineChart(labels, values);
    }
  }

  Widget _buildLineChart(List<String> labels, List<double> values) {
    return LineChart(
      LineChartData(
        lineTouchData: LineTouchData(
          touchTooltipData: LineTouchTooltipData(
            tooltipBgColor: Theme.of(context).colorScheme.secondaryContainer,
            getTooltipItems: (touchedSpots) {
              return touchedSpots.map((spot) {
                return LineTooltipItem(
                  '${labels[spot.x.toInt()]}: ${spot.y}',
                  TextStyle(color: Theme.of(context).colorScheme.onSecondaryContainer, fontWeight: FontWeight.bold),
                );
              }).toList();
            },
          ),
        ),
        gridData: const FlGridData(show: false),
        titlesData: _buildTitles(labels),
        borderData: FlBorderData(show: false),
        lineBarsData: [
          LineChartBarData(
            spots: values.asMap().entries.map((e) => FlSpot(e.key.toDouble(), e.value)).toList(),
            isCurved: true,
            curveSmoothness: 0.35,
            color: Theme.of(context).colorScheme.primary,
            barWidth: 4,
            isStrokeCapRound: true,
            dotData: const FlDotData(show: true),
            belowBarData: BarAreaData(
              show: true,
              gradient: LinearGradient(
                colors: [
                  Theme.of(context).colorScheme.primary.withOpacity(0.3),
                  Theme.of(context).colorScheme.primary.withOpacity(0.0),
                ],
                begin: Alignment.topCenter,
                end: Alignment.bottomCenter,
              ),
            ),
          ),
        ],
      ),
      duration: const Duration(milliseconds: 1500),
      curve: Curves.easeInOutBack,
    );
  }

  Widget _buildBarChart(List<String> labels, List<double> values) {
    return BarChart(
      BarChartData(
        barTouchData: BarTouchData(
          touchTooltipData: BarTouchTooltipData(
            tooltipBgColor: Theme.of(context).colorScheme.secondaryContainer,
          ),
        ),
        gridData: const FlGridData(show: false),
        titlesData: _buildTitles(labels),
        borderData: FlBorderData(show: false),
        barGroups: values.asMap().entries.map((e) => BarChartGroupData(
          x: e.key,
          barRods: [
            BarChartRodData(
              toY: e.value, 
              color: Theme.of(context).colorScheme.primary, 
              width: 20, 
              borderRadius: const BorderRadius.vertical(top: Radius.circular(6)),
              backDrawRodData: BackgroundBarChartRodData(
                show: true,
                toY: values.reduce((a, b) => a > b ? a : b) * 1.1,
                color: Theme.of(context).colorScheme.surfaceVariant,
              ),
            )
          ],
        )).toList(),
      ),
      duration: const Duration(milliseconds: 1000),
      curve: Curves.easeOutQuart,
    );
  }

  Widget _buildPieChart(List<String> labels, List<double> values) {
    return PieChart(
      PieChartData(
        pieTouchData: PieTouchData(
          touchCallback: (FlTouchEvent event, pieTouchResponse) {
            setState(() {
              if (!event.isInterestedForInteractions ||
                  pieTouchResponse == null ||
                  pieTouchResponse.touchedSection == null) {
                touchedIndex = -1;
                return;
              }
              touchedIndex = pieTouchResponse.touchedSection!.touchedSectionIndex;
            });
          },
        ),
        borderData: FlBorderData(show: false),
        sectionsSpace: 4,
        centerSpaceRadius: 40,
        sections: values.asMap().entries.map((e) {
          final isTouched = e.key == touchedIndex;
          final fontSize = isTouched ? 16.0 : 12.0;
          final radius = isTouched ? 60.0 : 50.0;
          final colorList = [
            Colors.blueAccent, Colors.purpleAccent, Colors.orangeAccent, 
            Colors.greenAccent, Colors.redAccent, Colors.cyanAccent
          ];
          
          return PieChartSectionData(
            color: colorList[e.key % colorList.length],
            value: e.value,
            title: isTouched ? '${labels[e.key]}\n${e.value}' : labels[e.key],
            radius: radius,
            titleStyle: TextStyle(
              fontSize: fontSize, 
              fontWeight: FontWeight.bold, 
              color: Colors.white,
              shadows: const [Shadow(color: Colors.black26, blurRadius: 2)],
            ),
          );
        }).toList(),
      ),
    );
  }

  FlTitlesData _buildTitles(List<String> labels) {
    return FlTitlesData(
      bottomTitles: AxisTitles(
        sideTitles: SideTitles(
          showTitles: true,
          getTitlesWidget: (value, meta) {
            if (value.toInt() >= 0 && value.toInt() < labels.length) {
              return SideTitleWidget(
                axisSide: meta.axisSide,
                child: Text(
                  labels[value.toInt()], 
                  style: TextStyle(
                    fontSize: 10, 
                    color: Theme.of(context).colorScheme.onSurfaceVariant,
                    fontWeight: FontWeight.w500,
                  ),
                ),
              );
            }
            return const SizedBox.shrink();
          },
          reservedSize: 30,
        ),
      ),
      leftTitles: AxisTitles(
        sideTitles: SideTitles(
          showTitles: true, 
          reservedSize: 35,
          getTitlesWidget: (value, meta) => SideTitleWidget(
            axisSide: meta.axisSide,
            child: Text(
              value.toInt().toString(),
              style: TextStyle(fontSize: 10, color: Theme.of(context).colorScheme.onSurfaceVariant),
            ),
          ),
        ),
      ),
      topTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
      rightTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
    );
  }
}
