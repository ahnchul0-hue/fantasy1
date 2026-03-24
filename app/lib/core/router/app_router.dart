import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../features/home/home_screen.dart';
import '../../features/profile/profile_screen.dart';
import '../../features/history/history_screen.dart';
import '../../features/settings/settings_screen.dart';
import '../../features/birth_input/birth_input_screen.dart';
import '../../features/saju_card/saju_card_result_screen.dart';
import '../../features/consultation/consultation_preview_screen.dart';
import '../../features/consultation/consultation_result_screen.dart';
import '../../features/chat/chat_screen.dart';
import '../../features/compatibility/compatibility_screen.dart';
import '../../features/fortune/fortune_screen.dart';
import '../../features/auth/login_screen.dart';
import '../../features/settings/account_deletion_screen.dart';
import '../shell/app_shell.dart';

final routerProvider = Provider<GoRouter>((ref) {
  return GoRouter(
    initialLocation: '/home',
    routes: [
      // Bottom nav shell
      ShellRoute(
        builder: (context, state, child) => AppShell(child: child),
        routes: [
          GoRoute(
            path: '/home',
            pageBuilder: (context, state) => const NoTransitionPage(
              child: HomeScreen(),
            ),
          ),
          GoRoute(
            path: '/profile',
            pageBuilder: (context, state) => const NoTransitionPage(
              child: ProfileScreen(),
            ),
          ),
          GoRoute(
            path: '/history',
            pageBuilder: (context, state) => const NoTransitionPage(
              child: HistoryScreen(),
            ),
          ),
          GoRoute(
            path: '/settings',
            pageBuilder: (context, state) => const NoTransitionPage(
              child: SettingsScreen(),
            ),
          ),
        ],
      ),

      // Stack routes
      GoRoute(
        path: '/login',
        builder: (context, state) => const LoginScreen(),
      ),
      GoRoute(
        path: '/birth-input',
        builder: (context, state) {
          final extra = state.extra as Map<String, dynamic>?;
          return BirthInputScreen(
            purpose: extra?['purpose'] as String? ?? 'card',
          );
        },
      ),
      GoRoute(
        path: '/saju-card/:id',
        builder: (context, state) => SajuCardResultScreen(
          cardId: state.pathParameters['id']!,
        ),
      ),
      GoRoute(
        path: '/consultation/preview',
        builder: (context, state) => const ConsultationPreviewScreen(),
      ),
      GoRoute(
        path: '/consultation/:id/result',
        builder: (context, state) => ConsultationResultScreen(
          consultationId: state.pathParameters['id']!,
        ),
      ),
      GoRoute(
        path: '/consultation/:id/chat',
        builder: (context, state) => ChatScreen(
          consultationId: state.pathParameters['id']!,
        ),
      ),
      GoRoute(
        path: '/compatibility',
        builder: (context, state) => const CompatibilityScreen(),
      ),
      GoRoute(
        path: '/fortune',
        builder: (context, state) => const FortuneScreen(),
      ),
      GoRoute(
        path: '/settings/delete-account',
        builder: (context, state) => const AccountDeletionScreen(),
      ),
    ],
  );
});
