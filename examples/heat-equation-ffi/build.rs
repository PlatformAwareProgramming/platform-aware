use std::{env, path::PathBuf, process::Command};

fn run(mut command: Command, tool: &str) {
    let status = command.status().unwrap_or_else(|e| panic!("cannot execute {tool}: {e}"));
    assert!(status.success(), "{tool} failed with status {status}");
}

fn hip_gcc_install_dir() -> Option<PathBuf> {
    if let Some(path) = env::var_os("HIP_GCC_INSTALL_DIR") {
        return Some(path.into());
    }

    let cxx = env::var_os("CXX").unwrap_or_else(|| "c++".into());
    let output = Command::new(cxx).arg("-print-libgcc-file-name").output().ok()?;
    if !output.status.success() {
        return None;
    }

    let libgcc = PathBuf::from(String::from_utf8(output.stdout).ok()?.trim());
    let directory = libgcc.parent()?;
    directory
        .join("../../../../include/c++")
        .is_dir()
        .then(|| directory.into())
}

fn add_sdk_library_path(environment: &str, default_root: &str, directories: &[&str]) {
    let root = env::var_os(environment)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(default_root));

    if let Some(directory) = directories
        .iter()
        .map(|directory| root.join(directory))
        .find(|directory| directory.is_dir())
    {
        println!("cargo:rustc-link-search=native={}", directory.display());
    }
}

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:rerun-if-env-changed=CXX");
    println!("cargo:rerun-if-env-changed=HIP_GCC_INSTALL_DIR");
    println!("cargo:rerun-if-env-changed=CUDA_PATH");
    println!("cargo:rerun-if-env-changed=ROCM_PATH");

    add_sdk_library_path(
        "CUDA_PATH",
        "/usr/local/cuda",
        &["lib64", "targets/x86_64-linux/lib"],
    );
    add_sdk_library_path("ROCM_PATH", "/opt/rocm", &["lib", "lib64"]);

    {
        let object = out.join("heat_cuda.o");
        let library = out.join("libheat_cuda.a");
        let mut nvcc = Command::new("nvcc");
        nvcc.args(["-c", "kernels/heat_cuda.cu", "-o"]).arg(&object).args([
            "-O2",
            "--compiler-options",
            "-fPIC",
        ]);
        run(nvcc, "nvcc");
        let mut ar = Command::new("ar");
        ar.arg("crus").arg(&library).arg(&object);
        run(ar, "ar");
        println!("cargo:rustc-link-search=native={}", out.display());
        println!("cargo:rustc-link-lib=static=heat_cuda");
        println!("cargo:rustc-link-lib=cudart");
        println!("cargo:rerun-if-changed=kernels/heat_cuda.cu");
    }

    {
        let object = out.join("heat_rocm.o");
        let library = out.join("libheat_rocm.a");
        let mut hipcc = Command::new("hipcc");
        if let Some(directory) = hip_gcc_install_dir() {
            hipcc.arg(format!("--gcc-install-dir={}", directory.display()));
        }
        hipcc
            .args(["-c", "kernels/heat_rocm.hip", "-o"])
            .arg(&object)
            .args(["-O2", "-fPIC"]);
        run(hipcc, "hipcc");
        let mut ar = Command::new("ar");
        ar.arg("crus").arg(&library).arg(&object);
        run(ar, "ar");
        println!("cargo:rustc-link-search=native={}", out.display());
        println!("cargo:rustc-link-lib=static=heat_rocm");
        println!("cargo:rustc-link-lib=amdhip64");
        println!("cargo:rerun-if-changed=kernels/heat_rocm.hip");
    }
    println!("cargo:rustc-link-lib=stdc++");
}
