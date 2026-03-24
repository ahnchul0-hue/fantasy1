import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../core/api/api_client.dart';
import '../../shared/models/models.dart';
import '../../shared/widgets/widgets.dart';

/// 궁합 state
final compatibilityProvider = StateNotifierProvider.autoDispose<
    CompatibilityNotifier, AsyncValue<CompatibilityPreview?>>((ref) {
  return CompatibilityNotifier(apiClient: ref.watch(apiClientProvider));
});

class CompatibilityNotifier
    extends StateNotifier<AsyncValue<CompatibilityPreview?>> {
  final ApiClient _apiClient;

  CompatibilityNotifier({required ApiClient apiClient})
      : _apiClient = apiClient,
        super(const AsyncValue.data(null));

  Future<void> check(BirthInput p1, BirthInput p2) async {
    state = const AsyncValue.loading();
    try {
      final result = await _apiClient.getCompatibility({
        'person1': p1.toJson(),
        'person2': p2.toJson(),
      });
      state = AsyncValue.data(result);
    } catch (e, st) {
      state = AsyncValue.error(e, st);
    }
  }
}

/// 궁합 미리보기 (무료) + upsell screen
class CompatibilityScreen extends ConsumerStatefulWidget {
  const CompatibilityScreen({super.key});

  @override
  ConsumerState<CompatibilityScreen> createState() =>
      _CompatibilityScreenState();
}

class _CompatibilityScreenState extends ConsumerState<CompatibilityScreen> {
  BirthInput? _person1;
  BirthInput? _person2;

  @override
  Widget build(BuildContext context) {
    final resultState = ref.watch(compatibilityProvider);

    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => context.pop(),
          tooltip: '뒤로',
        ),
        title: const Text('궁합 확인'),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(AppSpacing.md),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Person 1
            Text('첫 번째 사람', style: AppTypography.bodySemiBold),
            const SizedBox(height: AppSpacing.sm),
            _buildPersonInput(
              person: _person1,
              label: '생년월일 입력',
              onTap: () async {
                final result = await context.push<BirthInput>(
                  '/birth-input',
                  extra: {'purpose': 'compatibility'},
                );
                if (result != null) {
                  setState(() => _person1 = result);
                }
              },
            ),

            const SizedBox(height: AppSpacing.lg),

            // Person 2
            Text('두 번째 사람', style: AppTypography.bodySemiBold),
            const SizedBox(height: AppSpacing.sm),
            _buildPersonInput(
              person: _person2,
              label: '생년월일 입력',
              onTap: () async {
                final result = await context.push<BirthInput>(
                  '/birth-input',
                  extra: {'purpose': 'compatibility'},
                );
                if (result != null) {
                  setState(() => _person2 = result);
                }
              },
            ),

            const SizedBox(height: AppSpacing.lg),

            // Check button
            PrimaryButton(
              label: '궁합 확인',
              onPressed:
                  _person1 != null && _person2 != null ? _checkCompatibility : null,
              isLoading: resultState is AsyncLoading,
            ),

            const SizedBox(height: AppSpacing.lg),

            // Result
            resultState.when(
              loading: () => const Center(
                child: Column(
                  children: [
                    CircularProgressIndicator(color: AppColors.accent),
                    SizedBox(height: AppSpacing.md),
                    Text('궁합을 계산하는 중...', style: AppTypography.body),
                  ],
                ),
              ),
              error: (_, __) => ErrorRetryWidget(
                message: '다시 시도해주세요',
                onRetry: _checkCompatibility,
              ),
              data: (result) {
                if (result == null) return const SizedBox.shrink();
                return _buildResult(result);
              },
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildPersonInput({
    required BirthInput? person,
    required String label,
    required VoidCallback onTap,
  }) {
    if (person != null) {
      return Container(
        padding: const EdgeInsets.all(AppSpacing.md),
        decoration: BoxDecoration(
          color: AppColors.bannerInfoBg,
          borderRadius: BorderRadius.circular(AppRadius.button),
          border: Border.all(color: AppColors.divider),
        ),
        child: Row(
          children: [
            Expanded(
              child: Text(
                '${person.year}년 ${person.month}월 ${person.day}일 · '
                '${person.gender == Gender.male ? "남" : "여"}',
                style: AppTypography.body,
              ),
            ),
            TextButton(
              onPressed: onTap,
              child: const Text('수정'),
            ),
          ],
        ),
      );
    }

    return SecondaryButton(label: label, onPressed: onTap);
  }

  Widget _buildResult(CompatibilityPreview result) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Divider(),
        const SizedBox(height: AppSpacing.lg),

        // Score
        Center(
          child: Column(
            children: [
              Text(
                '${result.score}',
                style: AppTypography.display.copyWith(
                  fontSize: 56,
                  color: AppColors.accent,
                ),
              ),
              Text('/ 100', style: AppTypography.caption),
            ],
          ),
        ),
        const SizedBox(height: AppSpacing.md),

        // Summary
        Text(result.summary, style: AppTypography.body),
        const SizedBox(height: AppSpacing.sm),

        // Elements
        Text(
          '${result.person1Element} + ${result.person2Element}',
          style: AppTypography.caption,
        ),

        const SizedBox(height: AppSpacing.lg),
        const Divider(),
        const SizedBox(height: AppSpacing.lg),

        // Upsell CTA
        Text(
          '상세 궁합 분석 + AI 상담',
          style: AppTypography.title,
        ),
        const SizedBox(height: AppSpacing.xs),
        Text(
          '12,000원 · 72시간 AI 채팅 포함',
          style: AppTypography.body
              .copyWith(color: AppColors.secondaryText),
        ),
        const SizedBox(height: AppSpacing.md),
        PrimaryButton(
          label: '상세 궁합 분석 보기',
          onPressed: () {
            // TODO: IAP flow for compatibility consultation
          },
        ),
      ],
    );
  }

  void _checkCompatibility() {
    if (_person1 != null && _person2 != null) {
      ref.read(compatibilityProvider.notifier).check(_person1!, _person2!);
    }
  }
}
