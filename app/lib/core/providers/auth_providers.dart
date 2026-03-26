import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:purchases_flutter/purchases_flutter.dart';
import '../../shared/models/models.dart';
import '../auth/auth_service.dart';
import '../api/api_client.dart';

/// Auth state: null = not logged in, AppUser = logged in
final authStateProvider =
    StateNotifierProvider<AuthStateNotifier, AsyncValue<AppUser?>>((ref) {
  final notifier = AuthStateNotifier(
    authService: ref.watch(authServiceProvider),
    apiClient: ref.watch(apiClientProvider),
  );
  // AuthInterceptor에 토큰 refresh 실패 콜백 연결
  ref.watch(authInterceptorProvider).onTokenRefreshFailed =
      notifier.onTokenRefreshFailed;
  return notifier;
});

class AuthStateNotifier extends StateNotifier<AsyncValue<AppUser?>> {
  final AuthService _authService;
  final ApiClient _apiClient;

  AuthStateNotifier({
    required AuthService authService,
    required ApiClient apiClient,
  })  : _authService = authService,
        _apiClient = apiClient,
        super(const AsyncValue.loading()) {
    _init();
  }

  Future<void> _init() async {
    final user = await _authService.getUser();
    state = AsyncValue.data(user);
    // RevenueCat appUserID 동기화
    if (user != null) {
      try {
        await Purchases.logIn(user.id);
      } catch (_) {}
    }
  }

  Future<bool> login({
    required String provider,
    required String token,
  }) async {
    // 기존 상태를 보존하면서 로딩 표시 (UI 깜빡임 방지)
    final previousUser = state.valueOrNull;
    state = const AsyncValue.loading();
    try {
      final response = await _apiClient.login({
        'provider': provider,
        'token': token,
      });
      await _authService.saveTokens(
        accessToken: response.accessToken,
        refreshToken: response.refreshToken,
      );
      await _authService.saveUser(response.user);
      // RevenueCat appUserID 동기화
      try {
        await Purchases.logIn(response.user.id);
      } catch (_) {}
      state = AsyncValue.data(response.user);
      return true;
    } catch (e, st) {
      // 에러 시 이전 상태로 복원 (null or previous user)
      state = AsyncValue.data(previousUser);
      return false;
    }
  }

  Future<void> logout() async {
    await _authService.clearTokens();
    // RevenueCat 로그아웃
    try {
      await Purchases.logOut();
    } catch (_) {}
    state = const AsyncValue.data(null);
  }

  /// 프로필 생성/업데이트 후 hasProfile 플래그 갱신
  Future<void> refreshUserProfile() async {
    final currentUser = state.valueOrNull;
    if (currentUser == null) return;
    final updated = currentUser.copyWith(hasProfile: true);
    await _authService.saveUser(updated);
    state = AsyncValue.data(updated);
  }

  /// 토큰 refresh 실패 시 호출 — authState와 secure storage 동기화
  Future<void> onTokenRefreshFailed() async {
    await _authService.clearTokens();
    try {
      await Purchases.logOut();
    } catch (_) {}
    state = const AsyncValue.data(null);
  }

  Future<bool> deleteAccount() async {
    final previousUser = state.valueOrNull;
    state = const AsyncValue.loading();
    try {
      await _apiClient.deleteAccount();
      await _authService.clearTokens();
      try {
        await Purchases.logOut();
      } catch (_) {}
      state = const AsyncValue.data(null);
      return true;
    } catch (e, st) {
      state = AsyncValue.data(previousUser);
      return false;
    }
  }
}
