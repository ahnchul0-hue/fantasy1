import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../core/api/api_client.dart';
import '../../shared/models/consultation.dart';
import '../../shared/widgets/widgets.dart';

/// Consultation history — fetched from GET /consultations
/// 에러 발생 시 AsyncError로 전파하여 UI에서 재시도 버튼 표시
final consultationHistoryProvider =
    FutureProvider.autoDispose<List<Consultation>>((ref) async {
  final apiClient = ref.watch(apiClientProvider);
  return await apiClient.getConsultations();
});

/// 내 분석 탭 — history list with status badges
/// Active consultations pinned to top, then by date descending
class HistoryScreen extends ConsumerWidget {
  const HistoryScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final historyAsync = ref.watch(consultationHistoryProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('내 분석'),
      ),
      body: historyAsync.when(
        loading: () => ListView.builder(
          itemCount: 3,
          itemBuilder: (_, __) => const CardSkeleton(),
        ),
        error: (_, __) => ErrorRetryWidget(
          message: '불러오기 실패',
          onRetry: () => ref.invalidate(consultationHistoryProvider),
        ),
        data: (consultations) {
          if (consultations.isEmpty) {
            return EmptyStateWidget(
              message: '아직 상담 기록이 없습니다\n첫 사주 상담을 시작해보세요',
              ctaLabel: '상담 시작하기',
              onCta: () => context.push(
                '/birth-input',
                extra: {'purpose': 'consultation'},
              ),
            );
          }

          // Sort: active first, then by date
          final active = consultations.where((c) => c.isActive).toList();
          final past = consultations.where((c) => !c.isActive).toList();

          return ListView(
            children: [
              if (active.isNotEmpty) ...[
                _buildSectionHeader('진행 중'),
                ...active.map((c) => _buildHistoryItem(context, c)),
              ],
              if (past.isNotEmpty) ...[
                _buildSectionHeader('지난 상담'),
                ...past.map((c) => _buildHistoryItem(context, c)),
              ],
            ],
          );
        },
      ),
    );
  }

  Widget _buildSectionHeader(String title) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(
        AppSpacing.md,
        AppSpacing.md,
        AppSpacing.md,
        AppSpacing.sm,
      ),
      child: Text(title, style: AppTypography.bodySemiBold),
    );
  }

  /// History item — NOT a card. spacing + divider layout
  Widget _buildHistoryItem(BuildContext context, Consultation c) {
    return InkWell(
      onTap: () {
        if (c.isActive) {
          context.push('/consultation/${c.id}/chat');
        } else {
          context.push('/consultation/${c.id}/result');
        }
      },
      child: Padding(
        padding: const EdgeInsets.symmetric(
          horizontal: AppSpacing.md,
          vertical: AppSpacing.sm,
        ),
        child: Column(
          children: [
            Row(
              children: [
                // Thumbnail (48x48, r8)
                Container(
                  width: 48,
                  height: 48,
                  decoration: BoxDecoration(
                    color: AppColors.primary.withValues(alpha: 0.05),
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: c.resultImages.isNotEmpty
                      ? ClipRRect(
                          borderRadius: BorderRadius.circular(8),
                          child: Image.network(
                            c.resultImages.first,
                            fit: BoxFit.cover,
                            errorBuilder: (_, __, ___) => const Center(
                              child: Text(
                                '命',
                                style: TextStyle(
                                  fontFamily: AppTypography.fontHanja,
                                  fontSize: 20,
                                  color: AppColors.primary,
                                ),
                              ),
                            ),
                          ),
                        )
                      : const Center(
                          child: Text(
                            '命',
                            style: TextStyle(
                              fontFamily: AppTypography.fontHanja,
                              fontSize: 20,
                              color: AppColors.primary,
                            ),
                          ),
                        ),
                ),
                const SizedBox(width: AppSpacing.md),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        c.analysisSummary ?? '사주 상담',
                        style: AppTypography.bodySemiBold,
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                      ),
                      const SizedBox(height: 2),
                      Text(
                        c.expiresAt != null
                            ? _formatDate(c.expiresAt!)
                            : '',
                        style: AppTypography.caption,
                      ),
                      if (c.isActive && c.remainingTime != null)
                        Text(
                          '남은 시간 ${_formatDuration(c.remainingTime!)}',
                          style: AppTypography.caption
                              .copyWith(color: AppColors.success),
                        ),
                    ],
                  ),
                ),
                // Status badge
                _buildStatusBadge(c),
                const Icon(
                  Icons.chevron_right,
                  size: 20,
                  color: AppColors.disabled,
                ),
              ],
            ),
            const SizedBox(height: AppSpacing.sm),
            const Divider(height: 1),
          ],
        ),
      ),
    );
  }

  Widget _buildStatusBadge(Consultation c) {
    String label;
    Color color;

    if (c.isActive) {
      label = '진행중';
      color = AppColors.success;
    } else if (c.status == ConsultationStatus.generating) {
      label = '대기중';
      color = AppColors.accent;
    } else {
      label = '만료됨';
      color = AppColors.disabled;
    }

    return Container(
      padding: const EdgeInsets.symmetric(
        horizontal: AppSpacing.sm,
        vertical: 2,
      ),
      decoration: BoxDecoration(
        color: color.withValues(alpha: 0.1),
        borderRadius: BorderRadius.circular(4),
      ),
      child: Text(
        label,
        style: AppTypography.caption.copyWith(
          color: color,
          fontSize: 11,
        ),
      ),
    );
  }

  String _formatDate(DateTime dt) {
    return '${dt.year}.${dt.month.toString().padLeft(2, '0')}.${dt.day.toString().padLeft(2, '0')}';
  }

  String _formatDuration(Duration d) {
    final h = d.inHours;
    final m = d.inMinutes % 60;
    return '$h:${m.toString().padLeft(2, '0')}';
  }
}
