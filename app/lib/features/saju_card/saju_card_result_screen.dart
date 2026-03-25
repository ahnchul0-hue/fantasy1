import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:share_plus/share_plus.dart';

import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../shared/models/birth_input.dart';
import '../../shared/models/saju_card.dart';
import '../../shared/widgets/widgets.dart';
import 'saju_card_providers.dart';

/// Free saju card result screen
/// Card image + ilju name + keywords + share + paid CTA + social proof
class SajuCardResultScreen extends ConsumerStatefulWidget {
  final String? cardId;
  final BirthInput? birthInput;

  const SajuCardResultScreen({super.key, this.cardId, this.birthInput});

  @override
  ConsumerState<SajuCardResultScreen> createState() =>
      _SajuCardResultScreenState();
}

class _SajuCardResultScreenState extends ConsumerState<SajuCardResultScreen> {
  @override
  void initState() {
    super.initState();
    // birthInput이 있으면 카드 생성 시작
    if (widget.birthInput != null) {
      Future.microtask(() {
        ref
            .read(sajuCardCreationProvider.notifier)
            .createCard(widget.birthInput!);
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    // birthInput 기반 생성 플로우
    if (widget.birthInput != null) {
      return _buildWithCreationProvider();
    }

    // cardId 기반 조회 플로우
    if (widget.cardId != null) {
      return _buildWithCardId();
    }

    // 둘 다 없으면 에러
    return Scaffold(
      appBar: AppBar(),
      body: const EmptyStateWidget(message: '카드 정보를 불러올 수 없습니다'),
    );
  }

  Widget _buildWithCreationProvider() {
    final cardState = ref.watch(sajuCardCreationProvider);

    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => context.pop(),
          tooltip: '뒤로',
        ),
      ),
      body: cardState.when(
        loading: () => const Center(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              CircularProgressIndicator(color: AppColors.accent),
              SizedBox(height: AppSpacing.md),
              Text(
                '사주 카드를 만들고 있습니다',
                style: AppTypography.body,
              ),
            ],
          ),
        ),
        error: (e, _) => ErrorRetryWidget(
          message: '다시 시도해주세요',
          onRetry: () {
            if (widget.birthInput != null) {
              ref
                  .read(sajuCardCreationProvider.notifier)
                  .createCard(widget.birthInput!);
            }
          },
        ),
        data: (card) {
          if (card == null) {
            return const EmptyStateWidget(message: '카드 정보를 불러올 수 없습니다');
          }
          return _buildCardContent(card);
        },
      ),
    );
  }

  Widget _buildWithCardId() {
    final cardAsync = ref.watch(sajuCardByIdProvider(widget.cardId!));

    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => context.pop(),
          tooltip: '뒤로',
        ),
      ),
      body: cardAsync.when(
        loading: () => const Center(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              CircularProgressIndicator(color: AppColors.accent),
              SizedBox(height: AppSpacing.md),
              Text(
                '사주 카드를 불러오는 중입니다',
                style: AppTypography.body,
              ),
            ],
          ),
        ),
        error: (e, _) => ErrorRetryWidget(
          message: '다시 시도해주세요',
          onRetry: () => ref.invalidate(sajuCardByIdProvider(widget.cardId!)),
        ),
        data: (card) {
          if (card == null) {
            return const EmptyStateWidget(message: '카드 정보를 불러올 수 없습니다');
          }
          return _buildCardContent(card);
        },
      ),
    );
  }

  Widget _buildCardContent(SajuCard card) {
    return SingleChildScrollView(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // Card image — full width
          Padding(
            padding: const EdgeInsets.symmetric(
              horizontal: AppSpacing.md,
            ),
            child: SajuCardWidget(card: card, showImage: true),
          ),

          const SizedBox(height: AppSpacing.md),

          // Action buttons: 이미지 저장 + 공유
          Padding(
            padding: const EdgeInsets.symmetric(
              horizontal: AppSpacing.md,
            ),
            child: Row(
              children: [
                Expanded(
                  child: SecondaryButton(
                    label: '이미지 저장',
                    onPressed: () {
                      // TODO: save to gallery
                    },
                  ),
                ),
                const SizedBox(width: AppSpacing.sm),
                Expanded(
                  child: SecondaryButton(
                    label: '공유',
                    onPressed: () {
                      Share.share(
                        card.shareUrl ?? '내 사주 카드를 확인해보세요',
                        subject: '사주 카드',
                      );
                    },
                  ),
                ),
              ],
            ),
          ),

          const SizedBox(height: AppSpacing.lg),
          const Divider(),
          const SizedBox(height: AppSpacing.lg),

          // Paid CTA
          Padding(
            padding: const EdgeInsets.symmetric(
              horizontal: AppSpacing.md,
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  '더 자세한 사주 풀이 보기',
                  style: AppTypography.title,
                ),
                const SizedBox(height: AppSpacing.xs),
                Text(
                  '15,000원 · 72시간 AI 채팅 포함',
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

          const SizedBox(height: AppSpacing.lg),
          const Divider(),
          const SizedBox(height: AppSpacing.md),

          // Social proof
          Padding(
            padding: const EdgeInsets.symmetric(
              horizontal: AppSpacing.md,
            ),
            child: Text(
              '누적 상담 0건',
              style: AppTypography.caption,
              textAlign: TextAlign.center,
            ),
          ),

          const SizedBox(height: AppSpacing.xl),
        ],
      ),
    );
  }
}
