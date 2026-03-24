import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/api/api_client.dart';
import '../../shared/models/models.dart';

/// Saju profile provider
final profileProvider =
    FutureProvider.autoDispose<SajuProfile?>((ref) async {
  final apiClient = ref.watch(apiClientProvider);
  try {
    return await apiClient.getProfile();
  } catch (_) {
    return null;
  }
});
