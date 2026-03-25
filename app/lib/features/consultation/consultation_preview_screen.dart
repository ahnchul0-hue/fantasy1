import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../shared/models/birth_input.dart';
import '../../shared/models/payment.dart';
import '../../shared/widgets/widgets.dart';
import '../payment/payment_providers.dart';
import 'consultation_providers.dart';

/// 유료 전환 — 신뢰 스캐폴딩 화면
/// Blurred samples + example questions + social proof + disclaimer + CTA
class ConsultationPreviewScreen extends ConsumerStatefulWidget {
  final BirthInput? birthInput;

  const ConsultationPreviewScreen({super.key, this.birthInput});

  @override
  ConsumerState<ConsultationPreviewScreen> createState() =>
      _ConsultationPreviewScreenState();
}

class _ConsultationPreviewScreenState
    extends ConsumerState<ConsultationPreviewScreen> {
  bool _isProcessing = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => context.pop(),
          tooltip: '뒤로',
        ),
      ),
      body: Column(
        children: [
          Expanded(
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(AppSpacing.md),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  // Title
                  Text('AI 사주 상담', style: AppTypography.display),
                  const SizedBox(height: AppSpacing.xs),
                  Text(
                    '15,000원 · 72시간 AI 채팅 포함',
                    style: AppTypography.body
                        .copyWith(color: AppColors.secondaryText),
                  ),

                  const SizedBox(height: AppSpacing.lg),
                  const Divider(),
                  const SizedBox(height: AppSpacing.lg),

                  // Blurred sample images
                  Text(
                    '이런 분석을 받을 수 있어요',
                    style: AppTypography.bodySemiBold,
                  ),
                  const SizedBox(height: AppSpacing.md),
                  SizedBox(
                    height: 160,
                    child: ListView.separated(
                      scrollDirection: Axis.horizontal,
                      itemCount: 3,
                      separatorBuilder: (_, __) =>
                          const SizedBox(width: AppSpacing.sm),
                      itemBuilder: (_, index) => _BlurredSample(index: index),
                    ),
                  ),
                  const SizedBox(height: AppSpacing.xs),
                  Text(
                    '성격 · 연애 · 재물 · 커리어 · 조언',
                    style: AppTypography.caption,
                  ),

                  const SizedBox(height: AppSpacing.lg),
                  const Divider(),
                  const SizedBox(height: AppSpacing.lg),

                  // Example questions
                  Text(
                    '이런 질문을 할 수 있어요:',
                    style: AppTypography.bodySemiBold,
                  ),
                  const SizedBox(height: AppSpacing.sm),
                  ..._exampleQuestions.map((q) => Padding(
                        padding: const EdgeInsets.only(bottom: AppSpacing.sm),
                        child: Row(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text('"',
                                style: AppTypography.body
                                    .copyWith(color: AppColors.accent)),
                            Expanded(
                              child: Text(q, style: AppTypography.body),
                            ),
                            Text('"',
                                style: AppTypography.body
                                    .copyWith(color: AppColors.accent)),
                          ],
                        ),
                      )),

                  const SizedBox(height: AppSpacing.lg),
                  const Divider(),
                  const SizedBox(height: AppSpacing.lg),

                  // Social proof
                  Row(
                    children: [
                      Text(
                        '누적 상담 0건',
                        style: AppTypography.bodySemiBold,
                      ),
                      const SizedBox(width: AppSpacing.md),
                      Row(
                        children: [
                          const Icon(Icons.star, size: 16, color: AppColors.accent),
                          const SizedBox(width: 2),
                          Text('4.8', style: AppTypography.bodySemiBold),
                        ],
                      ),
                    ],
                  ),

                  const SizedBox(height: AppSpacing.lg),

                  // Disclaimer — 면책 고지 (13px, #8B8B8B)
                  Text(
                    '본 서비스는 엔터테인먼트 목적이며 전문 상담을 대체하지 않습니다',
                    style: AppTypography.caption.copyWith(
                      color: AppColors.disabled,
                      fontSize: 13,
                    ),
                  ),

                  const SizedBox(height: AppSpacing.lg),
                ],
              ),
            ),
          ),

          // Sticky bottom CTA
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
                label: '상담 시작하기',
                isLoading: _isProcessing,
                onPressed: _handlePurchase,
              ),
            ),
          ),
        ],
      ),
    );
  }

  static const _exampleQuestions = [
    '올해 이직해도 될까요?',
    '애인과 궁합이 어때요?',
    '내년에 사업 시작해도 될까요?',
  ];

  Future<void> _handlePurchase() async {
    setState(() => _isProcessing = true);
    try {
      final paymentNotifier = ref.read(paymentFlowProvider.notifier);
      final orderId = await paymentNotifier.purchaseAndVerify(
        PaymentProduct.sajuConsultation,
      );

      if (orderId == null) {
        // 사용자 취소이거나 에러 — 에러 시 snackbar 표시
        final paymentState = ref.read(paymentFlowProvider);
        if (paymentState.error != null && mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text(paymentState.error!)),
          );
        }
        return;
      }

      // 결제 성공 → birthInput이 있으면 상담 생성 후 이동
      if (widget.birthInput != null) {
        final consultationId = await ref
            .read(consultationCreationProvider.notifier)
            .startConsultation(
              birthInput: widget.birthInput!,
              orderId: orderId,
            );
        if (consultationId != null && mounted) {
          context.push('/consultation/$consultationId/result');
        } else if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('상담 생성에 실패했습니다. 다시 시도해주세요')),
          );
        }
      } else if (mounted) {
        // birthInput 없이 결제만 된 경우 (fallback)
        context.push('/consultation/$orderId/result');
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('결제에 실패했습니다. 다시 시도해주세요')),
        );
      }
    } finally {
      if (mounted) setState(() => _isProcessing = false);
    }
  }
}

class _BlurredSample extends StatelessWidget {
  final int index;
  const _BlurredSample({required this.index});

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(AppRadius.card),
      child: ImageFiltered(
        imageFilter: ImageFilter.blur(sigmaX: 12, sigmaY: 12),
        child: Container(
          width: 120,
          height: 160,
          color: [
            AppColors.primary,
            AppColors.accent,
            AppColors.divider,
          ][index]
              .withValues(alpha: 0.3),
          child: Center(
            child: Text(
              ['성격', '연애', '재물'][index],
              style: AppTypography.body.copyWith(
                color: AppColors.onSurface.withValues(alpha: 0.5),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
