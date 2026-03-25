import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/api/api_client.dart';
import '../../shared/models/models.dart';

/// Daily fortune provider — kept alive for the day, errors propagate
final dailyFortuneProvider = FutureProvider<DailyFortune?>((ref) async {
  final apiClient = ref.watch(apiClientProvider);
  try {
    return await apiClient.getDailyFortune();
  } on DioException catch (e) {
    if (e.response?.statusCode == 404) {
      return null;
    }
    rethrow;
  }
});
