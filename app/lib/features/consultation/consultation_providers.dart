import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/api/api_client.dart';
import '../../shared/models/models.dart';

/// Active consultation state
final consultationProvider = StateNotifierProvider.autoDispose<
    ConsultationNotifier, AsyncValue<Consultation?>>((ref) {
  return ConsultationNotifier(apiClient: ref.watch(apiClientProvider));
});

class ConsultationNotifier extends StateNotifier<AsyncValue<Consultation?>> {
  final ApiClient _apiClient;

  ConsultationNotifier({required ApiClient apiClient})
      : _apiClient = apiClient,
        super(const AsyncValue.data(null));

  Future<void> startConsultation({
    required BirthInput birthInput,
    required String receiptId,
  }) async {
    state = const AsyncValue.loading();
    try {
      final consultation = await _apiClient.startConsultation({
        'birth_input': birthInput.toJson(),
        'receipt_id': receiptId,
      });
      state = AsyncValue.data(consultation);
    } catch (e, st) {
      state = AsyncValue.error(e, st);
    }
  }
}
