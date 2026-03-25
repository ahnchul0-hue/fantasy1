import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../core/providers/auth_providers.dart';
import '../../shared/widgets/widgets.dart';

/// 계정 삭제 플로우 — Apple/Google 정책 필수
/// 2-step confirmation: explain consequences -> confirm deletion
class AccountDeletionScreen extends ConsumerStatefulWidget {
  const AccountDeletionScreen({super.key});

  @override
  ConsumerState<AccountDeletionScreen> createState() =>
      _AccountDeletionScreenState();
}

class _AccountDeletionScreenState
    extends ConsumerState<AccountDeletionScreen> {
  bool _confirmed = false;
  bool _isDeleting = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => context.pop(),
          tooltip: '뒤로',
        ),
        title: const Text('계정 삭제'),
      ),
      body: SafeArea(
        child: Padding(
          padding: const EdgeInsets.all(AppSpacing.md),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                '계정을 삭제하시겠습니까?',
                style: AppTypography.title,
              ),
              const SizedBox(height: AppSpacing.lg),

              Text(
                '계정을 삭제하면 다음 데이터가 영구적으로 삭제됩니다:',
                style: AppTypography.body,
              ),
              const SizedBox(height: AppSpacing.md),

              _buildDeleteItem('사주 프로필 및 분석 기록'),
              _buildDeleteItem('AI 상담 채팅 이력'),
              _buildDeleteItem('결제 이력 (환불은 앱스토어에서 진행)'),
              _buildDeleteItem('계정 정보'),

              const SizedBox(height: AppSpacing.lg),

              Text(
                '이 작업은 되돌릴 수 없습니다.',
                style: AppTypography.bodySemiBold.copyWith(
                  color: AppColors.error,
                ),
              ),

              const SizedBox(height: AppSpacing.lg),

              // Confirmation checkbox
              GestureDetector(
                onTap: () => setState(() => _confirmed = !_confirmed),
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    SizedBox(
                      width: 24,
                      height: 24,
                      child: Checkbox(
                        value: _confirmed,
                        onChanged: (v) =>
                            setState(() => _confirmed = v ?? false),
                        activeColor: AppColors.error,
                      ),
                    ),
                    const SizedBox(width: AppSpacing.sm),
                    Expanded(
                      child: Text(
                        '위 내용을 확인했으며, 계정 삭제에 동의합니다',
                        style: AppTypography.body,
                      ),
                    ),
                  ],
                ),
              ),

              const Spacer(),

              // Delete button
              SizedBox(
                width: double.infinity,
                height: AppDimensions.ctaPrimaryHeight,
                child: ElevatedButton(
                  onPressed:
                      _confirmed && !_isDeleting ? _handleDelete : null,
                  style: ElevatedButton.styleFrom(
                    backgroundColor: AppColors.error,
                    foregroundColor: Colors.white,
                    disabledBackgroundColor: AppColors.disabled,
                    shape: RoundedRectangleBorder(
                      borderRadius:
                          BorderRadius.circular(AppRadius.button),
                    ),
                    elevation: 0,
                  ),
                  child: _isDeleting
                      ? const SizedBox(
                          width: 24,
                          height: 24,
                          child: CircularProgressIndicator(
                            strokeWidth: 2.5,
                            color: Colors.white,
                          ),
                        )
                      : Text(
                          '계정 삭제',
                          style: AppTypography.bodySemiBold.copyWith(
                            color: Colors.white,
                          ),
                        ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildDeleteItem(String text) {
    return Padding(
      padding: const EdgeInsets.only(bottom: AppSpacing.sm),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Padding(
            padding: EdgeInsets.only(top: 6),
            child: Icon(Icons.remove, size: 14, color: AppColors.error),
          ),
          const SizedBox(width: AppSpacing.sm),
          Expanded(
            child: Text(text, style: AppTypography.body),
          ),
        ],
      ),
    );
  }

  Future<void> _handleDelete() async {
    // Second confirmation dialog
    final confirm = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('최종 확인'),
        content: const Text(
          '정말로 계정을 삭제하시겠습니까? 이 작업은 되돌릴 수 없습니다.',
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: const Text('취소'),
          ),
          TextButton(
            onPressed: () => Navigator.pop(context, true),
            child: Text(
              '삭제',
              style: TextStyle(color: AppColors.error),
            ),
          ),
        ],
      ),
    );

    if (confirm != true) return;

    setState(() => _isDeleting = true);
    try {
      final success =
          await ref.read(authStateProvider.notifier).deleteAccount();
      if (success && mounted) {
        context.go('/home');
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('계정이 삭제되었습니다')),
        );
      } else if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('계정 삭제에 실패했습니다. 다시 시도해주세요')),
        );
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('계정 삭제에 실패했습니다. 다시 시도해주세요')),
        );
      }
    } finally {
      if (mounted) setState(() => _isDeleting = false);
    }
  }
}
