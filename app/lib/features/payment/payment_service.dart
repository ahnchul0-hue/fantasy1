import 'dart:io';
import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:purchases_flutter/purchases_flutter.dart';
import '../../shared/models/payment.dart';

/// RevenueCat 래퍼 — IAP 초기화, 구매, 영수증 추출
class PaymentService {
  static const _iosApiKey = String.fromEnvironment(
    'REVENUECAT_IOS_KEY',
    defaultValue: '',
  );
  static const _androidApiKey = String.fromEnvironment(
    'REVENUECAT_ANDROID_KEY',
    defaultValue: '',
  );

  bool _initialized = false;

  /// RevenueCat SDK 초기화 — main.dart에서 앱 시작 시 호출
  Future<void> initialize({String? userId}) async {
    if (_initialized) return;

    final apiKey = Platform.isIOS ? _iosApiKey : _androidApiKey;
    if (apiKey.isEmpty) {
      debugPrint('[PaymentService] WARNING: RevenueCat API key is empty. '
          'Set REVENUECAT_IOS_KEY or REVENUECAT_ANDROID_KEY via --dart-define.');
      return;
    }

    final config = PurchasesConfiguration(apiKey);
    if (userId != null) {
      config..appUserID = userId;
    }
    await Purchases.configure(config);
    _initialized = true;
  }

  /// RevenueCat 사용자 ID 연결 (로그인 시)
  Future<void> login(String userId) async {
    if (!_initialized) return;
    await Purchases.logIn(userId);
  }

  /// 로그아웃 시 RevenueCat 익명 전환
  Future<void> logout() async {
    if (!_initialized) return;
    await Purchases.logOut();
  }

  /// 상품 정보 조회 (가격 등 스토어 기준값)
  Future<StoreProduct?> getProduct(PaymentProduct product) async {
    if (!_initialized) return null;
    final products = await Purchases.getProducts(
      [product.id],
      productCategory: ProductCategory.nonSubscription,
    );
    return products.isNotEmpty ? products.first : null;
  }

  /// 구매 실행 → 성공 시 영수증 정보 반환
  Future<PurchaseResult> purchase(PaymentProduct product) async {
    if (!_initialized) {
      return const PurchaseResult(
        success: false,
        error: '결제 서비스가 초기화되지 않았습니다',
      );
    }

    try {
      final storeProduct = await getProduct(product);
      if (storeProduct == null) {
        return const PurchaseResult(
          success: false,
          error: '상품 정보를 불러올 수 없습니다',
        );
      }

      final result = await Purchases.purchaseStoreProduct(storeProduct);
      final transaction = result.nonSubscriptionTransactions.lastOrNull;

      if (transaction == null) {
        return const PurchaseResult(
          success: false,
          error: '거래 정보를 확인할 수 없습니다',
        );
      }

      return PurchaseResult(
        success: true,
        receiptId: transaction.transactionIdentifier,
        platform: Platform.isIOS ? 'ios' : 'android',
      );
    } on PlatformException catch (e) {
      if (e.code == '1' /* userCancelled */) {
        return const PurchaseResult(
          success: false,
          cancelled: true,
        );
      }
      return PurchaseResult(
        success: false,
        error: '결제 처리 중 오류가 발생했습니다',
      );
    } catch (e) {
      return PurchaseResult(
        success: false,
        error: '결제 처리 중 오류가 발생했습니다',
      );
    }
  }
}

/// 구매 결과
class PurchaseResult {
  final bool success;
  final bool cancelled;
  final String? receiptId;
  final String? platform;
  final String? error;

  const PurchaseResult({
    required this.success,
    this.cancelled = false,
    this.receiptId,
    this.platform,
    this.error,
  });
}
