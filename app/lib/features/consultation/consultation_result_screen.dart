import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:cached_network_image/cached_network_image.dart';

import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../shared/widgets/widgets.dart';
import 'consultation_providers.dart';

/// Paid consultation result screen with Progressive Reveal
/// Phase 1: LLM text streaming, Phase 2: text + images loading
/// Vertical scroll (no carousel) — sections: 요약, 성격, 연애운, 재물운, 커리어, 조언
class ConsultationResultScreen extends ConsumerWidget {
  final String consultationId;

  const ConsultationResultScreen({
    super.key,
    required this.consultationId,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final consultationState = ref.watch(consultationProvider);

    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => context.pop(),
          tooltip: '뒤로',
        ),
        title: const Text('결과'),
        actions: [
          IconButton(
            icon: const Icon(Icons.share_outlined),
            onPressed: () {
              // TODO: Share functionality
            },
            tooltip: '공유',
          ),
        ],
      ),
      body: consultationState.when(
        loading: () => _buildProgressiveLoading(),
        error: (e, _) => ErrorRetryWidget(
          message: '해석 준비 중. 알림을 보내드리겠습니다',
          onRetry: () {
            // TODO: Retry or navigate home
            context.go('/home');
          },
        ),
        data: (consultation) {
          if (consultation == null) {
            return const EmptyStateWidget(message: '결과를 불러올 수 없습니다');
          }
          return _buildResult(context, consultation);
        },
      ),
    );
  }

  /// Progressive Reveal Phase 1: Loading state
  Widget _buildProgressiveLoading() {
    return Column(
      children: [
        // Progress header
        Padding(
          padding: const EdgeInsets.all(AppSpacing.md),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                '사주 해석을 작성하고 있습니다',
                style: AppTypography.bodySemiBold,
              ),
              const SizedBox(height: AppSpacing.sm),
              const LinearProgressIndicator(
                color: AppColors.accent,
                backgroundColor: AppColors.divider,
              ),
            ],
          ),
        ),
        const Divider(),
        // Skeleton sections
        Expanded(
          child: ListView.separated(
            padding: const EdgeInsets.all(AppSpacing.md),
            itemCount: 5,
            separatorBuilder: (_, __) => const Divider(height: AppSpacing.lg),
            itemBuilder: (_, index) => Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                LoadingSkeleton(width: 80, height: 22),
                const SizedBox(height: AppSpacing.sm),
                LoadingSkeleton(height: 120),
                const SizedBox(height: AppSpacing.sm),
                LoadingSkeleton(height: 60),
              ],
            ),
          ),
        ),
      ],
    );
  }

  /// Full result with sections
  Widget _buildResult(BuildContext context, dynamic consultation) {
    // Sections for vertical scroll
    const sections = ['성격', '연애운', '재물운', '커리어', '조언'];

    return Column(
      children: [
        Expanded(
          child: SingleChildScrollView(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                // Summary header with image
                Container(
                  width: double.infinity,
                  color: AppColors.primary.withValues(alpha: 0.05),
                  padding: const EdgeInsets.all(AppSpacing.md),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        consultation.analysisSummary ??
                            '사주 분석이 완료되었습니다',
                        style: AppTypography.title,
                      ),
                    ],
                  ),
                ),

                // Sections — vertical, each with image + text
                ...sections.asMap().entries.map((entry) {
                  final index = entry.key;
                  final section = entry.value;
                  final hasImage = consultation.resultImages.length > index;

                  return Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Divider(),
                      Padding(
                        padding: const EdgeInsets.all(AppSpacing.md),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(section, style: AppTypography.title),
                            const SizedBox(height: AppSpacing.md),
                            if (hasImage)
                              ClipRRect(
                                borderRadius: BorderRadius.circular(
                                    AppRadius.card),
                                child: CachedNetworkImage(
                                  imageUrl:
                                      consultation.resultImages[index],
                                  width: double.infinity,
                                  fit: BoxFit.cover,
                                  placeholder: (_, __) =>
                                      const LoadingSkeleton(height: 200),
                                  errorWidget: (_, __, ___) =>
                                      const LoadingSkeleton(height: 200),
                                ),
                              )
                            else
                              // Shimmer placeholder if image not yet generated
                              const LoadingSkeleton(height: 200),
                            const SizedBox(height: AppSpacing.md),
                            // Placeholder for SSE streaming text
                            Text(
                              '해석 텍스트가 여기에 표시됩니다...',
                              style: AppTypography.body.copyWith(
                                color: AppColors.secondaryText,
                              ),
                            ),
                          ],
                        ),
                      ),
                    ],
                  );
                }),

                const SizedBox(height: AppSpacing.xl),
              ],
            ),
          ),
        ),

        // Sticky bottom CTA: 월하선생에게 물어보기
        Container(
          padding: const EdgeInsets.all(AppSpacing.md),
          decoration: const BoxDecoration(
            color: AppColors.surface,
            border: Border(
              top: BorderSide(color: AppColors.divider),
            ),
          ),
          child: SafeArea(
            top: false,
            child: PrimaryButton(
              label: '월하선생에게 물어보기',
              onPressed: () => context.push(
                '/consultation/$consultationId/chat',
              ),
            ),
          ),
        ),
      ],
    );
  }
}
