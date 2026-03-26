import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:firebase_core/firebase_core.dart';
import 'package:kakao_flutter_sdk_user/kakao_flutter_sdk_user.dart' as kakao;

import 'core/theme/app_theme.dart';
import 'core/router/app_router.dart';
import 'features/payment/payment_service.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // Firebase 초기화
  await Firebase.initializeApp();

  // Kakao SDK 초기화
  kakao.KakaoSdk.init(
    nativeAppKey: const String.fromEnvironment(
      'KAKAO_NATIVE_APP_KEY',
      defaultValue: '',
    ),
  );

  // Lock to portrait orientation
  await SystemChrome.setPreferredOrientations([
    DeviceOrientation.portraitUp,
    DeviceOrientation.portraitDown,
  ]);

  // Status bar style
  SystemChrome.setSystemUIOverlayStyle(const SystemUiOverlayStyle(
    statusBarColor: Colors.transparent,
    statusBarIconBrightness: Brightness.dark,
    statusBarBrightness: Brightness.light,
  ));

  // RevenueCat 초기화
  final paymentService = PaymentService();
  await paymentService.initialize();

  runApp(ProviderScope(
    overrides: [
      paymentServiceProvider.overrideWithValue(paymentService),
    ],
    child: const SajuApp(),
  ));
}

class SajuApp extends ConsumerWidget {
  const SajuApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final router = ref.watch(routerProvider);

    return MaterialApp.router(
      title: '사주',
      debugShowCheckedModeBanner: false,
      theme: AppTheme.light,
      // darkTheme: AppTheme.dark, // v1.1
      routerConfig: router,
      builder: (context, child) {
        // Respect system font scale with cap at 1.5x
        final mediaQuery = MediaQuery.of(context);
        final clampedTextScale = mediaQuery.textScaler.clamp(
          minScaleFactor: 1.0,
          maxScaleFactor: 1.5,
        );

        return MediaQuery(
          data: mediaQuery.copyWith(textScaler: clampedTextScale),
          child: child ?? const SizedBox.shrink(),
        );
      },
    );
  }
}
