import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../core/providers/auth_providers.dart';
import '../../shared/widgets/widgets.dart';
import '../fortune/fortune_providers.dart';
import '../profile/profile_providers.dart';

/// Home screen — 3 layouts based on user state:
/// A: First visit (no login/profile) — single CTA focus
/// B: Logged in, no profile — card creation priority
/// C: Existing user (login + profile) — fortune + profile + CTAs
class HomeScreen extends ConsumerWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final authState = ref.watch(authStateProvider);

    return Scaffold(
      body: SafeArea(
        child: authState.when(
          loading: () => const Center(child: CircularProgressIndicator()),
          error: (_, __) => _buildStateA(context),
          data: (user) {
            if (user == null) return _buildStateA(context);
            if (!user.hasProfile) return _buildStateB(context);
            return _buildStateC(context, ref);
          },
        ),
      ),
    );
  }

  /// State A: First visit — brand mark + single CTA
  Widget _buildStateA(BuildContext context) {
    return SingleChildScrollView(
      child: Padding(
        padding: const EdgeInsets.all(AppSpacing.md),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const SizedBox(height: AppSpacing.xl),

            // Brand mark
            _buildBrandMark(),

            const SizedBox(height: AppSpacing.xl),

            // Single CTA: 사주 카드 만들기
            Text(
              '나의 사주 카드 만들기',
              style: AppTypography.title,
            ),
            const SizedBox(height: AppSpacing.sm),
            Text(
              '생년월일만 입력하세요',
              style: AppTypography.body.copyWith(
                color: AppColors.secondaryText,
              ),
            ),

            const SizedBox(height: AppSpacing.lg),

            PrimaryButton(
              label: '무료 카드 만들기',
              onPressed: () => context.push(
                '/birth-input',
                extra: {'purpose': 'card'},
              ),
            ),

            const SizedBox(height: AppSpacing.xl),
            const Divider(),
            const SizedBox(height: AppSpacing.md),

            // Social proof — 실제 데이터 연동 전 placeholder
            Text(
              'AI 기반 사주 분석 서비스',
              style: AppTypography.caption.copyWith(
                color: AppColors.secondaryText,
              ),
            ),
          ],
        ),
      ),
    );
  }

  /// State B: Logged in, no profile — card creation + motivation
  Widget _buildStateB(BuildContext context) {
    return SingleChildScrollView(
      child: Padding(
        padding: const EdgeInsets.all(AppSpacing.md),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const SizedBox(height: AppSpacing.lg),
            _buildBrandMark(),
            const SizedBox(height: AppSpacing.lg),
            const Divider(),
            const SizedBox(height: AppSpacing.lg),

            Text(
              '나의 사주 카드 만들기',
              style: AppTypography.title,
            ),
            const SizedBox(height: AppSpacing.sm),

            PrimaryButton(
              label: '무료 카드 만들기',
              onPressed: () => context.push(
                '/birth-input',
                extra: {'purpose': 'card'},
              ),
            ),

            const SizedBox(height: AppSpacing.lg),
            const Divider(),
            const SizedBox(height: AppSpacing.md),

            Text(
              '생년월일을 입력하면 매일 맞춤 운세를 받을 수 있어요',
              style: AppTypography.body.copyWith(
                color: AppColors.secondaryText,
              ),
            ),
          ],
        ),
      ),
    );
  }

  /// State C: Existing user — fortune + profile summary + paid CTA + 궁합
  Widget _buildStateC(BuildContext context, WidgetRef ref) {
    final fortuneAsync = ref.watch(dailyFortuneProvider);
    final profileAsync = ref.watch(profileProvider);

    return SingleChildScrollView(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // App bar with brand + notification
          Padding(
            padding: const EdgeInsets.symmetric(
              horizontal: AppSpacing.md,
              vertical: AppSpacing.sm,
            ),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                _buildBrandMark(),
                IconButton(
                  icon: const Icon(Icons.notifications_outlined),
                  onPressed: () {
                    // TODO: Notification screen
                  },
                  tooltip: '알림',
                ),
              ],
            ),
          ),

          const Divider(),

          // 1st: 오늘의 운세 (리텐션 훅)
          Padding(
            padding: const EdgeInsets.all(AppSpacing.md),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text('오늘의 운세', style: AppTypography.title),
                const SizedBox(height: AppSpacing.sm),
                fortuneAsync.when(
                  loading: () => const FortuneSkeleton(),
                  error: (e, _) => ErrorRetryWidget(
                    message: '운세를 불러올 수 없습니다',
                    onRetry: () => ref.invalidate(dailyFortuneProvider),
                  ),
                  data: (fortune) {
                    if (fortune == null) {
                      return Text(
                        '운세를 불러오는 중...',
                        style: AppTypography.body.copyWith(
                          color: AppColors.secondaryText,
                        ),
                      );
                    }
                    return Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          fortune.fortuneText,
                          style: AppTypography.body,
                          maxLines: 3,
                          overflow: TextOverflow.ellipsis,
                        ),
                        const SizedBox(height: AppSpacing.sm),
                        Text(
                          '행운 아이템: ${fortune.luckyColor}',
                          style: AppTypography.caption,
                        ),
                      ],
                    );
                  },
                ),
              ],
            ),
          ),

          const Divider(),

          // 2nd: 오행 미니 차트 + 상세 보기
          Padding(
            padding: const EdgeInsets.all(AppSpacing.md),
            child: profileAsync.when(
              loading: () => const LoadingSkeleton(height: 80),
              error: (_, __) => const SizedBox.shrink(),
              data: (profile) {
                if (profile == null) return const SizedBox.shrink();
                return Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Text('오행 차트', style: AppTypography.bodySemiBold),
                        TextButton(
                          onPressed: () => context.go('/profile'),
                          child: const Text('상세 보기'),
                        ),
                      ],
                    ),
                    OhengChart(
                      balance: profile.ohengBalance,
                      mini: true,
                    ),
                  ],
                );
              },
            ),
          ),

          const Divider(),

          // 3rd: 유료 CTA
          Padding(
            padding: const EdgeInsets.all(AppSpacing.md),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text('AI 사주 상담', style: AppTypography.title),
                const SizedBox(height: AppSpacing.xs),
                Text(
                  '15,000원 · 72시간 채팅 포함',
                  style: AppTypography.body.copyWith(
                    color: AppColors.secondaryText,
                  ),
                ),
                const SizedBox(height: AppSpacing.md),
                PrimaryButton(
                  label: '상담 시작하기',
                  onPressed: () => context.push(
                    '/birth-input',
                    extra: {'purpose': 'consultation'},
                  ),
                ),
              ],
            ),
          ),

          const Divider(),

          // 4th: 궁합
          Padding(
            padding: const EdgeInsets.all(AppSpacing.md),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text('궁합 알아보기', style: AppTypography.bodySemiBold),
                const SizedBox(height: AppSpacing.sm),
                SecondaryButton(
                  label: '두 사람의 궁합 확인',
                  onPressed: () => context.push('/compatibility'),
                ),
              ],
            ),
          ),

          const SizedBox(height: AppSpacing.lg),
        ],
      ),
    );
  }

  Widget _buildBrandMark() {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Container(
          width: 32,
          height: 32,
          decoration: BoxDecoration(
            color: AppColors.primary,
            borderRadius: BorderRadius.circular(8),
          ),
          child: const Center(
            child: Text(
              '命',
              style: TextStyle(
                fontFamily: AppTypography.fontHanja,
                fontSize: 18,
                color: AppColors.accent,
              ),
            ),
          ),
        ),
        const SizedBox(width: AppSpacing.sm),
        Text(
          '사주',
          style: AppTypography.title.copyWith(
            color: AppColors.primary,
          ),
        ),
      ],
    );
  }
}
