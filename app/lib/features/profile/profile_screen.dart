import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../shared/widgets/widgets.dart';
import '../../shared/models/saju_profile.dart';
import 'profile_providers.dart';

/// 평생 사주 프로필 with 오행 차트
class ProfileScreen extends ConsumerWidget {
  const ProfileScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final profileAsync = ref.watch(profileProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('내 프로필'),
      ),
      body: profileAsync.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error: (_, __) => ErrorRetryWidget(
          message: '프로필을 불러올 수 없습니다',
          onRetry: () => ref.invalidate(profileProvider),
        ),
        data: (profile) {
          if (profile == null) {
            return EmptyStateWidget(
              message: '사주 카드를 먼저 만들어주세요',
              ctaLabel: '사주 카드 만들기',
              onCta: () => context.push(
                '/birth-input',
                extra: {'purpose': 'card'},
              ),
            );
          }
          return _buildProfile(context, profile);
        },
      ),
    );
  }

  Widget _buildProfile(BuildContext context, SajuProfile profile) {
    final pillars = profile.fourPillars;
    final birth = profile.birthInput;

    return SingleChildScrollView(
      padding: const EdgeInsets.all(AppSpacing.md),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Birth info summary
          Text(
            '${birth.year}년 ${birth.month}월 ${birth.day}일',
            style: AppTypography.title,
          ),
          const SizedBox(height: AppSpacing.xs),
          Text(
            '${birth.calendarType == CalendarType.solar ? "양력" : "음력"} · '
            '${birth.birthHour.label} · '
            '${birth.gender == Gender.male ? "남성" : "여성"}',
            style: AppTypography.body.copyWith(
              color: AppColors.secondaryText,
            ),
          ),

          const SizedBox(height: AppSpacing.lg),
          const Divider(),
          const SizedBox(height: AppSpacing.lg),

          // 사주팔자 — Four Pillars
          Text('사주팔자', style: AppTypography.title),
          const SizedBox(height: AppSpacing.md),
          _buildFourPillarsTable(pillars),

          // Birth hour unknown banner
          if (birth.birthHour == BirthHour.unknown) ...[
            const SizedBox(height: AppSpacing.md),
            const BannerWidget(
              text: '출생시간을 추가하면 더 정확한 분석 가능',
              type: BannerType.info,
              icon: Icons.info_outline,
            ),
          ],

          const SizedBox(height: AppSpacing.lg),
          const Divider(),
          const SizedBox(height: AppSpacing.lg),

          // 오행 차트
          Text('오행 밸런스', style: AppTypography.title),
          const SizedBox(height: AppSpacing.md),
          OhengChart(balance: profile.ohengBalance),

          const SizedBox(height: AppSpacing.xl),
        ],
      ),
    );
  }

  Widget _buildFourPillarsTable(FourPillars pillars) {
    // Treat hour as unknown if null OR if hanja strings are empty
    final isHourUnknown = pillars.hour == null ||
        (pillars.hour!.heavenlyStemHanja.isEmpty &&
            pillars.hour!.earthlyBranchHanja.isEmpty);

    final pillarList = [
      ('시주', isHourUnknown ? null : pillars.hour),
      ('일주', pillars.day),
      ('월주', pillars.month),
      ('연주', pillars.year),
    ];

    return Row(
      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
      children: pillarList.map((entry) {
        final label = entry.$1;
        final pillar = entry.$2;

        return Column(
          children: [
            Text(label, style: AppTypography.caption),
            const SizedBox(height: AppSpacing.sm),
            if (pillar != null) ...[
              // Heavenly stem (천간)
              Container(
                width: 56,
                height: 56,
                decoration: BoxDecoration(
                  color: AppColors.primary.withValues(alpha: 0.05),
                  borderRadius: BorderRadius.circular(AppRadius.button),
                  border: Border.all(color: AppColors.divider),
                ),
                child: Center(
                  child: Semantics(
                    label: '${pillar.heavenlyStem}, 한자 ${pillar.heavenlyStemHanja}',
                    child: Text(
                      pillar.heavenlyStemHanja,
                      style: AppTypography.hanjaDisplay.copyWith(
                        fontSize: 24,
                      ),
                    ),
                  ),
                ),
              ),
              const SizedBox(height: AppSpacing.xs),
              // Earthly branch (지지)
              Container(
                width: 56,
                height: 56,
                decoration: BoxDecoration(
                  color: AppColors.primary.withValues(alpha: 0.05),
                  borderRadius: BorderRadius.circular(AppRadius.button),
                  border: Border.all(color: AppColors.divider),
                ),
                child: Center(
                  child: Semantics(
                    label: '${pillar.earthlyBranch}, 한자 ${pillar.earthlyBranchHanja}',
                    child: Text(
                      pillar.earthlyBranchHanja,
                      style: AppTypography.hanjaDisplay.copyWith(
                        fontSize: 24,
                      ),
                    ),
                  ),
                ),
              ),
              const SizedBox(height: AppSpacing.xs),
              Text(
                pillar.heavenlyStem,
                style: AppTypography.caption,
              ),
              Text(
                pillar.earthlyBranch,
                style: AppTypography.caption,
              ),
            ] else ...[
              // Unknown hour
              Container(
                width: 56,
                height: 56,
                decoration: BoxDecoration(
                  color: AppColors.divider.withValues(alpha: 0.3),
                  borderRadius: BorderRadius.circular(AppRadius.button),
                ),
                child: const Center(
                  child: Text('?', style: AppTypography.title),
                ),
              ),
              const SizedBox(height: AppSpacing.xs),
              Container(
                width: 56,
                height: 56,
                decoration: BoxDecoration(
                  color: AppColors.divider.withValues(alpha: 0.3),
                  borderRadius: BorderRadius.circular(AppRadius.button),
                ),
                child: const Center(
                  child: Text('?', style: AppTypography.title),
                ),
              ),
              const SizedBox(height: AppSpacing.xs),
              Text('모름', style: AppTypography.caption),
            ],
          ],
        );
      }).toList(),
    );
  }
}
