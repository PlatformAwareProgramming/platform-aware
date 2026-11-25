//! Isolated binary using ONLY the runtime `AdHocRuntimeEngine` (`is_x86_feature_detected!`).
//! Compiled with `--no-default-features` (no `platform-aware` crates in the dependency graph).

use std::hint::black_box;

use micro_tensor_engine::{AdHocRuntimeEngine, Tensor};

fn dims(size: &str) -> (usize, usize, usize, usize) {
    match size {
        "small" => (262_144, 1024, 128, 128),
        "large" => (8_388_608, 4096, 768, 768),
        _ => (4_194_304, 2048, 512, 512),
    }
}

fn main() {
    let mut size = "medium".to_string();
    let mut op = "all".to_string();
    let mut print_backend = false;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--size" => {
                if let Some(v) = args.next() {
                    size = v;
                }
            }
            "--op" => {
                if let Some(v) = args.next() {
                    op = v;
                }
            }
            "--backend" => print_backend = true,
            _ => {}
        }
    }

    let (vec_len, mv, mm, _) = dims(&size);
    let backend = AdHocRuntimeEngine::active_backend();
    if print_backend {
        println!("engine=ad-hoc backend={backend}");
        return;
    }

    match op.as_str() {
        "dot" => {
            let v1 = Tensor::random(&[vec_len], 11);
            let v2 = Tensor::random(&[vec_len], 22);
            black_box(AdHocRuntimeEngine::dot(black_box(&v1), black_box(&v2)));
        }
        "axpy" => {
            let v1 = Tensor::random(&[vec_len], 11);
            let mut v2 = Tensor::random(&[vec_len], 22);
            AdHocRuntimeEngine::axpy(0.25, black_box(&v1), black_box(&mut v2));
            black_box(&v2);
        }
        "relu" => {
            let v1 = Tensor::random(&[vec_len], 11);
            black_box(AdHocRuntimeEngine::relu(black_box(&v1)));
        }
        "matvec" => {
            let a = Tensor::random(&[mv, mv], 33);
            let x = Tensor::random(&[mv], 44);
            black_box(AdHocRuntimeEngine::matvec(black_box(&a), black_box(&x)));
        }
        "matmul" => {
            let a = Tensor::random(&[mm, mm], 55);
            let b = Tensor::random(&[mm, mm], 66);
            black_box(AdHocRuntimeEngine::matmul(black_box(&a), black_box(&b)));
        }
        _ => {
            let v1 = Tensor::random(&[vec_len], 11);
            let mut v2 = Tensor::random(&[vec_len], 22);
            let a_mv = Tensor::random(&[mv, mv], 33);
            let x_mv = Tensor::random(&[mv], 44);
            let a_mm = Tensor::random(&[mm, mm], 55);
            let b_mm = Tensor::random(&[mm, mm], 66);

            black_box(AdHocRuntimeEngine::dot(black_box(&v1), black_box(&v2)));
            AdHocRuntimeEngine::axpy(0.25, black_box(&v1), black_box(&mut v2));
            black_box(AdHocRuntimeEngine::relu(black_box(&v1)));
            black_box(AdHocRuntimeEngine::matvec(black_box(&a_mv), black_box(&x_mv)));
            black_box(AdHocRuntimeEngine::matmul(black_box(&a_mm), black_box(&b_mm)));
        }
    }
}
