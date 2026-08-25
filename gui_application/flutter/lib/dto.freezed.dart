// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'dto.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$DtoPosition {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DtoPosition);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'DtoPosition()';
}


}

/// @nodoc
class $DtoPositionCopyWith<$Res>  {
$DtoPositionCopyWith(DtoPosition _, $Res Function(DtoPosition) __);
}


/// Adds pattern-matching-related methods to [DtoPosition].
extension DtoPositionPatterns on DtoPosition {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( DtoPosition_Musician value)?  musician,TResult Function( DtoPosition_Organist value)?  organist,TResult Function( DtoPosition_Secretary value)?  secretary,TResult Function( DtoPosition_Unknown value)?  unknown,required TResult orElse(),}){
final _that = this;
switch (_that) {
case DtoPosition_Musician() when musician != null:
return musician(_that);case DtoPosition_Organist() when organist != null:
return organist(_that);case DtoPosition_Secretary() when secretary != null:
return secretary(_that);case DtoPosition_Unknown() when unknown != null:
return unknown(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( DtoPosition_Musician value)  musician,required TResult Function( DtoPosition_Organist value)  organist,required TResult Function( DtoPosition_Secretary value)  secretary,required TResult Function( DtoPosition_Unknown value)  unknown,}){
final _that = this;
switch (_that) {
case DtoPosition_Musician():
return musician(_that);case DtoPosition_Organist():
return organist(_that);case DtoPosition_Secretary():
return secretary(_that);case DtoPosition_Unknown():
return unknown(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( DtoPosition_Musician value)?  musician,TResult? Function( DtoPosition_Organist value)?  organist,TResult? Function( DtoPosition_Secretary value)?  secretary,TResult? Function( DtoPosition_Unknown value)?  unknown,}){
final _that = this;
switch (_that) {
case DtoPosition_Musician() when musician != null:
return musician(_that);case DtoPosition_Organist() when organist != null:
return organist(_that);case DtoPosition_Secretary() when secretary != null:
return secretary(_that);case DtoPosition_Unknown() when unknown != null:
return unknown(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String levelName)?  musician,TResult Function( String levelName)?  organist,TResult Function( String typeName)?  secretary,TResult Function( String raw)?  unknown,required TResult orElse(),}) {final _that = this;
switch (_that) {
case DtoPosition_Musician() when musician != null:
return musician(_that.levelName);case DtoPosition_Organist() when organist != null:
return organist(_that.levelName);case DtoPosition_Secretary() when secretary != null:
return secretary(_that.typeName);case DtoPosition_Unknown() when unknown != null:
return unknown(_that.raw);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String levelName)  musician,required TResult Function( String levelName)  organist,required TResult Function( String typeName)  secretary,required TResult Function( String raw)  unknown,}) {final _that = this;
switch (_that) {
case DtoPosition_Musician():
return musician(_that.levelName);case DtoPosition_Organist():
return organist(_that.levelName);case DtoPosition_Secretary():
return secretary(_that.typeName);case DtoPosition_Unknown():
return unknown(_that.raw);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String levelName)?  musician,TResult? Function( String levelName)?  organist,TResult? Function( String typeName)?  secretary,TResult? Function( String raw)?  unknown,}) {final _that = this;
switch (_that) {
case DtoPosition_Musician() when musician != null:
return musician(_that.levelName);case DtoPosition_Organist() when organist != null:
return organist(_that.levelName);case DtoPosition_Secretary() when secretary != null:
return secretary(_that.typeName);case DtoPosition_Unknown() when unknown != null:
return unknown(_that.raw);case _:
  return null;

}
}

}

/// @nodoc


class DtoPosition_Musician extends DtoPosition {
  const DtoPosition_Musician({required this.levelName}): super._();
  

 final  String levelName;

/// Create a copy of DtoPosition
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DtoPosition_MusicianCopyWith<DtoPosition_Musician> get copyWith => _$DtoPosition_MusicianCopyWithImpl<DtoPosition_Musician>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DtoPosition_Musician&&(identical(other.levelName, levelName) || other.levelName == levelName));
}


@override
int get hashCode => Object.hash(runtimeType,levelName);

@override
String toString() {
  return 'DtoPosition.musician(levelName: $levelName)';
}


}

/// @nodoc
abstract mixin class $DtoPosition_MusicianCopyWith<$Res> implements $DtoPositionCopyWith<$Res> {
  factory $DtoPosition_MusicianCopyWith(DtoPosition_Musician value, $Res Function(DtoPosition_Musician) _then) = _$DtoPosition_MusicianCopyWithImpl;
@useResult
$Res call({
 String levelName
});




}
/// @nodoc
class _$DtoPosition_MusicianCopyWithImpl<$Res>
    implements $DtoPosition_MusicianCopyWith<$Res> {
  _$DtoPosition_MusicianCopyWithImpl(this._self, this._then);

  final DtoPosition_Musician _self;
  final $Res Function(DtoPosition_Musician) _then;

/// Create a copy of DtoPosition
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? levelName = null,}) {
  return _then(DtoPosition_Musician(
levelName: null == levelName ? _self.levelName : levelName // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class DtoPosition_Organist extends DtoPosition {
  const DtoPosition_Organist({required this.levelName}): super._();
  

 final  String levelName;

/// Create a copy of DtoPosition
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DtoPosition_OrganistCopyWith<DtoPosition_Organist> get copyWith => _$DtoPosition_OrganistCopyWithImpl<DtoPosition_Organist>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DtoPosition_Organist&&(identical(other.levelName, levelName) || other.levelName == levelName));
}


@override
int get hashCode => Object.hash(runtimeType,levelName);

@override
String toString() {
  return 'DtoPosition.organist(levelName: $levelName)';
}


}

/// @nodoc
abstract mixin class $DtoPosition_OrganistCopyWith<$Res> implements $DtoPositionCopyWith<$Res> {
  factory $DtoPosition_OrganistCopyWith(DtoPosition_Organist value, $Res Function(DtoPosition_Organist) _then) = _$DtoPosition_OrganistCopyWithImpl;
@useResult
$Res call({
 String levelName
});




}
/// @nodoc
class _$DtoPosition_OrganistCopyWithImpl<$Res>
    implements $DtoPosition_OrganistCopyWith<$Res> {
  _$DtoPosition_OrganistCopyWithImpl(this._self, this._then);

  final DtoPosition_Organist _self;
  final $Res Function(DtoPosition_Organist) _then;

/// Create a copy of DtoPosition
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? levelName = null,}) {
  return _then(DtoPosition_Organist(
levelName: null == levelName ? _self.levelName : levelName // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class DtoPosition_Secretary extends DtoPosition {
  const DtoPosition_Secretary({required this.typeName}): super._();
  

 final  String typeName;

/// Create a copy of DtoPosition
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DtoPosition_SecretaryCopyWith<DtoPosition_Secretary> get copyWith => _$DtoPosition_SecretaryCopyWithImpl<DtoPosition_Secretary>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DtoPosition_Secretary&&(identical(other.typeName, typeName) || other.typeName == typeName));
}


@override
int get hashCode => Object.hash(runtimeType,typeName);

@override
String toString() {
  return 'DtoPosition.secretary(typeName: $typeName)';
}


}

/// @nodoc
abstract mixin class $DtoPosition_SecretaryCopyWith<$Res> implements $DtoPositionCopyWith<$Res> {
  factory $DtoPosition_SecretaryCopyWith(DtoPosition_Secretary value, $Res Function(DtoPosition_Secretary) _then) = _$DtoPosition_SecretaryCopyWithImpl;
@useResult
$Res call({
 String typeName
});




}
/// @nodoc
class _$DtoPosition_SecretaryCopyWithImpl<$Res>
    implements $DtoPosition_SecretaryCopyWith<$Res> {
  _$DtoPosition_SecretaryCopyWithImpl(this._self, this._then);

  final DtoPosition_Secretary _self;
  final $Res Function(DtoPosition_Secretary) _then;

/// Create a copy of DtoPosition
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? typeName = null,}) {
  return _then(DtoPosition_Secretary(
typeName: null == typeName ? _self.typeName : typeName // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class DtoPosition_Unknown extends DtoPosition {
  const DtoPosition_Unknown({required this.raw}): super._();
  

 final  String raw;

/// Create a copy of DtoPosition
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DtoPosition_UnknownCopyWith<DtoPosition_Unknown> get copyWith => _$DtoPosition_UnknownCopyWithImpl<DtoPosition_Unknown>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DtoPosition_Unknown&&(identical(other.raw, raw) || other.raw == raw));
}


@override
int get hashCode => Object.hash(runtimeType,raw);

@override
String toString() {
  return 'DtoPosition.unknown(raw: $raw)';
}


}

/// @nodoc
abstract mixin class $DtoPosition_UnknownCopyWith<$Res> implements $DtoPositionCopyWith<$Res> {
  factory $DtoPosition_UnknownCopyWith(DtoPosition_Unknown value, $Res Function(DtoPosition_Unknown) _then) = _$DtoPosition_UnknownCopyWithImpl;
@useResult
$Res call({
 String raw
});




}
/// @nodoc
class _$DtoPosition_UnknownCopyWithImpl<$Res>
    implements $DtoPosition_UnknownCopyWith<$Res> {
  _$DtoPosition_UnknownCopyWithImpl(this._self, this._then);

  final DtoPosition_Unknown _self;
  final $Res Function(DtoPosition_Unknown) _then;

/// Create a copy of DtoPosition
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? raw = null,}) {
  return _then(DtoPosition_Unknown(
raw: null == raw ? _self.raw : raw // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$DtoRegion {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DtoRegion);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'DtoRegion()';
}


}

/// @nodoc
class $DtoRegionCopyWith<$Res>  {
$DtoRegionCopyWith(DtoRegion _, $Res Function(DtoRegion) __);
}


/// Adds pattern-matching-related methods to [DtoRegion].
extension DtoRegionPatterns on DtoRegion {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( DtoRegion_AraraquaraSaoCarlos value)?  araraquaraSaoCarlos,TResult Function( DtoRegion_AraraquaraItirapina value)?  araraquaraItirapina,TResult Function( DtoRegion_Other value)?  other,required TResult orElse(),}){
final _that = this;
switch (_that) {
case DtoRegion_AraraquaraSaoCarlos() when araraquaraSaoCarlos != null:
return araraquaraSaoCarlos(_that);case DtoRegion_AraraquaraItirapina() when araraquaraItirapina != null:
return araraquaraItirapina(_that);case DtoRegion_Other() when other != null:
return other(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( DtoRegion_AraraquaraSaoCarlos value)  araraquaraSaoCarlos,required TResult Function( DtoRegion_AraraquaraItirapina value)  araraquaraItirapina,required TResult Function( DtoRegion_Other value)  other,}){
final _that = this;
switch (_that) {
case DtoRegion_AraraquaraSaoCarlos():
return araraquaraSaoCarlos(_that);case DtoRegion_AraraquaraItirapina():
return araraquaraItirapina(_that);case DtoRegion_Other():
return other(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( DtoRegion_AraraquaraSaoCarlos value)?  araraquaraSaoCarlos,TResult? Function( DtoRegion_AraraquaraItirapina value)?  araraquaraItirapina,TResult? Function( DtoRegion_Other value)?  other,}){
final _that = this;
switch (_that) {
case DtoRegion_AraraquaraSaoCarlos() when araraquaraSaoCarlos != null:
return araraquaraSaoCarlos(_that);case DtoRegion_AraraquaraItirapina() when araraquaraItirapina != null:
return araraquaraItirapina(_that);case DtoRegion_Other() when other != null:
return other(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  araraquaraSaoCarlos,TResult Function()?  araraquaraItirapina,TResult Function( String raw)?  other,required TResult orElse(),}) {final _that = this;
switch (_that) {
case DtoRegion_AraraquaraSaoCarlos() when araraquaraSaoCarlos != null:
return araraquaraSaoCarlos();case DtoRegion_AraraquaraItirapina() when araraquaraItirapina != null:
return araraquaraItirapina();case DtoRegion_Other() when other != null:
return other(_that.raw);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  araraquaraSaoCarlos,required TResult Function()  araraquaraItirapina,required TResult Function( String raw)  other,}) {final _that = this;
switch (_that) {
case DtoRegion_AraraquaraSaoCarlos():
return araraquaraSaoCarlos();case DtoRegion_AraraquaraItirapina():
return araraquaraItirapina();case DtoRegion_Other():
return other(_that.raw);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  araraquaraSaoCarlos,TResult? Function()?  araraquaraItirapina,TResult? Function( String raw)?  other,}) {final _that = this;
switch (_that) {
case DtoRegion_AraraquaraSaoCarlos() when araraquaraSaoCarlos != null:
return araraquaraSaoCarlos();case DtoRegion_AraraquaraItirapina() when araraquaraItirapina != null:
return araraquaraItirapina();case DtoRegion_Other() when other != null:
return other(_that.raw);case _:
  return null;

}
}

}

/// @nodoc


class DtoRegion_AraraquaraSaoCarlos extends DtoRegion {
  const DtoRegion_AraraquaraSaoCarlos(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DtoRegion_AraraquaraSaoCarlos);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'DtoRegion.araraquaraSaoCarlos()';
}


}




/// @nodoc


class DtoRegion_AraraquaraItirapina extends DtoRegion {
  const DtoRegion_AraraquaraItirapina(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DtoRegion_AraraquaraItirapina);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'DtoRegion.araraquaraItirapina()';
}


}




/// @nodoc


class DtoRegion_Other extends DtoRegion {
  const DtoRegion_Other({required this.raw}): super._();
  

 final  String raw;

/// Create a copy of DtoRegion
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DtoRegion_OtherCopyWith<DtoRegion_Other> get copyWith => _$DtoRegion_OtherCopyWithImpl<DtoRegion_Other>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DtoRegion_Other&&(identical(other.raw, raw) || other.raw == raw));
}


@override
int get hashCode => Object.hash(runtimeType,raw);

@override
String toString() {
  return 'DtoRegion.other(raw: $raw)';
}


}

/// @nodoc
abstract mixin class $DtoRegion_OtherCopyWith<$Res> implements $DtoRegionCopyWith<$Res> {
  factory $DtoRegion_OtherCopyWith(DtoRegion_Other value, $Res Function(DtoRegion_Other) _then) = _$DtoRegion_OtherCopyWithImpl;
@useResult
$Res call({
 String raw
});




}
/// @nodoc
class _$DtoRegion_OtherCopyWithImpl<$Res>
    implements $DtoRegion_OtherCopyWith<$Res> {
  _$DtoRegion_OtherCopyWithImpl(this._self, this._then);

  final DtoRegion_Other _self;
  final $Res Function(DtoRegion_Other) _then;

/// Create a copy of DtoRegion
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? raw = null,}) {
  return _then(DtoRegion_Other(
raw: null == raw ? _self.raw : raw // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
