import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:retrofit/retrofit.dart';
import 'package:uuid/uuid.dart';

import '../../shared/models/models.dart';
import '../auth/auth_service.dart';

part 'api_client.g.dart';

const _baseUrl = String.fromEnvironment(
  'API_BASE_URL',
  defaultValue: 'http://localhost:8080/v1',
);

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

  @GET('/saju/card/{id}')
  Future<SajuCard> getSajuCard(@Path('id') String id);

  // --- Consultation ---

  @POST('/saju/consultation')
  Future<Consultation> startConsultation(@Body() Map<String, dynamic> body);

  @GET('/saju/consultation/{id}')
  Future<Consultation> getConsultation(@Path('id') String id);

  @GET('/saju/consultation/{id}/messages')
  Future<List<ChatMessage>> getConsultationMessages(@Path('id') String id);

  @GET('/consultations')
  Future<List<Consultation>> getConsultations();

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

/// Interceptor that attaches JWT and handles 401 refresh with mutex
class AuthInterceptor extends Interceptor {
  final AuthService _authService;
  final Dio _dio;
  bool _isRefreshing = false;
  final _pendingRequests =
      <({RequestOptions options, ErrorInterceptorHandler handler})>[];

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
    if (err.response?.statusCode != 401) {
      return handler.next(err);
    }

    if (_isRefreshing) {
      _pendingRequests.add((options: err.requestOptions, handler: handler));
      return;
    }

    _isRefreshing = true;
    try {
      final refreshToken = await _authService.getRefreshToken();
      if (refreshToken == null) {
        await _authService.clearTokens();
        return handler.next(err);
      }

      // Use a SEPARATE dio instance for refresh to avoid recursion
      final refreshDio = Dio(BaseOptions(baseUrl: _dio.options.baseUrl));
      final response = await refreshDio.post('/auth/refresh', data: {
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
      opts.headers['Authorization'] = 'Bearer ${authResponse.accessToken}';
      final retryResponse = await _dio.fetch(opts);
      handler.resolve(retryResponse);

      // Retry pending requests
      for (final pending in _pendingRequests) {
        pending.options.headers['Authorization'] =
            'Bearer ${authResponse.accessToken}';
        try {
          final r = await _dio.fetch(pending.options);
          pending.handler.resolve(r);
        } catch (e) {
          if (e is DioException) {
            pending.handler.reject(e);
          }
        }
      }
    } catch (_) {
      await _authService.clearTokens();
      handler.next(err);
      for (final pending in _pendingRequests) {
        pending.handler.next(err);
      }
    } finally {
      _isRefreshing = false;
      _pendingRequests.clear();
    }
  }
}

/// Interceptor that attaches X-Device-ID header
class DeviceIdInterceptor extends Interceptor {
  String? _deviceId;

  @override
  void onRequest(
      RequestOptions options, RequestInterceptorHandler handler) async {
    _deviceId ??= await _getOrCreateDeviceId();
    options.headers['X-Device-ID'] = _deviceId;
    handler.next(options);
  }

  Future<String> _getOrCreateDeviceId() async {
    const storage = FlutterSecureStorage();
    var id = await storage.read(key: 'device_id');
    if (id == null) {
      id = const Uuid().v4();
      await storage.write(key: 'device_id', value: id);
    }
    return id;
  }
}

/// Configured Dio instance
Dio _createDio(AuthService authService) {
  final dio = Dio(BaseOptions(
    baseUrl: _baseUrl,
    connectTimeout: const Duration(seconds: 10),
    receiveTimeout: const Duration(seconds: 30),
    headers: {
      'Content-Type': 'application/json',
      'Accept': 'application/json',
    },
  ));
  dio.interceptors.add(DeviceIdInterceptor());
  dio.interceptors.add(AuthInterceptor(authService, dio));
  return dio;
}

/// Riverpod provider for ApiClient
final apiClientProvider = Provider<ApiClient>((ref) {
  final authService = ref.watch(authServiceProvider);
  final dio = _createDio(authService);
  return ApiClient(dio);
});
