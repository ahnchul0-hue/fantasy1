import 'package:flutter/material.dart';
import 'package:cached_network_image/cached_network_image.dart';
import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../shared/models/saju_card.dart';
import 'loading_skeleton.dart';

/// 사주 카드 widget — r12, no elevation, 1px #E0D8CF
/// ONLY used for 공유용 사주 카드 and 히스토리 아이템
class SajuCardWidget extends StatelessWidget {
  final SajuCard card;
  final VoidCallback? onTap;
  final bool showImage;

  const SajuCardWidget({
    super.key,
    required this.card,
    this.onTap,
    this.showImage = true,
  });

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap,
      child: Container(
        decoration: BoxDecoration(
          color: AppColors.surface,
          borderRadius: BorderRadius.circular(AppRadius.card),
          border: Border.all(color: AppColors.divider),
        ),
        clipBehavior: Clip.antiAlias,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            if (showImage && card.imageUrl != null)
              AspectRatio(
                aspectRatio: 3 / 4,
                child: CachedNetworkImage(
                  imageUrl: card.imageUrl!,
                  fit: BoxFit.cover,
                  placeholder: (_, __) =>
                      const LoadingSkeleton(height: double.infinity),
                  errorWidget: (_, __, ___) => Container(
                    color: AppColors.primary.withValues(alpha: 0.1),
                    child: Center(
                      child: Text(
                        card.iljuHanja,
                        style: AppTypography.hanjaDisplay.copyWith(
                          color: AppColors.primary,
                          fontSize: 48,
                        ),
                      ),
                    ),
                  ),
                ),
              ),
            Padding(
              padding: const EdgeInsets.all(AppSpacing.md),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  // 일주 한글(한자) e.g. "갑목(甲木)"
                  Semantics(
                    label: '${card.iljuName}, 한자 ${card.iljuHanja}',
                    child: Text.rich(
                      TextSpan(
                        children: [
                          TextSpan(
                            text: card.iljuName,
                            style: AppTypography.title,
                          ),
                          TextSpan(
                            text: '(${card.iljuHanja})',
                            style: AppTypography.title.copyWith(
                              fontFamily: AppTypography.fontHanja,
                            ),
                          ),
                          TextSpan(
                            text: ' 일주',
                            style: AppTypography.title,
                          ),
                        ],
                      ),
                    ),
                  ),
                  const SizedBox(height: AppSpacing.sm),
                  // Keywords
                  if (card.keywords.isNotEmpty)
                    Text(
                      '올해 키워드: ${card.keywords.join(', ')}',
                      style: AppTypography.body.copyWith(
                        color: AppColors.secondaryText,
                      ),
                    ),
                  const SizedBox(height: AppSpacing.xs),
                  Text(
                    '행운 요소: ${card.luckyElement}',
                    style: AppTypography.caption,
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
