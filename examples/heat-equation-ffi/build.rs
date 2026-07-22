use std::{env, path::PathBuf, process::Command};

fn run(mut command: Command, tool: &str) {
    let status = command.status().unwrap_or_else(|e| panic!("cannot execute {tool}: {e}"));
    assert!(status.success(), "{tool} failed with status {status}");
}

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

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
}
