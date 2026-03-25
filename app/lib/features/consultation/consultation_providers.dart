import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/api/api_client.dart';
import '../../shared/models/models.dart';

/// Active consultation state (for viewing an existing consultation)
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
    required String orderId,
  }) async {
    state = const AsyncValue.loading();
    try {
      final consultation = await _apiClient.startConsultation({
        'birth_input': birthInput.toJson(),
        'order_id': orderId,
      });
      state = AsyncValue.data(consultation);
    } catch (e, st) {
      state = AsyncValue.error(e, st);
    }
  }
}

/// Provider to start a new consultation (used by payment flow)
final consultationCreationProvider = StateNotifierProvider.autoDispose<
    ConsultationCreationNotifier, AsyncValue<Consultation?>>((ref) {
  return ConsultationCreationNotifier(apiClient: ref.watch(apiClientProvider));
});

class ConsultationCreationNotifier extends StateNotifier<AsyncValue<Consultation?>> {
  final ApiClient _apiClient;

  ConsultationCreationNotifier({required ApiClient apiClient})
      : _apiClient = apiClient,
        super(const AsyncValue.data(null));

  /// 결제 완료 후 상담 생성. 성공 시 consultation ID 반환.
  Future<String?> startConsultation({
    required BirthInput birthInput,
    required String orderId,
  }) async {
    state = const AsyncValue.loading();
    try {
      final consultation = await _apiClient.startConsultation({
        'birth_input': birthInput.toJson(),
        'order_id': orderId,
      });
      state = AsyncValue.data(consultation);
      return consultation.id;
    } catch (e, st) {
      state = AsyncValue.error(e, st);
      return null;
    }
  }
}

/// Consultation status polling — 서버에서 현재 상담 전체 데이터 조회
final consultationStatusProvider = FutureProvider.autoDispose.family<Consultation, String>((ref, id) async {
  final apiClient = ref.watch(apiClientProvider);
  return await apiClient.getConsultation(id);
});
