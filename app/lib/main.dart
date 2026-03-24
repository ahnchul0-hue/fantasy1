import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'core/theme/app_theme.dart';
import 'core/router/app_router.dart';
import 'features/payment/payment_service.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

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

  // TODO: Initialize Firebase
  // await Firebase.initializeApp();

  // RevenueCat 초기화
  await PaymentService().initialize();

  runApp(const ProviderScope(child: SajuApp()));
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
