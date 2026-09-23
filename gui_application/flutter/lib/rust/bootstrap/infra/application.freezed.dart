// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'application.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$LoginResult {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LoginResult);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'LoginResult()';
}


}

/// @nodoc
class $LoginResultCopyWith<$Res>  {
$LoginResultCopyWith(LoginResult _, $Res Function(LoginResult) __);
}


/// Adds pattern-matching-related methods to [LoginResult].
extension LoginResultPatterns on LoginResult {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( LoginResult_Successful value)?  successful,TResult Function( LoginResult_InvalidEmailOrPassword value)?  invalidEmailOrPassword,TResult Function( LoginResult_UnableToPerformAuthorization value)?  unableToPerformAuthorization,required TResult orElse(),}){
final _that = this;
switch (_that) {
case LoginResult_Successful() when successful != null:
return successful(_that);case LoginResult_InvalidEmailOrPassword() when invalidEmailOrPassword != null:
return invalidEmailOrPassword(_that);case LoginResult_UnableToPerformAuthorization() when unableToPerformAuthorization != null:
return unableToPerformAuthorization(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( LoginResult_Successful value)  successful,required TResult Function( LoginResult_InvalidEmailOrPassword value)  invalidEmailOrPassword,required TResult Function( LoginResult_UnableToPerformAuthorization value)  unableToPerformAuthorization,}){
final _that = this;
switch (_that) {
case LoginResult_Successful():
return successful(_that);case LoginResult_InvalidEmailOrPassword():
return invalidEmailOrPassword(_that);case LoginResult_UnableToPerformAuthorization():
return unableToPerformAuthorization(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( LoginResult_Successful value)?  successful,TResult? Function( LoginResult_InvalidEmailOrPassword value)?  invalidEmailOrPassword,TResult? Function( LoginResult_UnableToPerformAuthorization value)?  unableToPerformAuthorization,}){
final _that = this;
switch (_that) {
case LoginResult_Successful() when successful != null:
return successful(_that);case LoginResult_InvalidEmailOrPassword() when invalidEmailOrPassword != null:
return invalidEmailOrPassword(_that);case LoginResult_UnableToPerformAuthorization() when unableToPerformAuthorization != null:
return unableToPerformAuthorization(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  successful,TResult Function()?  invalidEmailOrPassword,TResult Function( ErrorReportDto field0)?  unableToPerformAuthorization,required TResult orElse(),}) {final _that = this;
switch (_that) {
case LoginResult_Successful() when successful != null:
return successful();case LoginResult_InvalidEmailOrPassword() when invalidEmailOrPassword != null:
return invalidEmailOrPassword();case LoginResult_UnableToPerformAuthorization() when unableToPerformAuthorization != null:
return unableToPerformAuthorization(_that.field0);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  successful,required TResult Function()  invalidEmailOrPassword,required TResult Function( ErrorReportDto field0)  unableToPerformAuthorization,}) {final _that = this;
switch (_that) {
case LoginResult_Successful():
return successful();case LoginResult_InvalidEmailOrPassword():
return invalidEmailOrPassword();case LoginResult_UnableToPerformAuthorization():
return unableToPerformAuthorization(_that.field0);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  successful,TResult? Function()?  invalidEmailOrPassword,TResult? Function( ErrorReportDto field0)?  unableToPerformAuthorization,}) {final _that = this;
switch (_that) {
case LoginResult_Successful() when successful != null:
return successful();case LoginResult_InvalidEmailOrPassword() when invalidEmailOrPassword != null:
return invalidEmailOrPassword();case LoginResult_UnableToPerformAuthorization() when unableToPerformAuthorization != null:
return unableToPerformAuthorization(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class LoginResult_Successful extends LoginResult {
  const LoginResult_Successful(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LoginResult_Successful);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'LoginResult.successful()';
}


}




/// @nodoc


class LoginResult_InvalidEmailOrPassword extends LoginResult {
  const LoginResult_InvalidEmailOrPassword(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LoginResult_InvalidEmailOrPassword);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'LoginResult.invalidEmailOrPassword()';
}


}




/// @nodoc


class LoginResult_UnableToPerformAuthorization extends LoginResult {
  const LoginResult_UnableToPerformAuthorization(this.field0): super._();
  

 final  ErrorReportDto field0;

/// Create a copy of LoginResult
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$LoginResult_UnableToPerformAuthorizationCopyWith<LoginResult_UnableToPerformAuthorization> get copyWith => _$LoginResult_UnableToPerformAuthorizationCopyWithImpl<LoginResult_UnableToPerformAuthorization>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LoginResult_UnableToPerformAuthorization&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'LoginResult.unableToPerformAuthorization(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $LoginResult_UnableToPerformAuthorizationCopyWith<$Res> implements $LoginResultCopyWith<$Res> {
  factory $LoginResult_UnableToPerformAuthorizationCopyWith(LoginResult_UnableToPerformAuthorization value, $Res Function(LoginResult_UnableToPerformAuthorization) _then) = _$LoginResult_UnableToPerformAuthorizationCopyWithImpl;
@useResult
$Res call({
 ErrorReportDto field0
});




}
/// @nodoc
class _$LoginResult_UnableToPerformAuthorizationCopyWithImpl<$Res>
    implements $LoginResult_UnableToPerformAuthorizationCopyWith<$Res> {
  _$LoginResult_UnableToPerformAuthorizationCopyWithImpl(this._self, this._then);

  final LoginResult_UnableToPerformAuthorization _self;
  final $Res Function(LoginResult_UnableToPerformAuthorization) _then;

/// Create a copy of LoginResult
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(LoginResult_UnableToPerformAuthorization(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( RestoreSessionOutcome_Restored value)?  restored,TResult Function( RestoreSessionOutcome_NotAvailable value)?  notAvailable,TResult Function( RestoreSessionOutcome_UnableToPerformOperation value)?  unableToPerformOperation,required TResult orElse(),}){
final _that = this;
switch (_that) {
case RestoreSessionOutcome_Restored() when restored != null:
return restored(_that);case RestoreSessionOutcome_NotAvailable() when notAvailable != null:
return notAvailable(_that);case RestoreSessionOutcome_UnableToPerformOperation() when unableToPerformOperation != null:
return unableToPerformOperation(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( RestoreSessionOutcome_Restored value)  restored,required TResult Function( RestoreSessionOutcome_NotAvailable value)  notAvailable,required TResult Function( RestoreSessionOutcome_UnableToPerformOperation value)  unableToPerformOperation,}){
final _that = this;
switch (_that) {
case RestoreSessionOutcome_Restored():
return restored(_that);case RestoreSessionOutcome_NotAvailable():
return notAvailable(_that);case RestoreSessionOutcome_UnableToPerformOperation():
return unableToPerformOperation(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( RestoreSessionOutcome_Restored value)?  restored,TResult? Function( RestoreSessionOutcome_NotAvailable value)?  notAvailable,TResult? Function( RestoreSessionOutcome_UnableToPerformOperation value)?  unableToPerformOperation,}){
final _that = this;
switch (_that) {
case RestoreSessionOutcome_Restored() when restored != null:
return restored(_that);case RestoreSessionOutcome_NotAvailable() when notAvailable != null:
return notAvailable(_that);case RestoreSessionOutcome_UnableToPerformOperation() when unableToPerformOperation != null:
return unableToPerformOperation(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  restored,TResult Function()?  notAvailable,TResult Function( ErrorReportDto field0)?  unableToPerformOperation,required TResult orElse(),}) {final _that = this;
switch (_that) {
case RestoreSessionOutcome_Restored() when restored != null:
return restored();case RestoreSessionOutcome_NotAvailable() when notAvailable != null:
return notAvailable();case RestoreSessionOutcome_UnableToPerformOperation() when unableToPerformOperation != null:
return unableToPerformOperation(_that.field0);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  restored,required TResult Function()  notAvailable,required TResult Function( ErrorReportDto field0)  unableToPerformOperation,}) {final _that = this;
switch (_that) {
case RestoreSessionOutcome_Restored():
return restored();case RestoreSessionOutcome_NotAvailable():
return notAvailable();case RestoreSessionOutcome_UnableToPerformOperation():
return unableToPerformOperation(_that.field0);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  restored,TResult? Function()?  notAvailable,TResult? Function( ErrorReportDto field0)?  unableToPerformOperation,}) {final _that = this;
switch (_that) {
case RestoreSessionOutcome_Restored() when restored != null:
return restored();case RestoreSessionOutcome_NotAvailable() when notAvailable != null:
return notAvailable();case RestoreSessionOutcome_UnableToPerformOperation() when unableToPerformOperation != null:
return unableToPerformOperation(_that.field0);case _:
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


class RestoreSessionOutcome_UnableToPerformOperation extends RestoreSessionOutcome {
  const RestoreSessionOutcome_UnableToPerformOperation(this.field0): super._();
  

 final  ErrorReportDto field0;

/// Create a copy of RestoreSessionOutcome
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RestoreSessionOutcome_UnableToPerformOperationCopyWith<RestoreSessionOutcome_UnableToPerformOperation> get copyWith => _$RestoreSessionOutcome_UnableToPerformOperationCopyWithImpl<RestoreSessionOutcome_UnableToPerformOperation>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RestoreSessionOutcome_UnableToPerformOperation&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'RestoreSessionOutcome.unableToPerformOperation(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $RestoreSessionOutcome_UnableToPerformOperationCopyWith<$Res> implements $RestoreSessionOutcomeCopyWith<$Res> {
  factory $RestoreSessionOutcome_UnableToPerformOperationCopyWith(RestoreSessionOutcome_UnableToPerformOperation value, $Res Function(RestoreSessionOutcome_UnableToPerformOperation) _then) = _$RestoreSessionOutcome_UnableToPerformOperationCopyWithImpl;
@useResult
$Res call({
 ErrorReportDto field0
});




}
/// @nodoc
class _$RestoreSessionOutcome_UnableToPerformOperationCopyWithImpl<$Res>
    implements $RestoreSessionOutcome_UnableToPerformOperationCopyWith<$Res> {
  _$RestoreSessionOutcome_UnableToPerformOperationCopyWithImpl(this._self, this._then);

  final RestoreSessionOutcome_UnableToPerformOperation _self;
  final $Res Function(RestoreSessionOutcome_UnableToPerformOperation) _then;

/// Create a copy of RestoreSessionOutcome
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(RestoreSessionOutcome_UnableToPerformOperation(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as ErrorReportDto,
  ));
}


}

// dart format on
