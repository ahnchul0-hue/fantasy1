import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/api/api_client.dart';
import '../../shared/models/models.dart';

/// Daily fortune provider — cached for the day
final dailyFortuneProvider =
    FutureProvider.autoDispose<DailyFortune?>((ref) async {
  final apiClient = ref.watch(apiClientProvider);
  try {
    return await apiClient.getDailyFortune();
  } catch (_) {
    return null;
  }
});
