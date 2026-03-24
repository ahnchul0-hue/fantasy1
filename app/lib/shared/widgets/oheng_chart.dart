import 'package:flutter/material.dart';
import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../core/theme/app_motion.dart';
import '../../shared/models/saju_profile.dart';

/// 오행 차트 — 木火土金水 밸런스 바 차트
/// 오행 colors ONLY on charts/data marks
class OhengChart extends StatefulWidget {
  final OhengBalance balance;
  final bool mini;

  const OhengChart({
    super.key,
    required this.balance,
    this.mini = false,
  });

  @override
  State<OhengChart> createState() => _OhengChartState();
}

class _OhengChartState extends State<OhengChart>
    with SingleTickerProviderStateMixin {
  late final AnimationController _controller;
  late final Animation<double> _animation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: AppMotion.chartAnimationDuration,
    );
    _animation = CurvedAnimation(
      parent: _controller,
      curve: AppMotion.chartCurve,
    );
    _controller.forward();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  static const _ohengColors = [
    AppColors.ohengWood,
    AppColors.ohengFire,
    AppColors.ohengEarth,
    AppColors.ohengMetal,
    AppColors.ohengWater,
  ];

  @override
  Widget build(BuildContext context) {
    final entries = widget.balance.entries;
    final barHeight = widget.mini ? 8.0 : 16.0;

    return AnimatedBuilder2(
      animation: _animation,
      builder: (context, _) {
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: List.generate(entries.length, (i) {
            final entry = entries[i];
            return Padding(
              padding: EdgeInsets.only(
                bottom: widget.mini ? AppSpacing.xs : AppSpacing.sm,
              ),
              child: Row(
                children: [
                  SizedBox(
                    width: widget.mini ? 24 : 40,
                    child: Semantics(
                      label: '${entry.hangul}, ${entry.hanja}',
                      child: Text(
                        widget.mini ? entry.hanja : '${entry.hanja}${entry.hangul}',
                        style: (widget.mini
                                ? AppTypography.caption
                                : AppTypography.body)
                            .copyWith(
                          fontFamily: AppTypography.fontHanja,
                          color: _ohengColors[i],
                        ),
                      ),
                    ),
                  ),
                  const SizedBox(width: AppSpacing.sm),
                  Expanded(
                    child: Container(
                      height: barHeight,
                      decoration: BoxDecoration(
                        color: AppColors.divider.withValues(alpha: 0.3),
                        borderRadius: BorderRadius.circular(barHeight / 2),
                      ),
                      child: FractionallySizedBox(
                        alignment: Alignment.centerLeft,
                        widthFactor: entry.ratio * _animation.value,
                        child: Container(
                          decoration: BoxDecoration(
                            color: _ohengColors[i],
                            borderRadius:
                                BorderRadius.circular(barHeight / 2),
                          ),
                        ),
                      ),
                    ),
                  ),
                  if (!widget.mini) ...[
                    const SizedBox(width: AppSpacing.sm),
                    SizedBox(
                      width: 32,
                      child: Text(
                        '${(entry.ratio * 100).round()}%',
                        style: AppTypography.caption,
                        textAlign: TextAlign.right,
                      ),
                    ),
                  ],
                ],
              ),
            );
          }),
        );
      },
    );
  }
}

class AnimatedBuilder2 extends AnimatedWidget {
  final Widget Function(BuildContext, Widget?) builder;

  const AnimatedBuilder2({
    super.key,
    required Animation<double> animation,
    required this.builder,
  }) : super(listenable: animation);

  @override
  Widget build(BuildContext context) {
    return builder(context, null);
  }
}
