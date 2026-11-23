# Micro Tensor Engine — Benchmark `platform-aware` vs. Abordagens Estáticas

Micro motor de tensores `f32` autocontido projetado para avaliar **portabilidade de desempenho**, **modularidade** e **overhead de despacho** da abordagem estruturada `platform-aware` frente a técnicas estáticas e *ad hoc* em Rust.

## Operações implementadas (`TensorOps`)

Todas as abordagens executam os **mesmos kernels numéricos** ([src/kernels.rs](src/kernels.rs)), garantindo comparação justa:

| Kernel | Operação | Perfil predominante |
| :--- | :--- | :--- |
| `dot` | Produto interno $\sum_i a_i b_i$ | Memória + redução SIMD |
| `axpy` | $y \leftarrow y + \alpha x$ | Largura de banda + FMA |
| `relu` | $\max(0, x)$ | Elementwise *branchless* SIMD |
| `matvec` | $y = A x$ (2D × 1D) | Memória + paralelismo por linhas |
| `matmul` | $C = A B$ (2D × 2D, GEMM com *cache blocking* + micro-kernel 4×16) | Intensivo em computação (GFLOP/s) |

## Abordagens comparadas

1. **`platform-aware` ([src/platform_engine/](src/platform_engine/mod.rs))**:
   - Usa `#[platformaware]`, *k-methods* no trait `TensorOps` e *platform modules* externos (`platform!(cpu_simd);` e `platform!(cpu_multicore);`, Seção 4.7 do artigo).
   - **Um único binário portátil** seleciona em tempo de execução entre `scalar` (*fallback*), `sse42`, `avx2`, `avx512`, `multicore_scalar`, `multicore_avx2` e `multicore_avx512` conforme o `Platform.toml`.
2. **`static-cfg` ([src/static_engine.rs](src/static_engine.rs))**:
   - Compilação condicional estática idiomática (`#[cfg(target_feature = "...")]` + feature `--features static-multicore`, Seção 2.3).
   - No build portátil padrão fica restrito ao `scalar`; com `RUSTFLAGS="-C target-cpu=native" --features static-multicore` atinge desempenho nativo, mas perde portabilidade binária.
3. **`static-direct` ([src/static_engine.rs](src/static_engine.rs))**:
   - Invocação direta do mesmo kernel selecionado pelo `platform-aware`, isolando o *overhead* puro de resolução/despacho (Seção 5.3).
4. **`ad-hoc` ([src/static_engine.rs](src/static_engine.rs))**:
   - Detecção em tempo de execução espalhada dentro de cada operação (`is_x86_feature_detected!` + `available_parallelism`, Seções 2.1 e 3.2).

## Como executar

### 1. Testes unitários (verificação numérica contra referência escalar)
```bash
cargo test --offline
```

### 2. Benchmark individual com um perfil `Platform.toml`
```bash
cargo build --release --offline
PLATFORM_DESCRIPTION=./profiles/multicore_avx2 ./target/release/micro-tensor-engine --size medium --iters 15
```

Opções da CLI:
- `--size small|medium|large` (padrão: `medium`)
- `--warmup N` (padrão: `3`)
- `--iters N` (padrão: `10`)
- `--overhead-calls N` (padrão: `1000000`)
- `--csv` (saída tabular em CSV para planilhas/gráficos)
- `TENSOR_THREADS=N` (sobrescreve o número de threads nas variantes multicore)

### 3. Comparação completa (binário portátil único em 5 perfis vs. binário estático nativo)
```bash
bash ./bench_compare.sh medium 10
```
