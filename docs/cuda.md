# Experimental CUDA frontend

The `cuda-in` feature parses a restricted subset of CUDA kernel source directly
into Naga IR. The resulting module uses the existing WGSL, SPIR-V, GLSL, HLSL,
and Metal backends. It does not require an NVIDIA driver or CUDA toolkit.
The initial implementation accepts the source subset described below.

```cuda
extern "C" __global__ void add(const float *a, const float *b, float *out) {
    unsigned int i = blockIdx.x * blockDim.x + threadIdx.x;
    out[i] = a[i] + b[i] * 2.0f;
}
```

Convert this kernel with:

```sh
cargo run -p naga-cli -- input.cu output.wgsl --cuda-workgroup-size 64,1,1
```

The library entry point is `naga::front::cuda::parse_str(source, &options)`, where
`naga::front::cuda::Options::workgroup_size` supplies the CUDA launch block shape.
The default is `[1, 1, 1]`; each dimension must be in `1..=16384`, and their
product must fit in `u32`. Validate the returned module with
`naga::valid::Validator` before passing it to a backend, as with other frontends.
Device and backend resource limits still apply.

## Kernel interface

Each source contains one `__global__ void` kernel, optionally preceded by
`extern "C"`. Parameters are flat pointers to `float`, `int`, or `unsigned int`
(`unsigned` is also accepted). Parameter number `i` becomes a storage buffer at
group 0, binding `i`, with a runtime array of the corresponding 32-bit scalar.
A `const` pointee produces a read-only buffer; other buffers permit reads and
writes. The caller supplies these buffers and dispatches the resulting compute
entry point by its original kernel name.

The frontend maps CUDA's built-in variables as follows:

| CUDA expression | Naga representation |
| --- | --- |
| `threadIdx.x`, `.y`, `.z` | Component of `LocalInvocationId` |
| `blockIdx.x`, `.y`, `.z` | Component of `WorkGroupId` |
| `gridDim.x`, `.y`, `.z` | Component of `NumWorkGroups` |
| `blockDim.x`, `.y`, `.z` | Component of `Options::workgroup_size` |

CUDA chooses block dimensions at launch. Naga compute shaders encode their
workgroup shape in the module, so compile a separate module for each required
block shape. `__launch_bounds__` does not specify that shape and is unsupported.
These mappings follow NVIDIA's [built-in variable definitions](https://docs.nvidia.com/cuda/archive/13.0.0/cuda-c-programming-guide/index.html#built-in-variables)
and WGSL's [compute built-in inputs](https://gpuweb.github.io/gpuweb/wgsl/#builtin-inputs-outputs).

### HLSL dispatch dimensions

HLSL needs a host-supplied constants buffer to implement `gridDim`. Select its
constant-buffer register and register space explicitly:

```sh
cargo run -p naga-cli -- input.cu output.hlsl --hlsl-special-constants-binding 0,1
```

This example uses `b0, space1`. At dispatch, supply the three workgroup counts
as 32-bit values at byte offsets 0, 4, and 8. The library equivalent is
`naga::back::hlsl::Options::special_constants_binding`. Kernels that use
`gridDim` produce an HLSL error if this binding is missing. Kernels using only
`threadIdx`, `blockIdx`, and `blockDim` do not need this buffer.

## Source subset

Supported expressions include indexed buffer reads, initialized scalar locals,
parentheses, and same-type binary `+`, `-`, `*`, and `/`. Statements include local
declarations, assignments, indexed buffer stores, and a final `return;`.
Line and block comments are accepted. Expression nesting and generated expression
depth are bounded; line continuations and trigraphs are unsupported.

Decimal integer literals without a suffix are `int`; the `u`/`U` suffix makes
them `unsigned int`. Floating literals require the `f`/`F` suffix, such as
`2.0f`. Implicit numeric conversions are not implemented: mixed-type arithmetic
and assignments produce errors. Use matching types in this initial subset.

Unsupported syntax produces a parse error with a source span. This includes
preprocessor directives and includes, host code and launch syntax, scalar
parameters, general pointer operations, device functions and calls, control
flow, shared memory, atomics, synchronization, templates, double precision,
and inline PTX. No CUDA runtime ABI or NVIDIA device code is generated.

## Further increments

Add each capability with a failing acceptance test and rejection tests before
changing the parser or lowering. Useful next increments are scalar launch
parameters and guarded indexing, then structured control flow, explicit casts,
device functions, shared storage and barriers. Each increment must preserve
validation and backend compatibility; CUDA-specific features need an explicit
portable representation or an unsupported-feature diagnostic.
