import 'dart:convert';

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
    await _storage.write(
      key: _userKey,
      value: jsonEncode({
        'id': user.id,
        'provider': user.provider,
        'nickname': user.nickname,
        'hasProfile': user.hasProfile,
        'createdAt': user.createdAt.toIso8601String(),
      }),
    );
  }

  Future<AppUser?> getUser() async {
    final raw = await _storage.read(key: _userKey);
    if (raw == null) return null;
    try {
      final json = jsonDecode(raw) as Map<String, dynamic>;
      return AppUser(
        id: json['id'] as String,
        provider: json['provider'] as String,
        nickname: json['nickname'] as String?,
        hasProfile: json['hasProfile'] as bool? ?? false,
        createdAt: DateTime.parse(json['createdAt'] as String),
      );
    } catch (_) {
      return null;
    }
  }

  Future<void> clearTokens() async {
    await _storage.delete(key: _accessTokenKey);
    await _storage.delete(key: _refreshTokenKey);
    await _storage.delete(key: _userKey);
  }

  Future<bool> get isLoggedIn async {
    final token = await getAccessToken();
    return token != null;
  }
}

final authServiceProvider = Provider<AuthService>((ref) {
  return AuthService();
});
