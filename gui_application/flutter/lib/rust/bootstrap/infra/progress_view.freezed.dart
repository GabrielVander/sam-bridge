// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'progress_view.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$AssessStudentProgressOutcome {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AssessStudentProgressOutcome);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AssessStudentProgressOutcome()';
}


}

/// @nodoc
class $AssessStudentProgressOutcomeCopyWith<$Res>  {
$AssessStudentProgressOutcomeCopyWith(AssessStudentProgressOutcome _, $Res Function(AssessStudentProgressOutcome) __);
}


/// Adds pattern-matching-related methods to [AssessStudentProgressOutcome].
extension AssessStudentProgressOutcomePatterns on AssessStudentProgressOutcome {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( AssessStudentProgressOutcome_Success value)?  success,TResult Function( AssessStudentProgressOutcome_NoInstrumentAssigned value)?  noInstrumentAssigned,TResult Function( AssessStudentProgressOutcome_UnknownLevel value)?  unknownLevel,TResult Function( AssessStudentProgressOutcome_Failure value)?  failure,required TResult orElse(),}){
final _that = this;
switch (_that) {
case AssessStudentProgressOutcome_Success() when success != null:
return success(_that);case AssessStudentProgressOutcome_NoInstrumentAssigned() when noInstrumentAssigned != null:
return noInstrumentAssigned(_that);case AssessStudentProgressOutcome_UnknownLevel() when unknownLevel != null:
return unknownLevel(_that);case AssessStudentProgressOutcome_Failure() when failure != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( AssessStudentProgressOutcome_Success value)  success,required TResult Function( AssessStudentProgressOutcome_NoInstrumentAssigned value)  noInstrumentAssigned,required TResult Function( AssessStudentProgressOutcome_UnknownLevel value)  unknownLevel,required TResult Function( AssessStudentProgressOutcome_Failure value)  failure,}){
final _that = this;
switch (_that) {
case AssessStudentProgressOutcome_Success():
return success(_that);case AssessStudentProgressOutcome_NoInstrumentAssigned():
return noInstrumentAssigned(_that);case AssessStudentProgressOutcome_UnknownLevel():
return unknownLevel(_that);case AssessStudentProgressOutcome_Failure():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( AssessStudentProgressOutcome_Success value)?  success,TResult? Function( AssessStudentProgressOutcome_NoInstrumentAssigned value)?  noInstrumentAssigned,TResult? Function( AssessStudentProgressOutcome_UnknownLevel value)?  unknownLevel,TResult? Function( AssessStudentProgressOutcome_Failure value)?  failure,}){
final _that = this;
switch (_that) {
case AssessStudentProgressOutcome_Success() when success != null:
return success(_that);case AssessStudentProgressOutcome_NoInstrumentAssigned() when noInstrumentAssigned != null:
return noInstrumentAssigned(_that);case AssessStudentProgressOutcome_UnknownLevel() when unknownLevel != null:
return unknownLevel(_that);case AssessStudentProgressOutcome_Failure() when failure != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( ProgressAssessmentDto field0)?  success,TResult Function()?  noInstrumentAssigned,TResult Function( String field0)?  unknownLevel,TResult Function( String field0)?  failure,required TResult orElse(),}) {final _that = this;
switch (_that) {
case AssessStudentProgressOutcome_Success() when success != null:
return success(_that.field0);case AssessStudentProgressOutcome_NoInstrumentAssigned() when noInstrumentAssigned != null:
return noInstrumentAssigned();case AssessStudentProgressOutcome_UnknownLevel() when unknownLevel != null:
return unknownLevel(_that.field0);case AssessStudentProgressOutcome_Failure() when failure != null:
return failure(_that.field0);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( ProgressAssessmentDto field0)  success,required TResult Function()  noInstrumentAssigned,required TResult Function( String field0)  unknownLevel,required TResult Function( String field0)  failure,}) {final _that = this;
switch (_that) {
case AssessStudentProgressOutcome_Success():
return success(_that.field0);case AssessStudentProgressOutcome_NoInstrumentAssigned():
return noInstrumentAssigned();case AssessStudentProgressOutcome_UnknownLevel():
return unknownLevel(_that.field0);case AssessStudentProgressOutcome_Failure():
return failure(_that.field0);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( ProgressAssessmentDto field0)?  success,TResult? Function()?  noInstrumentAssigned,TResult? Function( String field0)?  unknownLevel,TResult? Function( String field0)?  failure,}) {final _that = this;
switch (_that) {
case AssessStudentProgressOutcome_Success() when success != null:
return success(_that.field0);case AssessStudentProgressOutcome_NoInstrumentAssigned() when noInstrumentAssigned != null:
return noInstrumentAssigned();case AssessStudentProgressOutcome_UnknownLevel() when unknownLevel != null:
return unknownLevel(_that.field0);case AssessStudentProgressOutcome_Failure() when failure != null:
return failure(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class AssessStudentProgressOutcome_Success extends AssessStudentProgressOutcome {
  const AssessStudentProgressOutcome_Success(this.field0): super._();
  

 final  ProgressAssessmentDto field0;

/// Create a copy of AssessStudentProgressOutcome
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AssessStudentProgressOutcome_SuccessCopyWith<AssessStudentProgressOutcome_Success> get copyWith => _$AssessStudentProgressOutcome_SuccessCopyWithImpl<AssessStudentProgressOutcome_Success>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AssessStudentProgressOutcome_Success&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'AssessStudentProgressOutcome.success(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $AssessStudentProgressOutcome_SuccessCopyWith<$Res> implements $AssessStudentProgressOutcomeCopyWith<$Res> {
  factory $AssessStudentProgressOutcome_SuccessCopyWith(AssessStudentProgressOutcome_Success value, $Res Function(AssessStudentProgressOutcome_Success) _then) = _$AssessStudentProgressOutcome_SuccessCopyWithImpl;
@useResult
$Res call({
 ProgressAssessmentDto field0
});




}
/// @nodoc
class _$AssessStudentProgressOutcome_SuccessCopyWithImpl<$Res>
    implements $AssessStudentProgressOutcome_SuccessCopyWith<$Res> {
  _$AssessStudentProgressOutcome_SuccessCopyWithImpl(this._self, this._then);

  final AssessStudentProgressOutcome_Success _self;
  final $Res Function(AssessStudentProgressOutcome_Success) _then;

/// Create a copy of AssessStudentProgressOutcome
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(AssessStudentProgressOutcome_Success(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as ProgressAssessmentDto,
  ));
}


}

/// @nodoc


class AssessStudentProgressOutcome_NoInstrumentAssigned extends AssessStudentProgressOutcome {
  const AssessStudentProgressOutcome_NoInstrumentAssigned(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AssessStudentProgressOutcome_NoInstrumentAssigned);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AssessStudentProgressOutcome.noInstrumentAssigned()';
}


}




/// @nodoc


class AssessStudentProgressOutcome_UnknownLevel extends AssessStudentProgressOutcome {
  const AssessStudentProgressOutcome_UnknownLevel(this.field0): super._();
  

 final  String field0;

/// Create a copy of AssessStudentProgressOutcome
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AssessStudentProgressOutcome_UnknownLevelCopyWith<AssessStudentProgressOutcome_UnknownLevel> get copyWith => _$AssessStudentProgressOutcome_UnknownLevelCopyWithImpl<AssessStudentProgressOutcome_UnknownLevel>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AssessStudentProgressOutcome_UnknownLevel&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'AssessStudentProgressOutcome.unknownLevel(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $AssessStudentProgressOutcome_UnknownLevelCopyWith<$Res> implements $AssessStudentProgressOutcomeCopyWith<$Res> {
  factory $AssessStudentProgressOutcome_UnknownLevelCopyWith(AssessStudentProgressOutcome_UnknownLevel value, $Res Function(AssessStudentProgressOutcome_UnknownLevel) _then) = _$AssessStudentProgressOutcome_UnknownLevelCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$AssessStudentProgressOutcome_UnknownLevelCopyWithImpl<$Res>
    implements $AssessStudentProgressOutcome_UnknownLevelCopyWith<$Res> {
  _$AssessStudentProgressOutcome_UnknownLevelCopyWithImpl(this._self, this._then);

  final AssessStudentProgressOutcome_UnknownLevel _self;
  final $Res Function(AssessStudentProgressOutcome_UnknownLevel) _then;

/// Create a copy of AssessStudentProgressOutcome
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(AssessStudentProgressOutcome_UnknownLevel(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class AssessStudentProgressOutcome_Failure extends AssessStudentProgressOutcome {
  const AssessStudentProgressOutcome_Failure(this.field0): super._();
  

 final  String field0;

/// Create a copy of AssessStudentProgressOutcome
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AssessStudentProgressOutcome_FailureCopyWith<AssessStudentProgressOutcome_Failure> get copyWith => _$AssessStudentProgressOutcome_FailureCopyWithImpl<AssessStudentProgressOutcome_Failure>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AssessStudentProgressOutcome_Failure&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'AssessStudentProgressOutcome.failure(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $AssessStudentProgressOutcome_FailureCopyWith<$Res> implements $AssessStudentProgressOutcomeCopyWith<$Res> {
  factory $AssessStudentProgressOutcome_FailureCopyWith(AssessStudentProgressOutcome_Failure value, $Res Function(AssessStudentProgressOutcome_Failure) _then) = _$AssessStudentProgressOutcome_FailureCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$AssessStudentProgressOutcome_FailureCopyWithImpl<$Res>
    implements $AssessStudentProgressOutcome_FailureCopyWith<$Res> {
  _$AssessStudentProgressOutcome_FailureCopyWithImpl(this._self, this._then);

  final AssessStudentProgressOutcome_Failure _self;
  final $Res Function(AssessStudentProgressOutcome_Failure) _then;

/// Create a copy of AssessStudentProgressOutcome
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(AssessStudentProgressOutcome_Failure(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
