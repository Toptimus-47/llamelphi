import 'dart:ffi';
import 'dart:io';
import 'package:path/path.dart' as p;

typedef StartMagiBackendFunc = Int32 Function();
typedef StartMagiBackend = int Function();

class MagiFfi {
  static DynamicLibrary? _lib;

  static void init() {
    if (_lib != null) return;

    String libPath = "";
    if (Platform.isWindows) {
      // In development, look for the DLL in the target folder
      libPath = p.absolute('../magi_core/target/release/magi_core.dll');
      if (!File(libPath).existsSync()) {
        libPath = p.absolute('magi_core.dll'); // Check current dir (bundle)
      }
    } else if (Platform.isLinux) {
      libPath = p.absolute('../magi_core/target/release/libmagi_core.so');
    }

    try {
      _lib = DynamicLibrary.open(libPath);
      print("[MagiFfi] Native library loaded from: $libPath");
    } catch (e) {
      print("[MagiFfi] Failed to load native library: $e");
    }
  }

  static int startBackend() {
    if (_lib == null) init();
    if (_lib == null) return -1;

    final StartMagiBackend startFunc = _lib!
        .lookup<NativeFunction<StartMagiBackendFunc>>('start_magi_backend')
        .asFunction();

    return startFunc();
  }
}
