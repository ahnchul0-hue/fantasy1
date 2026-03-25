import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:cached_network_image/cached_network_image.dart';

import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../shared/models/consultation.dart';
import '../../shared/widgets/widgets.dart';
import 'consultation_providers.dart';

/// Paid consultation result screen with Progressive Reveal
/// Phase 1: LLM text streaming, Phase 2: text + images loading
/// Vertical scroll (no carousel) — sections: 요약, 성격, 연애운, 재물운, 커리어, 조언
class ConsultationResultScreen extends ConsumerStatefulWidget {
  final String consultationId;

  const ConsultationResultScreen({
    super.key,
    required this.consultationId,
  });

  @override
  ConsumerState<ConsultationResultScreen> createState() =>
      _ConsultationResultScreenState();
}

class _ConsultationResultScreenState
    extends ConsumerState<ConsultationResultScreen> {
  Timer? _pollingTimer;

  @override
  void initState() {
    super.initState();
    // 상태가 generating이면 주기적으로 폴링
    _startPollingIfNeeded();
  }

  @override
  void dispose() {
    _pollingTimer?.cancel();
    super.dispose();
  }

  void _startPollingIfNeeded() {
    _pollingTimer = Timer.periodic(const Duration(seconds: 3), (_) {
      final status = ref.read(consultationStatusProvider(widget.consultationId));
      status.whenData((consultation) {
        if (consultation.status != ConsultationStatus.generating) {
          _pollingTimer?.cancel();
          _pollingTimer = null;
        }
      });
      // 상태 갱신을 위해 invalidate
      ref.invalidate(consultationStatusProvider(widget.consultationId));
    });
  }

  @override
  Widget build(BuildContext context) {
    final consultationAsync =
        ref.watch(consultationStatusProvider(widget.consultationId));

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
      body: consultationAsync.when(
        loading: () => _buildProgressiveLoading(null),
        error: (e, _) => ErrorRetryWidget(
          message: '해석 준비 중. 알림을 보내드리겠습니다',
          onRetry: () {
            ref.invalidate(
                consultationStatusProvider(widget.consultationId));
          },
        ),
        data: (consultation) {
          // 아직 생성 중이면 진행 상태 표시
          if (consultation.status == ConsultationStatus.generating) {
            return _buildProgressiveLoading(consultation);
          }

          // 실패
          if (consultation.status == ConsultationStatus.failed) {
            return ErrorRetryWidget(
              message: '해석에 실패했습니다. 다시 시도해주세요',
              onRetry: () {
                ref.invalidate(
                    consultationStatusProvider(widget.consultationId));
              },
            );
          }

          // 완료 또는 만료 — 결과 표시
          return _buildResult(context, consultation);
        },
      ),
    );
  }

  /// Progressive Reveal: 생성 중 로딩 상태
  Widget _buildProgressiveLoading(Consultation? consultation) {
    final checkpointLabel = _checkpointLabel(
      consultation?.checkpointStatus ?? CheckpointStatus.none,
    );

    return Column(
      children: [
        // Progress header
        Padding(
          padding: const EdgeInsets.all(AppSpacing.md),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                checkpointLabel,
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
            separatorBuilder: (_, __) =>
                const Divider(height: AppSpacing.lg),
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

  String _checkpointLabel(CheckpointStatus status) {
    switch (status) {
      case CheckpointStatus.none:
        return '사주 해석을 준비하고 있습니다';
      case CheckpointStatus.analysisDone:
        return '분석이 완료되었습니다. 이미지를 생성하고 있습니다';
      case CheckpointStatus.imagesDone:
        return '이미지가 완료되었습니다. 마무리하고 있습니다';
      case CheckpointStatus.complete:
        return '해석이 완료되었습니다';
    }
  }

  /// Full result with sections
  Widget _buildResult(BuildContext context, Consultation consultation) {
    // Sections for vertical scroll
    const sections = ['성격', '연애운', '재물운', '커리어', '조언'];

    return Column(
      children: [
        Expanded(
          child: SingleChildScrollView(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                // Summary header
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
                  final hasImage =
                      consultation.resultImages.length > index;

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
                              const LoadingSkeleton(height: 200),
                            const SizedBox(height: AppSpacing.md),
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
        if (consultation.isActive)
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
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  if (consultation.remainingTime != null)
                    Padding(
                      padding:
                          const EdgeInsets.only(bottom: AppSpacing.xs),
                      child: Text(
                        '남은 시간: ${_formatDuration(consultation.remainingTime!)} · 채팅 ${consultation.chatTurnsRemaining}회 남음',
                        style: AppTypography.caption.copyWith(
                          color: AppColors.secondaryText,
                        ),
                      ),
                    ),
                  PrimaryButton(
                    label: '월하선생에게 물어보기',
                    onPressed: () => context.push(
                      '/consultation/${widget.consultationId}/chat',
                    ),
                  ),
                ],
              ),
            ),
          ),

        // 만료된 경우
        if (consultation.isExpired)
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
              child: Text(
                '상담 기간이 만료되었습니다',
                style: AppTypography.body.copyWith(
                  color: AppColors.secondaryText,
                ),
                textAlign: TextAlign.center,
              ),
            ),
          ),
      ],
    );
  }

  String _formatDuration(Duration duration) {
    final hours = duration.inHours;
    final minutes = duration.inMinutes.remainder(60);
    if (hours > 0) {
      return '${hours}시간 ${minutes}분';
    }
    return '${minutes}분';
  }
}
