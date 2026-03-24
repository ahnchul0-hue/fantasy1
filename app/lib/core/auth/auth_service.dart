import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import '../../shared/models/user.dart';

/// Secure token storage and auth state management
class AuthService {
  static const _accessTokenKey = 'access_token';
  static const _refreshTokenKey = 'refresh_token';
  static const _userKey = 'user_json';

  final FlutterSecureStorage _storage;

  AuthService({FlutterSecureStorage? storage})
      : _storage = storage ?? const FlutterSecureStorage();

  Future<String?> getAccessToken() async {
    return _storage.read(key: _accessTokenKey);
  }

  Future<String?> getRefreshToken() async {
    return _storage.read(key: _refreshTokenKey);
  }

  Future<void> saveTokens({
    required String accessToken,
    required String refreshToken,
  }) async {
    await _storage.write(key: _accessTokenKey, value: accessToken);
    await _storage.write(key: _refreshTokenKey, value: refreshToken);
  }

  Future<void> saveUser(AppUser user) async {
    // Store as simple string for quick access
    await _storage.write(
      key: _userKey,
      value: '${user.id}|${user.provider}|${user.nickname ?? ''}|${user.hasProfile}|${user.createdAt.toIso8601String()}',
    );
  }

  Future<AppUser?> getUser() async {
    final raw = await _storage.read(key: _userKey);
    if (raw == null) return null;
    final parts = raw.split('|');
    if (parts.length < 5) return null;
    return AppUser(
      id: parts[0],
      provider: parts[1],
      nickname: parts[2].isEmpty ? null : parts[2],
      hasProfile: parts[3] == 'true',
      createdAt: DateTime.parse(parts[4]),
    );
  }

  Future<void> clearTokens() async {
    await _storage.deleteAll();
  }

  Future<bool> get isLoggedIn async {
    final token = await getAccessToken();
    return token != null;
  }
}

final authServiceProvider = Provider<AuthService>((ref) {
  return AuthService();
});
