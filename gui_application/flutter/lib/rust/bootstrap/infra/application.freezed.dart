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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  successful,TResult Function()?  invalidEmailOrPassword,TResult Function( String context)?  unableToPerformAuthorization,required TResult orElse(),}) {final _that = this;
switch (_that) {
case LoginResult_Successful() when successful != null:
return successful();case LoginResult_InvalidEmailOrPassword() when invalidEmailOrPassword != null:
return invalidEmailOrPassword();case LoginResult_UnableToPerformAuthorization() when unableToPerformAuthorization != null:
return unableToPerformAuthorization(_that.context);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  successful,required TResult Function()  invalidEmailOrPassword,required TResult Function( String context)  unableToPerformAuthorization,}) {final _that = this;
switch (_that) {
case LoginResult_Successful():
return successful();case LoginResult_InvalidEmailOrPassword():
return invalidEmailOrPassword();case LoginResult_UnableToPerformAuthorization():
return unableToPerformAuthorization(_that.context);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  successful,TResult? Function()?  invalidEmailOrPassword,TResult? Function( String context)?  unableToPerformAuthorization,}) {final _that = this;
switch (_that) {
case LoginResult_Successful() when successful != null:
return successful();case LoginResult_InvalidEmailOrPassword() when invalidEmailOrPassword != null:
return invalidEmailOrPassword();case LoginResult_UnableToPerformAuthorization() when unableToPerformAuthorization != null:
return unableToPerformAuthorization(_that.context);case _:
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
  const LoginResult_UnableToPerformAuthorization({required this.context}): super._();
  

 final  String context;

/// Create a copy of LoginResult
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$LoginResult_UnableToPerformAuthorizationCopyWith<LoginResult_UnableToPerformAuthorization> get copyWith => _$LoginResult_UnableToPerformAuthorizationCopyWithImpl<LoginResult_UnableToPerformAuthorization>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LoginResult_UnableToPerformAuthorization&&(identical(other.context, context) || other.context == context));
}


@override
int get hashCode => Object.hash(runtimeType,context);

@override
String toString() {
  return 'LoginResult.unableToPerformAuthorization(context: $context)';
}


}

/// @nodoc
abstract mixin class $LoginResult_UnableToPerformAuthorizationCopyWith<$Res> implements $LoginResultCopyWith<$Res> {
  factory $LoginResult_UnableToPerformAuthorizationCopyWith(LoginResult_UnableToPerformAuthorization value, $Res Function(LoginResult_UnableToPerformAuthorization) _then) = _$LoginResult_UnableToPerformAuthorizationCopyWithImpl;
@useResult
$Res call({
 String context
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
@pragma('vm:prefer-inline') $Res call({Object? context = null,}) {
  return _then(LoginResult_UnableToPerformAuthorization(
context: null == context ? _self.context : context // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
