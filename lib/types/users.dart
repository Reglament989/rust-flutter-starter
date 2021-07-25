import 'dart:ffi';

import 'package:ffi/ffi.dart';

class User extends Struct {
  @Int32()
  external int id;

  external Pointer<Utf8> title;

  @override
  String toString() {
    return title.toDartString();
  }
}
