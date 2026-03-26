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
import '../../shared/models/birth_input.dart';
import '../providers/auth_providers.dart';
import '../shell/app_shell.dart';

// 인증 필수 경로 (prefix match — /consultation/:id/result 등도 포함)
const _protectedRoutes = {
  '/profile',
  '/history',
  '/settings',
  '/consultation',
  '/fortune',
};

// 인증 불필요 경로
const _publicRoutes = {
  '/login',
  '/home',
  '/birth-input',
};

/// Navigation keys for StatefulShellRoute (탭별 네비게이션 상태 보존)
final _homeNavigatorKey = GlobalKey<NavigatorState>(debugLabel: 'home');
final _profileNavigatorKey = GlobalKey<NavigatorState>(debugLabel: 'profile');
final _historyNavigatorKey = GlobalKey<NavigatorState>(debugLabel: 'history');
final _settingsNavigatorKey = GlobalKey<NavigatorState>(debugLabel: 'settings');

final routerProvider = Provider<GoRouter>((ref) {
  // refreshListenable: 인증 상태 변경 시 redirect 재평가
  final authState = ref.watch(authStateProvider);

  return GoRouter(
    initialLocation: '/home',
    redirect: (context, state) {
      final isLoading = authState.isLoading;
      final isLoggedIn = authState.valueOrNull != null;
      final currentPath = state.matchedLocation;

      // 인증 상태 로딩 중 → redirect 하지 않음 (딥링크 보존)
      if (isLoading) return null;

      // 인증 필수 경로에 비로그인 사용자 → 로그인 페이지
      if (!isLoggedIn && _protectedRoutes.any((r) => currentPath.startsWith(r))) {
        return '/login';
      }

      // 로그인 페이지에 이미 로그인된 사용자 → 홈
      if (isLoggedIn && currentPath == '/login') {
        return '/home';
      }

      return null;
    },
    routes: [
      // Bottom nav shell — StatefulShellRoute로 탭별 상태 보존
      StatefulShellRoute.indexedStack(
        builder: (context, state, navigationShell) =>
            AppShell(navigationShell: navigationShell),
        branches: [
          StatefulShellBranch(
            navigatorKey: _homeNavigatorKey,
            routes: [
              GoRoute(
                path: '/home',
                pageBuilder: (context, state) => const NoTransitionPage(
                  child: HomeScreen(),
                ),
              ),
            ],
          ),
          StatefulShellBranch(
            navigatorKey: _profileNavigatorKey,
            routes: [
              GoRoute(
                path: '/profile',
                pageBuilder: (context, state) => const NoTransitionPage(
                  child: ProfileScreen(),
                ),
              ),
            ],
          ),
          StatefulShellBranch(
            navigatorKey: _historyNavigatorKey,
            routes: [
              GoRoute(
                path: '/history',
                pageBuilder: (context, state) => const NoTransitionPage(
                  child: HistoryScreen(),
                ),
              ),
            ],
          ),
          StatefulShellBranch(
            navigatorKey: _settingsNavigatorKey,
            routes: [
              GoRoute(
                path: '/settings',
                pageBuilder: (context, state) => const NoTransitionPage(
                  child: SettingsScreen(),
                ),
              ),
            ],
          ),
        ],
      ),

      // Stack routes (full-screen, no bottom nav)
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
        path: '/saju-card/new',
        builder: (context, state) => SajuCardResultScreen(
          birthInput: state.extra as BirthInput?,
        ),
      ),
      GoRoute(
        path: '/saju-card/:id',
        builder: (context, state) => SajuCardResultScreen(
          cardId: state.pathParameters['id']!,
        ),
      ),
      GoRoute(
        path: '/consultation/preview',
        builder: (context, state) => ConsultationPreviewScreen(
          birthInput: state.extra as BirthInput?,
        ),
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
