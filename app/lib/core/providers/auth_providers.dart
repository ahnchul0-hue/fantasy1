import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../shared/models/models.dart';
import '../auth/auth_service.dart';
import '../api/api_client.dart';

/// Auth state: null = not logged in, AppUser = logged in
final authStateProvider =
    StateNotifierProvider<AuthStateNotifier, AsyncValue<AppUser?>>((ref) {
  return AuthStateNotifier(
    authService: ref.watch(authServiceProvider),
    apiClient: ref.watch(apiClientProvider),
  );
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
  }

  Future<void> login({
    required String provider,
    required String token,
  }) async {
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
      state = AsyncValue.data(response.user);
    } catch (e, st) {
      state = AsyncValue.error(e, st);
    }
  }

  Future<void> logout() async {
    await _authService.clearTokens();
    state = const AsyncValue.data(null);
  }

  Future<void> deleteAccount() async {
    state = const AsyncValue.loading();
    try {
      await _apiClient.deleteAccount();
      await _authService.clearTokens();
      state = const AsyncValue.data(null);
    } catch (e, st) {
      state = AsyncValue.error(e, st);
    }
  }
}
