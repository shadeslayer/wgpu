# Experimental CUDA frontend

The `cuda-in` feature parses a restricted subset of CUDA kernel source directly
into Naga IR. The resulting module uses the existing WGSL, SPIR-V, GLSL, HLSL,
and Metal backends. It does not require an NVIDIA driver or CUDA toolkit.
The initial implementation accepts the source subset described below.

```cuda
extern "C" __global__ void add(const float *a, const float *b, float *out, unsigned int n) {
    unsigned int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) {
        out[i] = a[i] + b[i] * 2.0f;
    }
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
`extern "C"`. Parameter types are `float`, `int`, or `unsigned int` (`unsigned`
is also accepted), either by value or as flat pointers. Parameter number `i`
uses group 0, binding `i`, preserving source order across both parameter kinds.
The caller supplies these buffers and dispatches the resulting compute entry
point by its original kernel name.

Pointer parameters become storage buffers with runtime arrays of the corresponding
32-bit scalar. A `const` pointee produces a read-only buffer; other buffers permit
reads and writes. Scalar parameters use 16-byte uniform buffers containing the
32-bit value at byte offset 0. Each invocation copies the value into a local
variable; assigning to a non-`const` parameter changes only that invocation's copy.

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
parentheses, arithmetic and comparisons with scalar conversions, and the logical operators
`!`, `&&`, and `||`. Logical operators preserve short-circuit evaluation. Numeric
conditions are false when equal to zero; boolean locals and `true`/`false`
literals are supported.

Statements include local declarations, expression statements, scoped blocks,
`if`/`else`, `for`, `while`, `do`/`while`, `break`, `continue`, and `return;`.
Loop conditions are scalar expressions; `for` initializers can declare one
initialized scalar or evaluate an expression. Empty `for` clauses are supported.
`continue` executes a `for` loop's update before testing the condition again,
and checks a `do`/`while` condition before the next iteration. A `for` initializer
is scoped to the loop. Nested blocks support local shadowing. Statements after a
guaranteed jump are still checked for source errors and then omitted.
Line and block comments are accepted. Expression nesting and generated expression
depth are bounded; line continuations and trigraphs are unsupported.

Decimal integer literals without a suffix are `int`; the `u`/`U` suffix makes
them `unsigned int`. Floating literals require the `f`/`F` suffix, such as
`2.0f`. Arithmetic promotes booleans to `int`; a `float` operand makes the common
type `float`, and mixed signed/unsigned integer operands use `unsigned int`.
Assignments and initializers convert to the destination type. C-style casts,
`static_cast`, and single-token functional casts such as `int(value)` are
supported for these scalar types. Float-to-integer conversions truncate toward
zero within the destination range; unsigned negation wraps modulo 2^32.

Assignments are expressions and associate right to left. Compound assignments
`+=`, `-=`, `*=`, and `/=` evaluate the indexed destination once; both prefix
and postfix `++`/`--` are supported on mutable numeric values. Assignment evaluates
its right operand before its left operand. Logical short-circuiting also applies
to assignments and updates inside the right operand.

Unsupported syntax produces a parse error with a source span. This includes
preprocessor directives and includes, host code and launch syntax,
general pointer operations, device functions and calls, switches,
shared memory, atomics, synchronization, templates, double precision,
and inline PTX. No CUDA runtime ABI or NVIDIA device code is generated.

## Further increments

Add each capability with a failing acceptance test and rejection tests before
changing the parser or lowering. Useful next increments are device functions,
shared storage and barriers. Each increment must preserve
validation and backend compatibility; CUDA-specific features need an explicit
portable representation or an unsupported-feature diagnostic.
