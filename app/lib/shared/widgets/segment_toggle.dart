import 'package:flutter/material.dart';
import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';

/// Segment toggle (양력/음력, 남/여) — r20, h36
class SegmentToggle<T> extends StatelessWidget {
  final List<SegmentOption<T>> options;
  final T selected;
  final ValueChanged<T> onChanged;

  const SegmentToggle({
    super.key,
    required this.options,
    required this.selected,
    required this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      height: AppDimensions.segmentToggleHeight,
      decoration: BoxDecoration(
        color: AppColors.divider.withValues(alpha: 0.5),
        borderRadius: BorderRadius.circular(AppRadius.segment),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: options.map((option) {
          final isSelected = option.value == selected;
          return GestureDetector(
            onTap: () => onChanged(option.value),
            child: AnimatedContainer(
              duration: const Duration(milliseconds: 200),
              curve: Curves.easeOutCubic,
              padding: const EdgeInsets.symmetric(horizontal: AppSpacing.md),
              decoration: BoxDecoration(
                color: isSelected ? AppColors.primary : Colors.transparent,
                borderRadius: BorderRadius.circular(AppRadius.segment),
              ),
              alignment: Alignment.center,
              child: Text(
                option.label,
                style: AppTypography.caption.copyWith(
                  color: isSelected ? AppColors.surface : AppColors.onSurface,
                  fontWeight:
                      isSelected ? FontWeight.w600 : FontWeight.w400,
                ),
              ),
            ),
          );
        }).toList(),
      ),
    );
  }
}

class SegmentOption<T> {
  final T value;
  final String label;

  const SegmentOption({required this.value, required this.label});
}
