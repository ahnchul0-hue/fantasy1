import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/api/api_client.dart';
import '../../shared/models/models.dart';

/// Saju profile provider — kept alive to avoid refetching on tab switch
final profileProvider = FutureProvider<SajuProfile?>((ref) async {
  final apiClient = ref.watch(apiClientProvider);
  try {
    return await apiClient.getProfile();
  } on DioException catch (e) {
    if (e.response?.statusCode == 404) {
      // No profile yet — this is expected for new users
      return null;
    }
    rethrow;
  }
});
