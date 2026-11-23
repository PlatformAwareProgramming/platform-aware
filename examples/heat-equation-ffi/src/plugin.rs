use std::ffi::{CStr, CString, c_char, c_int, c_void};
use std::path::{Path, PathBuf};

const RTLD_NOW: c_int = 2;
const RTLD_LOCAL: c_int = 0;

#[cfg_attr(target_os = "linux", link(name = "dl"))]
extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;
    fn dlclose(handle: *mut c_void) -> c_int;
}

/// Shared-library plugin handle loaded dynamically on demand after platform-aware dispatch.
pub struct DynamicPlugin {
    handle: *mut c_void,
}

// Safety: a loaded shared-library handle is process-wide and only used for immutable symbol lookup.
unsafe impl Send for DynamicPlugin {}
unsafe impl Sync for DynamicPlugin {}

impl DynamicPlugin {
    /// Dynamically loads a plugin library (e.g. `"libheat_cuda.so"`).
    ///
    /// Search order:
    /// 1. Explicit per-backend full-path override (`env_override`, e.g. `HEAT_CUDA_LIBRARY`)
    /// 2. `HEAT_PLUGIN_DIR` runtime environment variable (if set)
    /// 3. Directory containing the current executable (sibling plugin)
    /// 4. Compile-time `HEAT_PLUGIN_DIR` emitted by `build.rs`
    /// 5. System loader search path (`LD_LIBRARY_PATH` / default library paths)
    pub fn load(env_override: &str, filename: &str) -> Result<Self, String> {
        if let Some(explicit) = std::env::var_os(env_override) {
            return Self::open_path(Path::new(&explicit))
                .map_err(|e| format!("{}: {e}", Path::new(&explicit).display()));
        }

        let mut candidates: Vec<PathBuf> = Vec::new();

        if let Ok(dir) = std::env::var("HEAT_PLUGIN_DIR") {
            candidates.push(PathBuf::from(dir).join(filename));
        }
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                candidates.push(dir.join(filename));
            }
        }
        if let Some(build_dir) = option_env!("HEAT_PLUGIN_DIR") {
            candidates.push(Path::new(build_dir).join(filename));
        }
        candidates.push(PathBuf::from(filename));

        let mut errors = Vec::new();
        for path in &candidates {
            match Self::open_path(path) {
                Ok(plugin) => return Ok(plugin),
                Err(err) => errors.push(format!("{}: {}", path.display(), err)),
            }
        }

        Err(format!(
            "failed to dynamically load plugin '{filename}'. Tried:\n  {}",
            errors.join("\n  ")
        ))
    }

    pub fn open_path(path: &Path) -> Result<Self, String> {
        let c_path = CString::new(path.as_os_str().as_encoded_bytes())
            .map_err(|e| format!("invalid library path: {e}"))?;
        let handle = unsafe { dlopen(c_path.as_ptr(), RTLD_NOW | RTLD_LOCAL) };
        if handle.is_null() {
            return Err(last_dlerror());
        }
        Ok(Self { handle })
    }

    /// Resolves a raw symbol pointer from the plugin or its `DT_NEEDED` runtime libraries.
    pub unsafe fn symbol_raw(&self, name: &str) -> Result<*mut c_void, String> {
        let c_name = CString::new(name).map_err(|e| format!("invalid symbol name: {e}"))?;
        // Clear any prior dlerror before calling dlsym.
        let _ = dlerror();
        let sym = dlsym(self.handle, c_name.as_ptr());
        let err = dlerror();
        if !err.is_null() {
            let msg = CStr::from_ptr(err).to_string_lossy().into_owned();
            return Err(format!("failed to resolve symbol '{name}': {msg}"));
        }
        if sym.is_null() {
            return Err(format!("symbol '{name}' resolved to null"));
        }
        Ok(sym)
    }
}

impl Drop for DynamicPlugin {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                dlclose(self.handle);
            }
        }
    }
}

fn last_dlerror() -> String {
    unsafe {
        let err = dlerror();
        if err.is_null() {
            "unknown dynamic linker error".to_string()
        } else {
            CStr::from_ptr(err).to_string_lossy().into_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_dynamic_plugin_loads_shared_library_and_resolves_symbol() {
        let tmp_dir = std::env::temp_dir().join(format!("heat_plugin_test_{}", std::process::id()));
        std::fs::create_dir_all(&tmp_dir).unwrap();
        let src = tmp_dir.join("mock_plugin.c");
        let so = tmp_dir.join("libmock_plugin.so");

        std::fs::write(&src, "int mock_add(int a, int b) { return a + b; }\n").unwrap();
        let status = Command::new("cc")
            .args(["-shared", "-fPIC"])
            .arg(&src)
            .arg("-o")
            .arg(&so)
            .status()
            .expect("cc must be available to compile test plugin");
        assert!(status.success());

        let plugin = DynamicPlugin::open_path(&so).expect("plugin should load dynamically");
        let sym = unsafe { plugin.symbol_raw("mock_add") }.expect("symbol should resolve");
        let mock_add: unsafe extern "C" fn(i32, i32) -> i32 = unsafe { std::mem::transmute(sym) };
        assert_eq!(unsafe { mock_add(19, 23) }, 42);

        let _ = std::fs::remove_dir_all(&tmp_dir);
    }

    #[test]
    fn test_missing_plugin_returns_descriptive_error() {
        let err = DynamicPlugin::load(
            "HEAT_NONEXISTENT_TEST_LIBRARY",
            "libnonexistent_heat_plugin_xyz.so",
        )
        .err()
        .expect("missing plugin must fail cleanly");
        assert!(err.contains("libnonexistent_heat_plugin_xyz.so"));
    }
}
