import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../core/theme/app_colors.dart';
import '../../core/theme/app_typography.dart';
import '../../core/theme/app_spacing.dart';
import '../../core/providers/auth_providers.dart';

/// Social login screen — 카카오 / Apple / Google
class LoginScreen extends ConsumerWidget {
  const LoginScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final authState = ref.watch(authStateProvider);
    final isLoading = authState is AsyncLoading;

    return Scaffold(
      body: SafeArea(
        child: Padding(
          padding: const EdgeInsets.all(AppSpacing.lg),
          child: Column(
            children: [
              const Spacer(flex: 2),

              // Brand
              Container(
                width: 80,
                height: 80,
                decoration: BoxDecoration(
                  color: AppColors.primary,
                  borderRadius: BorderRadius.circular(20),
                ),
                child: const Center(
                  child: Text(
                    '命',
                    style: TextStyle(
                      fontFamily: AppTypography.fontHanja,
                      fontSize: 40,
                      color: AppColors.accent,
                    ),
                  ),
                ),
              ),
              const SizedBox(height: AppSpacing.lg),
              Text('사주', style: AppTypography.display),
              const SizedBox(height: AppSpacing.sm),
              Text(
                'AI 사주 상담',
                style: AppTypography.body.copyWith(
                  color: AppColors.secondaryText,
                ),
              ),

              const Spacer(flex: 3),

              // Login buttons
              if (isLoading)
                const CircularProgressIndicator()
              else ...[
                // Kakao login
                _LoginButton(
                  label: '카카오로 시작하기',
                  backgroundColor: const Color(0xFFFEE500),
                  textColor: const Color(0xFF191919),
                  icon: Icons.chat_bubble,
                  onPressed: () => _handleLogin(ref, 'kakao'),
                ),
                const SizedBox(height: AppSpacing.sm),

                // Apple login
                _LoginButton(
                  label: 'Apple로 시작하기',
                  backgroundColor: AppColors.primary,
                  textColor: AppColors.surface,
                  icon: Icons.apple,
                  onPressed: () => _handleLogin(ref, 'apple'),
                ),
                const SizedBox(height: AppSpacing.sm),

                // Google login
                _LoginButton(
                  label: 'Google로 시작하기',
                  backgroundColor: Colors.white,
                  textColor: AppColors.onSurface,
                  icon: Icons.g_mobiledata,
                  border: true,
                  onPressed: () => _handleLogin(ref, 'google'),
                ),
              ],

              const SizedBox(height: AppSpacing.lg),

              // Skip login
              TextButton(
                onPressed: () => context.go('/home'),
                child: Text(
                  '로그인 없이 둘러보기',
                  style: AppTypography.caption.copyWith(
                    color: AppColors.secondaryText,
                  ),
                ),
              ),

              const Spacer(),
            ],
          ),
        ),
      ),
    );
  }

  Future<void> _handleLogin(WidgetRef ref, String provider) async {
    // TODO: Integrate actual social login SDK
    // For now, simulate with a placeholder token
    await ref.read(authStateProvider.notifier).login(
          provider: provider,
          token: 'placeholder_social_token',
        );
  }
}

class _LoginButton extends StatelessWidget {
  final String label;
  final Color backgroundColor;
  final Color textColor;
  final IconData icon;
  final bool border;
  final VoidCallback onPressed;

  const _LoginButton({
    required this.label,
    required this.backgroundColor,
    required this.textColor,
    required this.icon,
    this.border = false,
    required this.onPressed,
  });

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      height: AppDimensions.ctaPrimaryHeight,
      child: ElevatedButton(
        onPressed: onPressed,
        style: ElevatedButton.styleFrom(
          backgroundColor: backgroundColor,
          foregroundColor: textColor,
          elevation: 0,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(AppRadius.button),
            side: border
                ? const BorderSide(color: AppColors.divider)
                : BorderSide.none,
          ),
        ),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(icon, size: 24),
            const SizedBox(width: AppSpacing.sm),
            Text(label, style: AppTypography.bodySemiBold.copyWith(
              color: textColor,
            )),
          ],
        ),
      ),
    );
  }
}
