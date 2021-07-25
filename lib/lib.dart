import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:mylib/bindings.dart';

const DYNAMIC_LIBRARY_FILE_NAME = "libexample.so";

class RustLib {
  late final RustBindings _ffi;
  RustLib() {
    _ffi = RustBindings(DynamicLibrary.open(DYNAMIC_LIBRARY_FILE_NAME));
  }

  init() {
    _ffi.database_init();
  }

  createUser(String name) {
    final nativeName = name.toNativeUtf8().cast<Int8>();
    _ffi.database_create_user(nativeName);
    calloc.free(nativeName);
  }
}
