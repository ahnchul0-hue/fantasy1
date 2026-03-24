import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/api/api_client.dart';
import '../../shared/models/models.dart';

/// Provider to create a free saju card
final sajuCardCreationProvider = StateNotifierProvider.autoDispose<
    SajuCardNotifier, AsyncValue<SajuCard?>>((ref) {
  return SajuCardNotifier(apiClient: ref.watch(apiClientProvider));
});

class SajuCardNotifier extends StateNotifier<AsyncValue<SajuCard?>> {
  final ApiClient _apiClient;

  SajuCardNotifier({required ApiClient apiClient})
      : _apiClient = apiClient,
        super(const AsyncValue.data(null));

  Future<void> createCard(BirthInput input) async {
    state = const AsyncValue.loading();
    try {
      final card = await _apiClient.createSajuCard(input);
      state = AsyncValue.data(card);
    } catch (e, st) {
      state = AsyncValue.error(e, st);
    }
  }
}
