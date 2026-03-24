import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:retrofit/retrofit.dart';

import '../../shared/models/models.dart';
import '../auth/auth_service.dart';

part 'api_client.g.dart';

@RestApi()
abstract class ApiClient {
  factory ApiClient(Dio dio, {String baseUrl}) = _ApiClient;

  // --- Auth ---

  @POST('/auth/login')
  Future<AuthResponse> login(@Body() Map<String, dynamic> body);

  @POST('/auth/refresh')
  Future<AuthResponse> refreshToken(@Body() Map<String, dynamic> body);

  @DELETE('/auth/delete-account')
  Future<void> deleteAccount();

  // --- Saju Card ---

  @POST('/saju/card')
  Future<SajuCard> createSajuCard(@Body() BirthInput input);

  // --- Consultation ---

  @POST('/saju/consultation')
  Future<Consultation> startConsultation(@Body() Map<String, dynamic> body);

  @POST('/saju/consultation/{id}/chat')
  Future<ChatMessage> sendChatMessage(
    @Path('id') String consultationId,
    @Body() Map<String, dynamic> body,
  );

  // --- Compatibility ---

  @POST('/saju/compatibility')
  Future<CompatibilityPreview> getCompatibility(
      @Body() Map<String, dynamic> body);

  // --- Fortune ---

  @GET('/fortune/daily')
  Future<DailyFortune> getDailyFortune();

  // --- Profile ---

  @GET('/profile')
  Future<SajuProfile> getProfile();

  // --- Payment ---

  @POST('/payment/verify')
  Future<PaymentVerificationResponse> verifyPayment(
      @Body() PaymentVerificationRequest request);
}

/// Interceptor that attaches JWT and handles 401 refresh
class AuthInterceptor extends Interceptor {
  final AuthService _authService;
  final Dio _dio;

  AuthInterceptor(this._authService, this._dio);

  @override
  void onRequest(
    RequestOptions options,
    RequestInterceptorHandler handler,
  ) async {
    final token = await _authService.getAccessToken();
    if (token != null) {
      options.headers['Authorization'] = 'Bearer $token';
    }
    handler.next(options);
  }

  @override
  void onError(DioException err, ErrorInterceptorHandler handler) async {
    if (err.response?.statusCode == 401) {
      try {
        final refreshToken = await _authService.getRefreshToken();
        if (refreshToken != null) {
          final response = await _dio.post('/auth/refresh', data: {
            'refresh_token': refreshToken,
          });
          final authResponse =
              AuthResponse.fromJson(response.data as Map<String, dynamic>);
          await _authService.saveTokens(
            accessToken: authResponse.accessToken,
            refreshToken: authResponse.refreshToken,
          );

          // Retry original request
          final opts = err.requestOptions;
          opts.headers['Authorization'] =
              'Bearer ${authResponse.accessToken}';
          final retryResponse = await _dio.fetch(opts);
          return handler.resolve(retryResponse);
        }
      } catch (_) {
        await _authService.clearTokens();
      }
    }
    handler.next(err);
  }
}

/// Configured Dio instance
Dio _createDio(AuthService authService) {
  final dio = Dio(BaseOptions(
    baseUrl: 'https://api.saju.app/v1',
    connectTimeout: const Duration(seconds: 10),
    receiveTimeout: const Duration(seconds: 30),
    headers: {
      'Content-Type': 'application/json',
      'Accept': 'application/json',
    },
  ));
  dio.interceptors.add(AuthInterceptor(authService, dio));
  return dio;
}

/// Riverpod provider for ApiClient
final apiClientProvider = Provider<ApiClient>((ref) {
  final authService = ref.watch(authServiceProvider);
  final dio = _createDio(authService);
  return ApiClient(dio);
});
