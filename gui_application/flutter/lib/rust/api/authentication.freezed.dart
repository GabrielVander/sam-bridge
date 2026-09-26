// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'authentication.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$LoginOutcomeDto {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LoginOutcomeDto);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'LoginOutcomeDto()';
}


}

/// @nodoc
class $LoginOutcomeDtoCopyWith<$Res>  {
$LoginOutcomeDtoCopyWith(LoginOutcomeDto _, $Res Function(LoginOutcomeDto) __);
}


/// Adds pattern-matching-related methods to [LoginOutcomeDto].
extension LoginOutcomeDtoPatterns on LoginOutcomeDto {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( LoginOutcomeDto_Successful value)?  successful,TResult Function( LoginOutcomeDto_InvalidEmailOrPassword value)?  invalidEmailOrPassword,TResult Function( LoginOutcomeDto_Failure value)?  failure,required TResult orElse(),}){
final _that = this;
switch (_that) {
case LoginOutcomeDto_Successful() when successful != null:
return successful(_that);case LoginOutcomeDto_InvalidEmailOrPassword() when invalidEmailOrPassword != null:
return invalidEmailOrPassword(_that);case LoginOutcomeDto_Failure() when failure != null:
return failure(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( LoginOutcomeDto_Successful value)  successful,required TResult Function( LoginOutcomeDto_InvalidEmailOrPassword value)  invalidEmailOrPassword,required TResult Function( LoginOutcomeDto_Failure value)  failure,}){
final _that = this;
switch (_that) {
case LoginOutcomeDto_Successful():
return successful(_that);case LoginOutcomeDto_InvalidEmailOrPassword():
return invalidEmailOrPassword(_that);case LoginOutcomeDto_Failure():
return failure(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( LoginOutcomeDto_Successful value)?  successful,TResult? Function( LoginOutcomeDto_InvalidEmailOrPassword value)?  invalidEmailOrPassword,TResult? Function( LoginOutcomeDto_Failure value)?  failure,}){
final _that = this;
switch (_that) {
case LoginOutcomeDto_Successful() when successful != null:
return successful(_that);case LoginOutcomeDto_InvalidEmailOrPassword() when invalidEmailOrPassword != null:
return invalidEmailOrPassword(_that);case LoginOutcomeDto_Failure() when failure != null:
return failure(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  successful,TResult Function()?  invalidEmailOrPassword,TResult Function( ErrorReportDto report)?  failure,required TResult orElse(),}) {final _that = this;
switch (_that) {
case LoginOutcomeDto_Successful() when successful != null:
return successful();case LoginOutcomeDto_InvalidEmailOrPassword() when invalidEmailOrPassword != null:
return invalidEmailOrPassword();case LoginOutcomeDto_Failure() when failure != null:
return failure(_that.report);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  successful,required TResult Function()  invalidEmailOrPassword,required TResult Function( ErrorReportDto report)  failure,}) {final _that = this;
switch (_that) {
case LoginOutcomeDto_Successful():
return successful();case LoginOutcomeDto_InvalidEmailOrPassword():
return invalidEmailOrPassword();case LoginOutcomeDto_Failure():
return failure(_that.report);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  successful,TResult? Function()?  invalidEmailOrPassword,TResult? Function( ErrorReportDto report)?  failure,}) {final _that = this;
switch (_that) {
case LoginOutcomeDto_Successful() when successful != null:
return successful();case LoginOutcomeDto_InvalidEmailOrPassword() when invalidEmailOrPassword != null:
return invalidEmailOrPassword();case LoginOutcomeDto_Failure() when failure != null:
return failure(_that.report);case _:
  return null;

}
}

}

/// @nodoc


class LoginOutcomeDto_Successful extends LoginOutcomeDto {
  const LoginOutcomeDto_Successful(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LoginOutcomeDto_Successful);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'LoginOutcomeDto.successful()';
}


}




/// @nodoc


class LoginOutcomeDto_InvalidEmailOrPassword extends LoginOutcomeDto {
  const LoginOutcomeDto_InvalidEmailOrPassword(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LoginOutcomeDto_InvalidEmailOrPassword);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'LoginOutcomeDto.invalidEmailOrPassword()';
}


}




/// @nodoc


class LoginOutcomeDto_Failure extends LoginOutcomeDto {
  const LoginOutcomeDto_Failure({required this.report}): super._();
  

 final  ErrorReportDto report;

/// Create a copy of LoginOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$LoginOutcomeDto_FailureCopyWith<LoginOutcomeDto_Failure> get copyWith => _$LoginOutcomeDto_FailureCopyWithImpl<LoginOutcomeDto_Failure>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LoginOutcomeDto_Failure&&(identical(other.report, report) || other.report == report));
}


@override
int get hashCode => Object.hash(runtimeType,report);

@override
String toString() {
  return 'LoginOutcomeDto.failure(report: $report)';
}


}

/// @nodoc
abstract mixin class $LoginOutcomeDto_FailureCopyWith<$Res> implements $LoginOutcomeDtoCopyWith<$Res> {
  factory $LoginOutcomeDto_FailureCopyWith(LoginOutcomeDto_Failure value, $Res Function(LoginOutcomeDto_Failure) _then) = _$LoginOutcomeDto_FailureCopyWithImpl;
@useResult
$Res call({
 ErrorReportDto report
});




}
/// @nodoc
class _$LoginOutcomeDto_FailureCopyWithImpl<$Res>
    implements $LoginOutcomeDto_FailureCopyWith<$Res> {
  _$LoginOutcomeDto_FailureCopyWithImpl(this._self, this._then);

  final LoginOutcomeDto_Failure _self;
  final $Res Function(LoginOutcomeDto_Failure) _then;

/// Create a copy of LoginOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? report = null,}) {
  return _then(LoginOutcomeDto_Failure(
report: null == report ? _self.report : report // ignore: cast_nullable_to_non_nullable
as ErrorReportDto,
  ));
}


}

/// @nodoc
mixin _$LogoutOutcomeDto {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LogoutOutcomeDto);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'LogoutOutcomeDto()';
}


}

/// @nodoc
class $LogoutOutcomeDtoCopyWith<$Res>  {
$LogoutOutcomeDtoCopyWith(LogoutOutcomeDto _, $Res Function(LogoutOutcomeDto) __);
}


/// Adds pattern-matching-related methods to [LogoutOutcomeDto].
extension LogoutOutcomeDtoPatterns on LogoutOutcomeDto {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( LogoutOutcomeDto_Successful value)?  successful,TResult Function( LogoutOutcomeDto_Failure value)?  failure,required TResult orElse(),}){
final _that = this;
switch (_that) {
case LogoutOutcomeDto_Successful() when successful != null:
return successful(_that);case LogoutOutcomeDto_Failure() when failure != null:
return failure(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( LogoutOutcomeDto_Successful value)  successful,required TResult Function( LogoutOutcomeDto_Failure value)  failure,}){
final _that = this;
switch (_that) {
case LogoutOutcomeDto_Successful():
return successful(_that);case LogoutOutcomeDto_Failure():
return failure(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( LogoutOutcomeDto_Successful value)?  successful,TResult? Function( LogoutOutcomeDto_Failure value)?  failure,}){
final _that = this;
switch (_that) {
case LogoutOutcomeDto_Successful() when successful != null:
return successful(_that);case LogoutOutcomeDto_Failure() when failure != null:
return failure(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  successful,TResult Function( ErrorReportDto report)?  failure,required TResult orElse(),}) {final _that = this;
switch (_that) {
case LogoutOutcomeDto_Successful() when successful != null:
return successful();case LogoutOutcomeDto_Failure() when failure != null:
return failure(_that.report);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  successful,required TResult Function( ErrorReportDto report)  failure,}) {final _that = this;
switch (_that) {
case LogoutOutcomeDto_Successful():
return successful();case LogoutOutcomeDto_Failure():
return failure(_that.report);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  successful,TResult? Function( ErrorReportDto report)?  failure,}) {final _that = this;
switch (_that) {
case LogoutOutcomeDto_Successful() when successful != null:
return successful();case LogoutOutcomeDto_Failure() when failure != null:
return failure(_that.report);case _:
  return null;

}
}

}

/// @nodoc


class LogoutOutcomeDto_Successful extends LogoutOutcomeDto {
  const LogoutOutcomeDto_Successful(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LogoutOutcomeDto_Successful);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'LogoutOutcomeDto.successful()';
}


}




/// @nodoc


class LogoutOutcomeDto_Failure extends LogoutOutcomeDto {
  const LogoutOutcomeDto_Failure({required this.report}): super._();
  

 final  ErrorReportDto report;

/// Create a copy of LogoutOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$LogoutOutcomeDto_FailureCopyWith<LogoutOutcomeDto_Failure> get copyWith => _$LogoutOutcomeDto_FailureCopyWithImpl<LogoutOutcomeDto_Failure>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LogoutOutcomeDto_Failure&&(identical(other.report, report) || other.report == report));
}


@override
int get hashCode => Object.hash(runtimeType,report);

@override
String toString() {
  return 'LogoutOutcomeDto.failure(report: $report)';
}


}

/// @nodoc
abstract mixin class $LogoutOutcomeDto_FailureCopyWith<$Res> implements $LogoutOutcomeDtoCopyWith<$Res> {
  factory $LogoutOutcomeDto_FailureCopyWith(LogoutOutcomeDto_Failure value, $Res Function(LogoutOutcomeDto_Failure) _then) = _$LogoutOutcomeDto_FailureCopyWithImpl;
@useResult
$Res call({
 ErrorReportDto report
});




}
/// @nodoc
class _$LogoutOutcomeDto_FailureCopyWithImpl<$Res>
    implements $LogoutOutcomeDto_FailureCopyWith<$Res> {
  _$LogoutOutcomeDto_FailureCopyWithImpl(this._self, this._then);

  final LogoutOutcomeDto_Failure _self;
  final $Res Function(LogoutOutcomeDto_Failure) _then;

/// Create a copy of LogoutOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? report = null,}) {
  return _then(LogoutOutcomeDto_Failure(
report: null == report ? _self.report : report // ignore: cast_nullable_to_non_nullable
as ErrorReportDto,
  ));
}


}

/// @nodoc
mixin _$RestoreSessionOutcomeDto {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RestoreSessionOutcomeDto);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'RestoreSessionOutcomeDto()';
}


}

/// @nodoc
class $RestoreSessionOutcomeDtoCopyWith<$Res>  {
$RestoreSessionOutcomeDtoCopyWith(RestoreSessionOutcomeDto _, $Res Function(RestoreSessionOutcomeDto) __);
}


/// Adds pattern-matching-related methods to [RestoreSessionOutcomeDto].
extension RestoreSessionOutcomeDtoPatterns on RestoreSessionOutcomeDto {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( RestoreSessionOutcomeDto_Restored value)?  restored,TResult Function( RestoreSessionOutcomeDto_NotAvailable value)?  notAvailable,TResult Function( RestoreSessionOutcomeDto_Failure value)?  failure,required TResult orElse(),}){
final _that = this;
switch (_that) {
case RestoreSessionOutcomeDto_Restored() when restored != null:
return restored(_that);case RestoreSessionOutcomeDto_NotAvailable() when notAvailable != null:
return notAvailable(_that);case RestoreSessionOutcomeDto_Failure() when failure != null:
return failure(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( RestoreSessionOutcomeDto_Restored value)  restored,required TResult Function( RestoreSessionOutcomeDto_NotAvailable value)  notAvailable,required TResult Function( RestoreSessionOutcomeDto_Failure value)  failure,}){
final _that = this;
switch (_that) {
case RestoreSessionOutcomeDto_Restored():
return restored(_that);case RestoreSessionOutcomeDto_NotAvailable():
return notAvailable(_that);case RestoreSessionOutcomeDto_Failure():
return failure(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( RestoreSessionOutcomeDto_Restored value)?  restored,TResult? Function( RestoreSessionOutcomeDto_NotAvailable value)?  notAvailable,TResult? Function( RestoreSessionOutcomeDto_Failure value)?  failure,}){
final _that = this;
switch (_that) {
case RestoreSessionOutcomeDto_Restored() when restored != null:
return restored(_that);case RestoreSessionOutcomeDto_NotAvailable() when notAvailable != null:
return notAvailable(_that);case RestoreSessionOutcomeDto_Failure() when failure != null:
return failure(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  restored,TResult Function()?  notAvailable,TResult Function( ErrorReportDto report)?  failure,required TResult orElse(),}) {final _that = this;
switch (_that) {
case RestoreSessionOutcomeDto_Restored() when restored != null:
return restored();case RestoreSessionOutcomeDto_NotAvailable() when notAvailable != null:
return notAvailable();case RestoreSessionOutcomeDto_Failure() when failure != null:
return failure(_that.report);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  restored,required TResult Function()  notAvailable,required TResult Function( ErrorReportDto report)  failure,}) {final _that = this;
switch (_that) {
case RestoreSessionOutcomeDto_Restored():
return restored();case RestoreSessionOutcomeDto_NotAvailable():
return notAvailable();case RestoreSessionOutcomeDto_Failure():
return failure(_that.report);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  restored,TResult? Function()?  notAvailable,TResult? Function( ErrorReportDto report)?  failure,}) {final _that = this;
switch (_that) {
case RestoreSessionOutcomeDto_Restored() when restored != null:
return restored();case RestoreSessionOutcomeDto_NotAvailable() when notAvailable != null:
return notAvailable();case RestoreSessionOutcomeDto_Failure() when failure != null:
return failure(_that.report);case _:
  return null;

}
}

}

/// @nodoc


class RestoreSessionOutcomeDto_Restored extends RestoreSessionOutcomeDto {
  const RestoreSessionOutcomeDto_Restored(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RestoreSessionOutcomeDto_Restored);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'RestoreSessionOutcomeDto.restored()';
}


}




/// @nodoc


class RestoreSessionOutcomeDto_NotAvailable extends RestoreSessionOutcomeDto {
  const RestoreSessionOutcomeDto_NotAvailable(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RestoreSessionOutcomeDto_NotAvailable);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'RestoreSessionOutcomeDto.notAvailable()';
}


}




/// @nodoc


class RestoreSessionOutcomeDto_Failure extends RestoreSessionOutcomeDto {
  const RestoreSessionOutcomeDto_Failure({required this.report}): super._();
  

 final  ErrorReportDto report;

/// Create a copy of RestoreSessionOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RestoreSessionOutcomeDto_FailureCopyWith<RestoreSessionOutcomeDto_Failure> get copyWith => _$RestoreSessionOutcomeDto_FailureCopyWithImpl<RestoreSessionOutcomeDto_Failure>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RestoreSessionOutcomeDto_Failure&&(identical(other.report, report) || other.report == report));
}


@override
int get hashCode => Object.hash(runtimeType,report);

@override
String toString() {
  return 'RestoreSessionOutcomeDto.failure(report: $report)';
}


}

/// @nodoc
abstract mixin class $RestoreSessionOutcomeDto_FailureCopyWith<$Res> implements $RestoreSessionOutcomeDtoCopyWith<$Res> {
  factory $RestoreSessionOutcomeDto_FailureCopyWith(RestoreSessionOutcomeDto_Failure value, $Res Function(RestoreSessionOutcomeDto_Failure) _then) = _$RestoreSessionOutcomeDto_FailureCopyWithImpl;
@useResult
$Res call({
 ErrorReportDto report
});




}
/// @nodoc
class _$RestoreSessionOutcomeDto_FailureCopyWithImpl<$Res>
    implements $RestoreSessionOutcomeDto_FailureCopyWith<$Res> {
  _$RestoreSessionOutcomeDto_FailureCopyWithImpl(this._self, this._then);

  final RestoreSessionOutcomeDto_Failure _self;
  final $Res Function(RestoreSessionOutcomeDto_Failure) _then;

/// Create a copy of RestoreSessionOutcomeDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? report = null,}) {
  return _then(RestoreSessionOutcomeDto_Failure(
report: null == report ? _self.report : report // ignore: cast_nullable_to_non_nullable
as ErrorReportDto,
  ));
}


}

// dart format on
