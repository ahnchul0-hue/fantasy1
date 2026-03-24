import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../shared/widgets/widgets.dart';
import 'fortune_providers.dart';

/// 오늘의 운세 screen
class FortuneScreen extends ConsumerWidget {
  const FortuneScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final fortuneAsync = ref.watch(dailyFortuneProvider);

    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => context.pop(),
          tooltip: '뒤로',
        ),
        title: const Text('오늘의 운세'),
      ),
      body: fortuneAsync.when(
        loading: () => const FortuneSkeleton(),
        error: (_, __) => ErrorRetryWidget(
          message: '운세를 불러올 수 없습니다',
          onRetry: () => ref.invalidate(dailyFortuneProvider),
        ),
        data: (fortune) {
          if (fortune == null) {
            return EmptyStateWidget(
              message: '생년월일을 입력하면 맞춤 운세를 받을 수 있어요',
              ctaLabel: '사주 카드 만들기',
              onCta: () => context.push(
                '/birth-input',
                extra: {'purpose': 'card'},
              ),
            );
          }

          return SingleChildScrollView(
            padding: const EdgeInsets.all(AppSpacing.md),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                // Date
                Text(
                  fortune.date,
                  style: AppTypography.caption,
                ),
                const SizedBox(height: AppSpacing.sm),

                // Ilju
                Text(
                  fortune.ilju,
                  style: AppTypography.title,
                ),
                const SizedBox(height: AppSpacing.lg),

                // Score
                Row(
                  children: [
                    Text('종합운', style: AppTypography.bodySemiBold),
                    const SizedBox(width: AppSpacing.sm),
                    ...List.generate(5, (i) => Icon(
                      i < fortune.overallScore
                          ? Icons.circle
                          : Icons.circle_outlined,
                      size: 12,
                      color: i < fortune.overallScore
                          ? AppColors.accent
                          : AppColors.disabled,
                    )),
                  ],
                ),

                const SizedBox(height: AppSpacing.lg),
                const Divider(),
                const SizedBox(height: AppSpacing.lg),

                // Fortune text
                Text(
                  fortune.fortuneText,
                  style: AppTypography.body,
                ),

                const SizedBox(height: AppSpacing.lg),
                const Divider(),
                const SizedBox(height: AppSpacing.lg),

                // Lucky items
                _buildLuckyItem('행운 색상', fortune.luckyColor),
                const SizedBox(height: AppSpacing.sm),
                _buildLuckyItem(
                    '행운 숫자', fortune.luckyNumber.toString()),
              ],
            ),
          );
        },
      ),
    );
  }

  Widget _buildLuckyItem(String label, String value) {
    return Row(
      children: [
        SizedBox(
          width: 80,
          child: Text(label, style: AppTypography.caption),
        ),
        Text(value, style: AppTypography.bodySemiBold),
      ],
    );
  }
}
