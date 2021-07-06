import 'dart:ffi' as ffi;
import 'dart:io';
import 'package:ffi/ffi.dart';

const DYNAMIC_LIBRARY_FILE_NAME = "libexample.so";

/// Wraps the native functions and converts specific data types in order to
/// handle C strings.

class Greeter {
  static final ffi.DynamicLibrary _dylib = Greeter._loadLibrary();

  static ffi.DynamicLibrary _loadLibrary() {
    return ffi.DynamicLibrary.open(DYNAMIC_LIBRARY_FILE_NAME);
  }

  /// Computes a greeting for the given name using the native function
  static String greet(String name) {
    final ffi.Pointer<Utf8> charPointer = name.toNativeUtf8();

    final result = _dylib.lookupFunction<_c_rust_greeting, _dart_rust_greeting>('rust_greeting');

    // Clone the given result, so that the original string can be freed
    final resultString = result(charPointer);
    final resultCopy = resultString.toDartString();

    calloc.free(charPointer);
    calloc.free(resultString);

    return resultCopy;
    
  }
}

typedef _c_rust_greeting = ffi.Pointer<Utf8> Function(ffi.Pointer<Utf8>);

typedef _dart_rust_greeting = ffi.Pointer<Utf8> Function(ffi.Pointer<Utf8>);
