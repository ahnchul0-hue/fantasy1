import 'package:freezed_annotation/freezed_annotation.dart';

part 'payment.freezed.dart';
part 'payment.g.dart';

/// 결제 상품 정의
class PaymentProduct {
  final String id;
  final String name;
  final int priceKrw;

  const PaymentProduct({
    required this.id,
    required this.name,
    required this.priceKrw,
  });

  /// 사주 상담 상품
  static const sajuConsultation = PaymentProduct(
    id: 'saju_consultation_15000',
    name: 'AI 사주 상담',
    priceKrw: 15000,
  );

  /// 궁합 상담 상품
  static const compatibilityConsultation = PaymentProduct(
    id: 'compatibility_consultation_12000',
    name: '궁합 상세 분석',
    priceKrw: 12000,
  );
}

/// POST /v1/payment/verify 요청
@freezed
class PaymentVerificationRequest with _$PaymentVerificationRequest {
  const factory PaymentVerificationRequest({
    required String productId,
    required String platform,
    required String receiptId,
    required String receiptData,
  }) = _PaymentVerificationRequest;

  factory PaymentVerificationRequest.fromJson(Map<String, dynamic> json) =>
      _$PaymentVerificationRequestFromJson(json);
}

/// POST /v1/payment/verify 응답
@freezed
class PaymentVerificationResponse with _$PaymentVerificationResponse {
  const factory PaymentVerificationResponse({
    required bool verified,
    String? orderId,
  }) = _PaymentVerificationResponse;

  factory PaymentVerificationResponse.fromJson(Map<String, dynamic> json) =>
      _$PaymentVerificationResponseFromJson(json);
}
