import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../core/providers/auth_providers.dart';

/// 야자시/조자시 toggle state
final yajaSiModeProvider = StateProvider<bool>((ref) => true); // default: 야자시론

/// Settings screen
/// 계정, 캐릭터 선택, 알림, 야자시/조자시, 고객지원, 약관, 계정 삭제
class SettingsScreen extends ConsumerWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final authState = ref.watch(authStateProvider);
    final isLoggedIn =
        authState.valueOrNull != null;
    final yajaMode = ref.watch(yajaSiModeProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('설정'),
      ),
      body: ListView(
        children: [
          // Account section
          _buildSectionHeader('계정'),
          if (isLoggedIn) ...[
            _buildItem(
              title: '로그인 정보',
              subtitle: authState.valueOrNull?.provider ?? '',
              onTap: null,
            ),
          ] else ...[
            _buildItem(
              title: '로그인',
              onTap: () => context.push('/login'),
            ),
          ],

          const Divider(),

          // Saju settings
          _buildSectionHeader('사주 설정'),
          _buildToggleItem(
            title: '야자시론 사용',
            subtitle: '23:00~01:00을 전날 자시로 계산',
            value: yajaMode,
            onChanged: (v) =>
                ref.read(yajaSiModeProvider.notifier).state = v,
          ),

          const Divider(),

          // Character selection
          _buildSectionHeader('AI 상담'),
          _buildItem(
            title: '상담 캐릭터',
            subtitle: '월하선생',
            onTap: () {
              // TODO: Character selection screen (v1: single character)
            },
          ),

          const Divider(),

          // Notification settings
          _buildSectionHeader('알림'),
          _buildItem(
            title: '알림 설정',
            onTap: () {
              // TODO: Notification settings
            },
          ),

          const Divider(),

          // Support & legal
          _buildSectionHeader('고객지원'),
          _buildItem(
            title: '문의하기',
            onTap: () {
              // TODO: Open support email/channel
            },
          ),
          _buildItem(
            title: '이용약관',
            onTap: () {
              // TODO: Terms of service webview
            },
          ),
          _buildItem(
            title: '개인정보처리방침',
            onTap: () {
              // TODO: Privacy policy webview
            },
          ),

          if (isLoggedIn) ...[
            const Divider(),

            // Account management
            _buildSectionHeader('계정 관리'),
            _buildItem(
              title: '로그아웃',
              onTap: () async {
                final confirm = await _showConfirmDialog(
                  context,
                  title: '로그아웃',
                  message: '로그아웃 하시겠습니까?',
                );
                if (confirm == true) {
                  await ref.read(authStateProvider.notifier).logout();
                  if (context.mounted) context.go('/home');
                }
              },
            ),
            _buildItem(
              title: '계정 삭제',
              titleColor: AppColors.error,
              onTap: () => context.push('/settings/delete-account'),
            ),
          ],

          const SizedBox(height: AppSpacing.lg),

          // App version
          Center(
            child: Text(
              'v1.0.0',
              style: AppTypography.caption.copyWith(
                color: AppColors.disabled,
              ),
            ),
          ),

          const SizedBox(height: AppSpacing.lg),
        ],
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
      child: Text(
        title,
        style: AppTypography.caption.copyWith(
          fontWeight: FontWeight.w600,
        ),
      ),
    );
  }

  Widget _buildItem({
    required String title,
    String? subtitle,
    Color? titleColor,
    VoidCallback? onTap,
  }) {
    return InkWell(
      onTap: onTap,
      child: Padding(
        padding: const EdgeInsets.symmetric(
          horizontal: AppSpacing.md,
          vertical: 14,
        ),
        child: Row(
          children: [
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    title,
                    style: AppTypography.body.copyWith(
                      color: titleColor ?? AppColors.onSurface,
                    ),
                  ),
                  if (subtitle != null)
                    Text(subtitle, style: AppTypography.caption),
                ],
              ),
            ),
            if (onTap != null)
              const Icon(
                Icons.chevron_right,
                size: 20,
                color: AppColors.disabled,
              ),
          ],
        ),
      ),
    );
  }

  Widget _buildToggleItem({
    required String title,
    required String subtitle,
    required bool value,
    required ValueChanged<bool> onChanged,
  }) {
    return Padding(
      padding: const EdgeInsets.symmetric(
        horizontal: AppSpacing.md,
        vertical: AppSpacing.sm,
      ),
      child: Row(
        children: [
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(title, style: AppTypography.body),
                Text(subtitle, style: AppTypography.caption),
              ],
            ),
          ),
          Switch(
            value: value,
            onChanged: onChanged,
            activeColor: AppColors.accent,
          ),
        ],
      ),
    );
  }

  Future<bool?> _showConfirmDialog(
    BuildContext context, {
    required String title,
    required String message,
  }) {
    return showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(title),
        content: Text(message),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: const Text('취소'),
          ),
          TextButton(
            onPressed: () => Navigator.pop(context, true),
            child: const Text('확인'),
          ),
        ],
      ),
    );
  }
}
