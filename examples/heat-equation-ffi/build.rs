use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

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

fn sdk_library_dir(environment: &str, default_root: &str, directories: &[&str]) -> Option<PathBuf> {
    let root = env::var_os(environment)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(default_root));
    directories
        .iter()
        .map(|directory| root.join(directory))
        .find(|directory| directory.is_dir())
}

fn profile_dir(out: &Path) -> &Path {
    // target/{profile}/build/{package-hash}/out
    out.ancestors().nth(3).expect("unexpected Cargo OUT_DIR")
}

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let destination = profile_dir(&out);
    println!("cargo:rerun-if-env-changed=CXX");
    println!("cargo:rerun-if-env-changed=HIP_GCC_INSTALL_DIR");
    println!("cargo:rerun-if-env-changed=CUDA_PATH");
    println!("cargo:rerun-if-env-changed=ROCM_PATH");

    let cuda_object = out.join("heat_cuda.o");
    let mut nvcc = Command::new("nvcc");
    nvcc.args(["-c", "kernels/heat_cuda.cu", "-o"]).arg(&cuda_object).args([
        "-O2",
        "--compiler-options",
        "-fPIC",
    ]);
    run(nvcc, "nvcc");
    let mut cuda_linker = Command::new(env::var_os("CXX").unwrap_or_else(|| "c++".into()));
    cuda_linker
        .args(["-shared", "-o"])
        .arg(destination.join("libheat_cuda.so"))
        .arg(&cuda_object)
        .arg("-lcudart");
    if let Some(directory) = sdk_library_dir(
        "CUDA_PATH",
        "/usr/local/cuda",
        &["lib64", "targets/x86_64-linux/lib"],
    ) {
        cuda_linker.arg(format!("-L{}", directory.display()));
    }
    run(cuda_linker, "C++ linker for CUDA plugin");
    println!("cargo:rerun-if-changed=kernels/heat_cuda.cu");

    let mut hipcc = Command::new("hipcc");
    if let Some(directory) = hip_gcc_install_dir() {
        hipcc.arg(format!("--gcc-install-dir={}", directory.display()));
    }
    let rocm_object = out.join("heat_rocm.o");
    hipcc
        .args(["-c", "kernels/heat_rocm.hip", "-o"])
        .arg(&rocm_object)
        .args(["-O2", "-fPIC"]);
    run(hipcc, "hipcc");
    let mut rocm_linker = Command::new(env::var_os("CXX").unwrap_or_else(|| "c++".into()));
    rocm_linker
        .args(["-shared", "-o"])
        .arg(destination.join("libheat_rocm.so"))
        .arg(&rocm_object)
        .args(["-lamdhip64", "-lstdc++"]);
    if let Some(directory) = sdk_library_dir("ROCM_PATH", "/opt/rocm", &["lib", "lib64"]) {
        rocm_linker.arg(format!("-L{}", directory.display()));
    }
    run(rocm_linker, "C++ linker for ROCm plugin");
    println!("cargo:rerun-if-changed=kernels/heat_rocm.hip");
}
