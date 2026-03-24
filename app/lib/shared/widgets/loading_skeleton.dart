import 'package:flutter/material.dart';
import 'package:shimmer/shimmer.dart';
import '../../core/theme/app_colors.dart';
import '../../core/theme/app_spacing.dart';
import '../../core/theme/app_motion.dart';

/// Skeleton loading — r8, bg #E0D8CF 30% shimmer
class LoadingSkeleton extends StatelessWidget {
  final double width;
  final double height;
  final double borderRadius;

  const LoadingSkeleton({
    super.key,
    this.width = double.infinity,
    required this.height,
    this.borderRadius = AppRadius.skeleton,
  });

  @override
  Widget build(BuildContext context) {
    if (AppMotion.reducedMotion) {
      return Container(
        width: width,
        height: height,
        decoration: BoxDecoration(
          color: AppColors.divider.withValues(alpha: 0.3),
          borderRadius: BorderRadius.circular(borderRadius),
        ),
      );
    }

    return Shimmer.fromColors(
      baseColor: AppColors.divider.withValues(alpha: 0.3),
      highlightColor: AppColors.surface,
      child: Container(
        width: width,
        height: height,
        decoration: BoxDecoration(
          color: AppColors.divider,
          borderRadius: BorderRadius.circular(borderRadius),
        ),
      ),
    );
  }
}

/// Full card skeleton for history list
class CardSkeleton extends StatelessWidget {
  const CardSkeleton({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(
        horizontal: AppSpacing.md,
        vertical: AppSpacing.sm,
      ),
      child: Row(
        children: [
          const LoadingSkeleton(width: 48, height: 48, borderRadius: 8),
          const SizedBox(width: AppSpacing.md),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                LoadingSkeleton(
                  width: 120,
                  height: 16,
                  borderRadius: AppRadius.skeleton,
                ),
                const SizedBox(height: AppSpacing.sm),
                LoadingSkeleton(
                  width: 80,
                  height: 13,
                  borderRadius: AppRadius.skeleton,
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

/// Fortune skeleton
class FortuneSkeleton extends StatelessWidget {
  const FortuneSkeleton({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(AppSpacing.md),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const LoadingSkeleton(width: 100, height: 22),
          const SizedBox(height: AppSpacing.md),
          const LoadingSkeleton(height: 60),
          const SizedBox(height: AppSpacing.sm),
          LoadingSkeleton(width: 150, height: 13),
        ],
      ),
    );
  }
}
