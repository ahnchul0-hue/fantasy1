import 'package:flutter/material.dart';
import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../shared/models/birth_input.dart';

/// 12시진 그리드 (4열 x 3행) + "모르겠습니다" 전폭 버튼
class BirthHourGrid extends StatelessWidget {
  final BirthHour selected;
  final ValueChanged<BirthHour> onChanged;

  const BirthHourGrid({
    super.key,
    required this.selected,
    required this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    final siJin = BirthHour.siJin;

    return Column(
      children: [
        // 4 columns x 3 rows grid
        GridView.builder(
          shrinkWrap: true,
          physics: const NeverScrollableScrollPhysics(),
          gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
            crossAxisCount: 4,
            childAspectRatio: 1.6,
            crossAxisSpacing: AppSpacing.sm,
            mainAxisSpacing: AppSpacing.sm,
          ),
          itemCount: siJin.length,
          itemBuilder: (context, index) {
            final hour = siJin[index];
            final isSelected = hour == selected;

            return GestureDetector(
              onTap: () => onChanged(hour),
              child: Semantics(
                label: '${hour.label} ${hour.timeRange}',
                selected: isSelected,
                child: AnimatedContainer(
                  duration: const Duration(milliseconds: 200),
                  decoration: BoxDecoration(
                    color: isSelected
                        ? AppColors.primary
                        : AppColors.surface,
                    borderRadius: BorderRadius.circular(AppRadius.button),
                    border: Border.all(
                      color: isSelected
                          ? AppColors.primary
                          : AppColors.divider,
                    ),
                  ),
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Text(
                        hour.label,
                        style: AppTypography.bodySemiBold.copyWith(
                          color: isSelected
                              ? AppColors.surface
                              : AppColors.onSurface,
                          fontSize: 14,
                        ),
                      ),
                      Text(
                        hour.timeRange.split('~')[0],
                        style: AppTypography.caption.copyWith(
                          color: isSelected
                              ? AppColors.surface.withValues(alpha: 0.7)
                              : AppColors.secondaryText,
                          fontSize: 11,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            );
          },
        ),

        const SizedBox(height: AppSpacing.sm),

        // "모르겠습니다" full-width button — secondary style
        SizedBox(
          width: double.infinity,
          height: AppDimensions.ctaSecondaryHeight,
          child: OutlinedButton(
            onPressed: () => onChanged(BirthHour.unknown),
            style: OutlinedButton.styleFrom(
              backgroundColor: selected == BirthHour.unknown
                  ? AppColors.primary
                  : Colors.transparent,
              foregroundColor: selected == BirthHour.unknown
                  ? AppColors.surface
                  : AppColors.onSurface,
              side: BorderSide(
                color: selected == BirthHour.unknown
                    ? AppColors.primary
                    : AppColors.divider,
              ),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(AppRadius.button),
              ),
            ),
            child: Text(
              '모르겠습니다',
              style: AppTypography.bodySemiBold.copyWith(
                color: selected == BirthHour.unknown
                    ? AppColors.surface
                    : AppColors.onSurface,
              ),
            ),
          ),
        ),
      ],
    );
  }
}
