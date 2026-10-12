<!-- GENERATED from keccak_permutation/fips202@1.0.0. Do not edit — `--update` rewrites it. -->
# Keccak-f[1600] exactly as FIPS 202 defines it

An answer to `keccak_permutation/fips202@1.0.0`, in rust.

## What you write

`src/solve.rs`, and nothing else.

```
init(Point)         -> state     setup. Not measured.
run(state, Inputs)  -> Outputs   the task. Measured, and only this.
free(state)                      optional.
```

`init` never sees the data, so there is nothing for it to compute ahead of
time. Reading the case and writing the answer happen outside the measured
call, in generated code — which is why seeing the inputs early is not a way
to answer early.

## The shapes

- `state` — tensor<N x 25 x u64>
- returns `out` — tensor<N x 25 x u64>

## Generated, and not yours

Every file but the one above carries a header saying so. They are rewritten
by `fherma implementation init --update`, which never touches yours.
