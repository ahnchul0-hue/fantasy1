import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/api/api_client.dart';
import '../../shared/models/payment.dart';
import 'payment_service.dart';

/// PaymentService 싱글턴
final paymentServiceProvider = Provider<PaymentService>((ref) {
  return PaymentService();
});

/// 구매 + 서버 검증 플로우 상태
enum PaymentFlowStatus { idle, purchasing, verifying, success, error }

class PaymentFlowState {
  final PaymentFlowStatus status;
  final String? orderId;
  final String? error;

  const PaymentFlowState({
    this.status = PaymentFlowStatus.idle,
    this.orderId,
    this.error,
  });

  PaymentFlowState copyWith({
    PaymentFlowStatus? status,
    String? orderId,
    String? error,
  }) =>
      PaymentFlowState(
        status: status ?? this.status,
        orderId: orderId ?? this.orderId,
        error: error ?? this.error,
      );
}

/// 결제 플로우 StateNotifier
class PaymentFlowNotifier extends StateNotifier<PaymentFlowState> {
  final PaymentService _paymentService;
  final ApiClient _apiClient;

  PaymentFlowNotifier({
    required PaymentService paymentService,
    required ApiClient apiClient,
  })  : _paymentService = paymentService,
        _apiClient = apiClient,
        super(const PaymentFlowState());

  /// 전체 결제 플로우: IAP → 서버 검증 → 상담 시작
  Future<String?> purchaseAndVerify(PaymentProduct product) async {
    state = state.copyWith(status: PaymentFlowStatus.purchasing, error: null);

    // 1. RevenueCat IAP
    final purchaseResult = await _paymentService.purchase(product);

    if (!purchaseResult.success) {
      if (purchaseResult.cancelled) {
        state = state.copyWith(status: PaymentFlowStatus.idle);
        return null;
      }
      state = state.copyWith(
        status: PaymentFlowStatus.error,
        error: purchaseResult.error ?? '결제에 실패했습니다',
      );
      return null;
    }

    // 2. 서버 검증
    state = state.copyWith(status: PaymentFlowStatus.verifying);

    try {
      final verifyResponse = await _apiClient.verifyPayment(
        PaymentVerificationRequest(
          productId: product.id,
          platform: purchaseResult.platform!,
          receiptId: purchaseResult.receiptId!,
          receiptData: purchaseResult.receiptId!,
        ),
      );

      if (!verifyResponse.verified) {
        state = state.copyWith(
          status: PaymentFlowStatus.error,
          error: '결제 검증에 실패했습니다',
        );
        return null;
      }

      state = state.copyWith(
        status: PaymentFlowStatus.success,
        orderId: verifyResponse.orderId,
      );
      return verifyResponse.orderId;
    } catch (e) {
      state = state.copyWith(
        status: PaymentFlowStatus.error,
        error: '결제 검증 중 오류가 발생했습니다',
      );
      return null;
    }
  }

  void reset() {
    state = const PaymentFlowState();
  }
}

/// 결제 플로우 provider
final paymentFlowProvider =
    StateNotifierProvider.autoDispose<PaymentFlowNotifier, PaymentFlowState>(
        (ref) {
  return PaymentFlowNotifier(
    paymentService: ref.watch(paymentServiceProvider),
    apiClient: ref.watch(apiClientProvider),
  );
});
