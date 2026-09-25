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
mixin _$LoginOutcome {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LoginOutcome);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'LoginOutcome()';
}


}

/// @nodoc
class $LoginOutcomeCopyWith<$Res>  {
$LoginOutcomeCopyWith(LoginOutcome _, $Res Function(LoginOutcome) __);
}


/// Adds pattern-matching-related methods to [LoginOutcome].
extension LoginOutcomePatterns on LoginOutcome {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( LoginOutcome_Successful value)?  successful,TResult Function( LoginOutcome_InvalidEmailOrPassword value)?  invalidEmailOrPassword,TResult Function( LoginOutcome_Failure value)?  failure,required TResult orElse(),}){
final _that = this;
switch (_that) {
case LoginOutcome_Successful() when successful != null:
return successful(_that);case LoginOutcome_InvalidEmailOrPassword() when invalidEmailOrPassword != null:
return invalidEmailOrPassword(_that);case LoginOutcome_Failure() when failure != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( LoginOutcome_Successful value)  successful,required TResult Function( LoginOutcome_InvalidEmailOrPassword value)  invalidEmailOrPassword,required TResult Function( LoginOutcome_Failure value)  failure,}){
final _that = this;
switch (_that) {
case LoginOutcome_Successful():
return successful(_that);case LoginOutcome_InvalidEmailOrPassword():
return invalidEmailOrPassword(_that);case LoginOutcome_Failure():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( LoginOutcome_Successful value)?  successful,TResult? Function( LoginOutcome_InvalidEmailOrPassword value)?  invalidEmailOrPassword,TResult? Function( LoginOutcome_Failure value)?  failure,}){
final _that = this;
switch (_that) {
case LoginOutcome_Successful() when successful != null:
return successful(_that);case LoginOutcome_InvalidEmailOrPassword() when invalidEmailOrPassword != null:
return invalidEmailOrPassword(_that);case LoginOutcome_Failure() when failure != null:
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
case LoginOutcome_Successful() when successful != null:
return successful();case LoginOutcome_InvalidEmailOrPassword() when invalidEmailOrPassword != null:
return invalidEmailOrPassword();case LoginOutcome_Failure() when failure != null:
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
case LoginOutcome_Successful():
return successful();case LoginOutcome_InvalidEmailOrPassword():
return invalidEmailOrPassword();case LoginOutcome_Failure():
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
case LoginOutcome_Successful() when successful != null:
return successful();case LoginOutcome_InvalidEmailOrPassword() when invalidEmailOrPassword != null:
return invalidEmailOrPassword();case LoginOutcome_Failure() when failure != null:
return failure(_that.report);case _:
  return null;

}
}

}

/// @nodoc


class LoginOutcome_Successful extends LoginOutcome {
  const LoginOutcome_Successful(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LoginOutcome_Successful);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'LoginOutcome.successful()';
}


}




/// @nodoc


class LoginOutcome_InvalidEmailOrPassword extends LoginOutcome {
  const LoginOutcome_InvalidEmailOrPassword(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LoginOutcome_InvalidEmailOrPassword);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'LoginOutcome.invalidEmailOrPassword()';
}


}




/// @nodoc


class LoginOutcome_Failure extends LoginOutcome {
  const LoginOutcome_Failure({required this.report}): super._();
  

 final  ErrorReportDto report;

/// Create a copy of LoginOutcome
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$LoginOutcome_FailureCopyWith<LoginOutcome_Failure> get copyWith => _$LoginOutcome_FailureCopyWithImpl<LoginOutcome_Failure>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LoginOutcome_Failure&&(identical(other.report, report) || other.report == report));
}


@override
int get hashCode => Object.hash(runtimeType,report);

@override
String toString() {
  return 'LoginOutcome.failure(report: $report)';
}


}

/// @nodoc
abstract mixin class $LoginOutcome_FailureCopyWith<$Res> implements $LoginOutcomeCopyWith<$Res> {
  factory $LoginOutcome_FailureCopyWith(LoginOutcome_Failure value, $Res Function(LoginOutcome_Failure) _then) = _$LoginOutcome_FailureCopyWithImpl;
@useResult
$Res call({
 ErrorReportDto report
});




}
/// @nodoc
class _$LoginOutcome_FailureCopyWithImpl<$Res>
    implements $LoginOutcome_FailureCopyWith<$Res> {
  _$LoginOutcome_FailureCopyWithImpl(this._self, this._then);

  final LoginOutcome_Failure _self;
  final $Res Function(LoginOutcome_Failure) _then;

/// Create a copy of LoginOutcome
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? report = null,}) {
  return _then(LoginOutcome_Failure(
report: null == report ? _self.report : report // ignore: cast_nullable_to_non_nullable
as ErrorReportDto,
  ));
}


}

/// @nodoc
mixin _$RestoreSessionOutcome {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RestoreSessionOutcome);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'RestoreSessionOutcome()';
}


}

/// @nodoc
class $RestoreSessionOutcomeCopyWith<$Res>  {
$RestoreSessionOutcomeCopyWith(RestoreSessionOutcome _, $Res Function(RestoreSessionOutcome) __);
}


/// Adds pattern-matching-related methods to [RestoreSessionOutcome].
extension RestoreSessionOutcomePatterns on RestoreSessionOutcome {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( RestoreSessionOutcome_Restored value)?  restored,TResult Function( RestoreSessionOutcome_NotAvailable value)?  notAvailable,TResult Function( RestoreSessionOutcome_Failure value)?  failure,required TResult orElse(),}){
final _that = this;
switch (_that) {
case RestoreSessionOutcome_Restored() when restored != null:
return restored(_that);case RestoreSessionOutcome_NotAvailable() when notAvailable != null:
return notAvailable(_that);case RestoreSessionOutcome_Failure() when failure != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( RestoreSessionOutcome_Restored value)  restored,required TResult Function( RestoreSessionOutcome_NotAvailable value)  notAvailable,required TResult Function( RestoreSessionOutcome_Failure value)  failure,}){
final _that = this;
switch (_that) {
case RestoreSessionOutcome_Restored():
return restored(_that);case RestoreSessionOutcome_NotAvailable():
return notAvailable(_that);case RestoreSessionOutcome_Failure():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( RestoreSessionOutcome_Restored value)?  restored,TResult? Function( RestoreSessionOutcome_NotAvailable value)?  notAvailable,TResult? Function( RestoreSessionOutcome_Failure value)?  failure,}){
final _that = this;
switch (_that) {
case RestoreSessionOutcome_Restored() when restored != null:
return restored(_that);case RestoreSessionOutcome_NotAvailable() when notAvailable != null:
return notAvailable(_that);case RestoreSessionOutcome_Failure() when failure != null:
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
case RestoreSessionOutcome_Restored() when restored != null:
return restored();case RestoreSessionOutcome_NotAvailable() when notAvailable != null:
return notAvailable();case RestoreSessionOutcome_Failure() when failure != null:
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
case RestoreSessionOutcome_Restored():
return restored();case RestoreSessionOutcome_NotAvailable():
return notAvailable();case RestoreSessionOutcome_Failure():
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
case RestoreSessionOutcome_Restored() when restored != null:
return restored();case RestoreSessionOutcome_NotAvailable() when notAvailable != null:
return notAvailable();case RestoreSessionOutcome_Failure() when failure != null:
return failure(_that.report);case _:
  return null;

}
}

}

/// @nodoc


class RestoreSessionOutcome_Restored extends RestoreSessionOutcome {
  const RestoreSessionOutcome_Restored(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RestoreSessionOutcome_Restored);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'RestoreSessionOutcome.restored()';
}


}




/// @nodoc


class RestoreSessionOutcome_NotAvailable extends RestoreSessionOutcome {
  const RestoreSessionOutcome_NotAvailable(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RestoreSessionOutcome_NotAvailable);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'RestoreSessionOutcome.notAvailable()';
}


}




/// @nodoc


class RestoreSessionOutcome_Failure extends RestoreSessionOutcome {
  const RestoreSessionOutcome_Failure({required this.report}): super._();
  

 final  ErrorReportDto report;

/// Create a copy of RestoreSessionOutcome
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RestoreSessionOutcome_FailureCopyWith<RestoreSessionOutcome_Failure> get copyWith => _$RestoreSessionOutcome_FailureCopyWithImpl<RestoreSessionOutcome_Failure>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RestoreSessionOutcome_Failure&&(identical(other.report, report) || other.report == report));
}


@override
int get hashCode => Object.hash(runtimeType,report);

@override
String toString() {
  return 'RestoreSessionOutcome.failure(report: $report)';
}


}

/// @nodoc
abstract mixin class $RestoreSessionOutcome_FailureCopyWith<$Res> implements $RestoreSessionOutcomeCopyWith<$Res> {
  factory $RestoreSessionOutcome_FailureCopyWith(RestoreSessionOutcome_Failure value, $Res Function(RestoreSessionOutcome_Failure) _then) = _$RestoreSessionOutcome_FailureCopyWithImpl;
@useResult
$Res call({
 ErrorReportDto report
});




}
/// @nodoc
class _$RestoreSessionOutcome_FailureCopyWithImpl<$Res>
    implements $RestoreSessionOutcome_FailureCopyWith<$Res> {
  _$RestoreSessionOutcome_FailureCopyWithImpl(this._self, this._then);

  final RestoreSessionOutcome_Failure _self;
  final $Res Function(RestoreSessionOutcome_Failure) _then;

/// Create a copy of RestoreSessionOutcome
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? report = null,}) {
  return _then(RestoreSessionOutcome_Failure(
report: null == report ? _self.report : report // ignore: cast_nullable_to_non_nullable
as ErrorReportDto,
  ));
}


}

// dart format on
