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
`extern "C"`. Device helper declarations and definitions can surround the kernel.
Parameter types are `float`,
`int`, or `unsigned int` (`unsigned` is also accepted), either by value or as
flat pointers. Pointer parameters also accept `float2`, `float3`, `float4`,
`int2`, `int3`, `int4`, `uint2`, `uint3`, and `uint4`. Aliases and pointee
`const` qualification preserve the underlying element type.
Pointer parameters also accept completed named structs described below.
Parameter number `i` uses group 0, binding `i`, preserving source
order across both parameter kinds.
The caller supplies these buffers and dispatches the resulting compute entry
point by its original kernel name.

Pointer parameters become storage buffers with runtime arrays of the corresponding
element type. A `const` pointee produces a read-only buffer; other buffers permit
reads and writes. Element layouts are:

| Element type | Size | Alignment | Array stride |
| --- | --- | --- | --- |
| 32-bit scalar | 4 bytes | 4 bytes | 4 bytes |
| Two-component vector | 8 bytes | 8 bytes | 8 bytes |
| Three-component vector | 12 bytes | 4 bytes | 12 bytes |
| Four-component vector | 16 bytes | 16 bytes | 16 bytes |

Struct fields use their element alignment, with padding before a field when
needed. The struct's alignment is its greatest field alignment, and its size
rounds up to that alignment. Arrays of structs use that rounded size as their
stride. Copies preserve field values; padding bytes are unspecified.

Vector elements support whole-value copies and scalar field access, including
assignments and updates to writable fields. Vector parameters require pointers.
Scalar parameters use 16-byte uniform buffers containing the
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

Inputs referenced by helper definitions are retained even when those helpers
are unused. An unused helper that reads `gridDim` can therefore require this
HLSL constants binding. Helper builtin reads do not add kernel buffer bindings.

## Device helpers

`__device__` functions can take `float`, `int`,
`unsigned int`, or `bool` scalars, or the vector and struct value types described below,
and return a supported value or `void`. Helpers can use
local declarations, arithmetic, control flow, and calls to previously declared helpers:

```cuda
__device__ unsigned global_index();
__device__ float scale(float, float);

__global__ void apply(float *out, const float *input) {
    unsigned i = global_index();
    out[i] = scale(input[i], 2.0f);
}
__device__ unsigned global_index() {
    return blockIdx.x * blockDim.x + threadIdx.x;
}
__device__ float scale(float value, float factor) {
    value *= factor;
    return value;
}
```

Prototypes end with `;` and can omit parameter names. Compatible declarations
can repeat before or after a definition; parameter names, aliases, and top-level
by-value parameter `const` do not distinguish signatures. Reference parameters
retain their referent's constness. The definition determines its
parameter names and constness. Conflicting result types and duplicate definitions
produce errors. These rules follow C++ [function declarations](https://eel.is/c++draft/dcl.fct).

Each called helper needs a definition in the same source. An unused prototype
does not emit a function. Missing definitions and direct or indirect recursion
are rejected even in unused helpers or discarded source statements. A prototype
for an existing intrinsic preserves its builtin implementation; a conflicting
result or a source definition of the builtin signature produces an error.

Helpers can read `.x`, `.y`, and `.z` of `threadIdx`, `blockIdx`, `blockDim`,
and `gridDim`. Each component is a read-only 32-bit unsigned integer and has
the same invocation or dispatch value as a read in the kernel. `blockDim`
comes from `Options::workgroup_size`; none of these reads is a C++ constant
expression, including `blockDim`.

Helpers may also carry `__host__`. Translation checks and emits their device
implementation, including calls to previously declared device-only helpers; the host
implementation is not checked or emitted. Host-only definitions remain unsupported.
Repeated source declarations must agree on whether the helper also has `__host__`.

The frontend evaluates arguments left to right. By-value arguments convert to the
parameter types and are copied before later arguments execute. Assignments affect
only those copies; `const` parameters cannot be assigned. Return values convert to the declared
result type. Value-returning helpers must explicitly return on every supported
path; a return inside a loop alone does not satisfy this conservative check.

Void helpers support bare `return;`, implicit fallthrough, and
`return other_void_helper(...);`. Void calls are accepted as discarded
expressions, including parenthesized calls and `for` initializer/update clauses.
Using a void call where a value is required produces a parse error.

Helpers can share a name when their parameter types or counts differ.
Calls prefer exact argument types, then promotion from `bool` to `int`, then
other scalar conversions. The selected overload must be no worse for every
argument and better for at least one; otherwise the call is ambiguous.
Return types, aliases, and top-level parameter `const` do not distinguish
by-value signatures. Mutable and const reference parameters distinguish overloads.
When both candidates bind references, a mutable lvalue prefers `T&` over
`const T&`. An equally ranked by-value candidate and reference candidate remain
ambiguous. Each call uses only the overloads visible at that point in the source;
later declarations do not change its selection. These rules follow
C++ [conversion ranking](https://eel.is/c++draft/over.ics.rank) and
[overload selection](https://eel.is/c++draft/over.match.best.general).

Helpers accept `T&` and `const T&` for the scalar, vector, and struct types above.
A mutable reference requires a writable lvalue of exactly the same type. A const
reference keeps a live alias to a same-type lvalue; a supported scalar conversion
or typed temporary instead creates a private snapshot for that call. Arguments
can refer to local, shared, or buffer storage, including overlapping objects:

```cuda
__device__ void combine(int &first, int &second) {
    first = 5;
    second += 2;
    first += second;
}
__global__ void aliases(int *out) {
    int value = 1;
    combine(value, value);
    out[0] = value; // 14
}
```

Each reference argument's address is selected once, before later arguments
execute. Binding an output-only parameter does not read the referenced value.
Reference helpers are expanded into their callers to preserve live aliases across
backends; nested calls, ordinary returns, and early returns keep their source
behavior. This expansion does not make calls constant expressions. Formal
references also retain unknown constant-expression provenance, even when an
actual argument names fixed shared storage. Invalid unused definitions and
discarded calls are still checked before expansion.

Helpers also accept `T*` and `const T*`, including `T const*`, for the same
scalar, vector, and struct types. Pointer arguments borrow local, shared, or
buffer storage. Fixed arrays decay to their first element; `&object` and
`&array[index]` borrow an existing lvalue. Pointee types must match exactly,
and conversion may add `const` but cannot remove it:

```cuda
__device__ void add_next(int *destination, const int *source) {
    destination[0] += source[1];
}
__global__ void update(int *out) {
    int values[3]{2, 3, 5};
    add_next(out, values + 1);
}
```

Pointers preserve live aliases, including overlapping arguments and forwarding
through reference parameters. Selecting an argument's address happens once,
before later arguments execute, without reading the pointee. Pointer helpers
use the same caller expansion as reference helpers.

Borrowed expressions support `*pointer`, unary `+pointer`, `pointer + index`,
`index + pointer`, `pointer - index`, and both subscript orders. Indices must
have integral types; `bool` promotes to `int`. Parentheses and comma expressions
preserve pointer values, and conditional expressions select compatible views,
adding pointee `const` when either arm requires it. Offsets retain the original
array or individual object's extent. Forming a one-past address does not access
storage, so `*(&values[3] - 1)` can read the final element of a three-element
array. Accesses must remain within the original object and the existing 32-bit
buffer-index domain.

Pointer member access, `pointer->field`, uses the same dereference and field
rules as `(*pointer).field`. It supports struct fields and individual numeric
vector components, including chained fields, array subscripts, assignments,
updates, and reference arguments. The pointer operand is evaluated once, and
pointee `const` remains effective throughout the member chain. An array of
structs or vectors can decay to its first element for member access.
Overloaded `operator->` and pointer-to-member operators remain unsupported.

Local pointer declarations require an initializer: `T *value = source`,
`T *value{source}`, or `T *value = {source}`. They accept the same array decays,
object addresses, pointer expressions, and pointee qualifications as helper
arguments. Initializers evaluate once, so later changes to an index or condition
do not retarget a pointer. Locals preserve live aliases in kernels and helpers,
including mixed declaration lists and block or loop scopes. Pointee `const`
prevents writes; optional top-level pointer `const` does not affect the pointee.
Pointer reassignment, pointer updates, and `constexpr` pointer declarations
remain unsupported.

Single-dimensional helper array parameters, `T values[]` and `T values[N]`,
adjust to `T*`. A written bound must be a positive integral constant expression;
it does not constrain the argument's length. Pointer parameters and adjusted
array parameters share overload signatures. Top-level parameter `const`, as in
`T* const`, does not distinguish signatures; pointee `const` does.

Discarding a bare pointer parameter or local, as in `(pointer, 4)`, does not read
its value. Reading an unknown formal pointer or a local pointer through
indirection, indexing, unary `+`, or offset arithmetic prevents a required constant expression under
the frontend's C++20 constant-expression rules. Caller expansion does not make
those expressions constant.
Constant-address proofs currently cover shared storage; automatic local
addresses cannot establish a required constant expression.
Constant-address evaluation through a named reference currently uses a
one-element extent. Runtime pointer forwarding retains the original array
extent, including neighboring elements.

Indirect parameter expansion has cumulative limits of 1,048,576 generated nodes,
4,194,304 work steps, and 64 MiB of accounted additional owned payload. Generated
expression depth is limited to 64 and generated control depth to 127, accounting
for backend scopes. These limits are checked before copying generated payload;
they do not describe an exact allocator or process memory limit.

Helpers have no resource bindings. Pointer and reference results,
whole-array references, shared declarations, and barriers in helper bodies remain unsupported. Kernel prototypes
remain unsupported.
Pointer members, pointer assignment and updates, null pointers,
pointer comparisons and differences, pointer casts, and general pointer
declarators remain unsupported. Arrays selected from temporary struct values
cannot supply borrowed pointers.

Both helpers and kernels accept `static` and standard `inline` in the function
header. `static` cannot be combined with the direct `extern "C"` prefix.
A helper first declared `static` retains that linkage when later declarations
omit the specifier. Adding `static` after an external declaration is rejected.
Device helpers, including dual-qualified helpers, also accept `__forceinline__`
or `__noinline__`. These CUDA specifiers cannot be combined with each other or
with standard `inline`, including across compatible helper redeclarations,
and cannot be applied to kernels. Their optimization
behavior depends on the target backend and compiler; forced inlining or preserved
calls are not guaranteed. Functions and calls use the existing Naga lowering.
These constraints follow NVIDIA's [inlining rules](https://docs.nvidia.com/cuda/archive/13.0.0/cuda-c-programming-guide/index.html#noinline-and-forceinline)
and C++ [linkage rules](https://eel.is/c++draft/dcl.link#8).
Execution-space, function, return-type, and `const` specifiers may appear in any
valid order; `unsigned` and `int` may be interleaved with those specifiers.
Static variables and the `__inline_hint__` specifier remain unsupported.
Top-level return `const`, including `const void`, does not change the result type.
The frontend preserves ordinary value-only function calls; `inline` does not
force substitution. Indirect parameter helpers require expansion even with `__noinline__`.

## Mathematical functions

`min` and `max` accept `int` and `unsigned int` operands in either order.
Boolean arguments promote to `int`. If either operand is unsigned, both values
convert to `unsigned int` and the result is unsigned; otherwise the result is
`int`. For example, `min(-1, 2u)` is `2u`, while `max(-1, 2u)` is `0xffffffffu`.
These rules follow NVIDIA's [integer overloads](https://docs.nvidia.com/cuda/archive/13.0.0/cuda-math-api/cuda_math_api/group__CUDA__MATH__INT.html).

The `float, float` overloads of `min` and `max`, and the functions `fminf` and
`fmaxf`, return `float`. `fminf` and `fmaxf` also accept scalar arguments that
convert to `float`, including integers and booleans. Mixed floating-point and
integer calls to `min` or `max` can be ambiguous.

These floating-point functions choose the numeric argument when the other is
NaN, and return a quiet NaN when both are NaNs. The frontend chooses the
canonical bits `0x7fc00000` for that result; CUDA does not specify its payload.
The lowering orders negative zero below positive zero and preserves subnormal
operand bits, following the default PTX
[`min`](https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-min)
and [`max`](https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-max)
rules. Floating `min` and `max` are equivalent to `fminf` and `fmaxf`, as described
in NVIDIA's [single-precision functions](https://docs.nvidia.com/cuda/archive/13.0.0/cuda-math-api/cuda_math_api/group__CUDA__MATH__SINGLE.html).

`fabsf(float)` and the `fabs(float)` overload return the absolute value as
`float`. Integer operations preserve the magnitude of the received representation,
including subnormal inputs. Both signed zeros produce positive zero, both
infinities produce positive infinity, and NaNs produce the canonical quiet NaN
`0x7fc00000`; CUDA leaves the NaN representation unspecified.
These results follow NVIDIA's
[`fabsf` specification](https://docs.nvidia.com/cuda/archive/13.0.1/cuda-math-api/cuda_math_api/group__CUDA__MATH__SINGLE.html).
`fabsf` converts scalar arguments to `float` before taking the absolute value.
Integral arguments to `fabs` select unsupported double-precision overloads;
use `fabsf` or explicitly convert to `float`. Exact user-defined integral
overloads still participate and take precedence over the library templates.
Calls execute at runtime and are not constant expressions.

`sqrtf(float)` returns `float` and accepts scalar arguments that convert to
`float`. Positive finite inputs use correctly rounded, round-to-nearest
ties-to-even results, matching CUDA's default
[`prec-sqrt=true`](https://docs.nvidia.com/cuda/archive/13.0.1/cuda-compiler-driver-nvcc/index.html#prec-sqrt-true-false-prec-sqrt).
The calculation uses integer operations on the input representation, including
subnormal inputs. Both signed zeros and positive infinity retain their bits;
negative nonzero inputs and NaNs produce the canonical quiet NaN `0x7fc00000`.
Calls execute at runtime and are not constant expressions, including calls with
literal arguments. The double-precision `sqrt` and directed-rounding intrinsics
remain unsupported.

`floorf(float)` returns the greatest integral value no greater than its argument,
represented as `float`. Scalar arguments convert to `float` before rounding.
For example, `floorf(-0.25f)` returns `-1.0f`; negative nonzero subnormal inputs
also produce `-1.0f`. Integer operations on the received representation preserve
both signed zeros, both infinities, and large integral values. NaNs produce the
canonical quiet NaN `0x7fc00000`. These results follow NVIDIA's
[`floorf` specification](https://docs.nvidia.com/cuda/archive/13.0.1/cuda-math-api/cuda_math_api/group__CUDA__MATH__SINGLE.html).
Calls execute at runtime and are not constant expressions. Double-precision
`floor` remains unsupported.

`sinf(float)` and `cosf(float)` return `float` and accept scalar arguments that
convert to `float`. Integer range reduction and polynomial evaluation cover
the full finite input range, with results within two ULP of the exact sine or
cosine rounded to `float`. This matches the accuracy reported for NVIDIA's
[single-precision functions](https://docs.nvidia.com/cuda/archive/13.1.0/cuda-programming-guide/05-appendices/mathematical-functions.html).
The calculation uses the received input representation, including subnormals.
`sinf` preserves signed zero, and `cosf` returns `1.0f` for both signed zeros.
Infinities and NaNs produce the canonical quiet NaN `0x7fc00000`.
Calls execute at runtime and are not constant expressions. Double-precision
`sin` and `cos`, fast intrinsics, and `sincosf` remain unsupported.

Target floating-point rules still apply. In particular, WGSL permits
[finite-math assumptions, changes to zero signs, and flushing subnormals](https://gpuweb.github.io/gpuweb/wgsl/#floating-point-evaluation),
so exceptional values are not guaranteed to retain these bits on every target.

These names use the same argument sequencing and overload ranking as device
helpers.
Additional user-defined overloads participate alongside the predefined ones;
definitions repeating a predefined parameter signature are rejected, including
through aliases or top-level `const`. Local values and type names can hide these
functions. Kernels and file-scope type aliases named `min`, `max`, `fminf`,
`fmaxf`, `fabs`, `fabsf`, `sqrtf`, `floorf`, `sinf`, or `cosf` remain unsupported.

## Bit reinterpretation

These predefined functions reinterpret a 32-bit scalar's representation:

| Function | Parameter | Result |
| --- | --- | --- |
| `__float_as_int` | `float` | `int` |
| `__float_as_uint` | `float` | `unsigned int` |
| `__int_as_float` | `int` | `float` |
| `__uint_as_float` | `unsigned int` | `float` |

Each call first converts its argument to the parameter type, then reinterprets
the bits as the result type. For example, `__float_as_uint(1u)` returns
`0x3f800000u`, the representation of `1.0f`. Ordinary casts still perform numeric
conversions. These signatures follow NVIDIA's
[type casting intrinsics](https://docs.nvidia.com/cuda/archive/13.0.0/cuda-math-api/cuda_math_api/group__CUDA__MATH__INTRINSIC__CAST.html).

Calls use the same argument evaluation, overload selection, and name hiding
rules as other predefined functions. Additional user-defined overloads are
accepted; definitions repeating a predefined parameter signature are rejected,
including through aliases or top-level `const`. Kernels and file-scope aliases
cannot use these function names. Other bit reinterpretation intrinsics remain
unsupported.

Floating-point results execute at runtime, including calls with literal bit
patterns for infinity and NaN. Target floating-point rules still apply:
WGSL permits [changes to zero signs, subnormal flushing, and finite-math assumptions](https://gpuweb.github.io/gpuweb/wgsl/#floating-point-evaluation),
so exceptional bit patterns are not guaranteed to survive every target.

## Integer intrinsics

These predefined functions operate on 32-bit integer representations:

| Function | Parameters | Result | Operation |
| --- | --- | --- | --- |
| `__clz` | `int` | `int` | Count leading zero bits; zero returns 32. |
| `__ffs` | `int` | `int` | Return the first set bit's position, starting at 1; zero returns 0. |
| `__popc` | `unsigned int` | `int` | Count set bits. |
| `__brev` | `unsigned int` | `unsigned int` | Reverse all 32 bits. |
| `__mul24` | `int, int` | `int` | Multiply signed 24-bit operands and return the low 32 product bits. |
| `__umul24` | `unsigned int, unsigned int` | `unsigned int` | Multiply unsigned 24-bit operands and return the low 32 product bits. |

Negative `int` arguments use their two's-complement representation. For example,
`__clz(-1)` is 0, `__ffs(-2)` is 2, and `__brev(1u)` is `0x80000000u`.
Arguments convert to the declared parameter type before the operation, so
`__popc(3.75f)` is 2. The signed result of `__popc` is preserved: `__popc(3u) - 3`
is -1. These signatures and results follow NVIDIA's
[integer intrinsics](https://docs.nvidia.com/cuda/archive/13.0.0/cuda-math-api/cuda_math_api/group__CUDA__MATH__INTRINSIC__INT.html).

The multiply functions ignore the upper eight bits of each converted argument.
`__mul24` interprets bit 23 as the sign bit; `__umul24` treats the low 24 bits as
unsigned. Results retain all 32 low product bits, including when the mathematical
product exceeds 32 bits. For example, `__mul24(0x00800000, 1)` is -8388608, while
`__umul24(0x00ffffffu, 0x00ffffffu)` is `0xfe000001u`. This follows NVIDIA's
[24-bit multiply intrinsics](https://docs.nvidia.com/cuda/cuda-math-api/cuda_math_api/group__CUDA__MATH__INTRINSIC__INT.html).
Calls do not provide source constant expressions, including calls with literal
arguments.

These names use the same argument evaluation, overload selection, name hiding,
and predefined-signature restrictions as the bit reinterpretation functions
above. The 64-bit `ll` variants and other integer intrinsics remain unsupported.
Metal output for `__brev` requires MSL version 1.2 or newer.

## Constant expressions

Scalar `constexpr` variables support `bool`, `int`, `unsigned int`, `float`,
and their aliases with an `= expression` or braced initializer. They are read-only and
can appear in blocks, `for` initializers, and at file scope before, between,
or after functions. File-scope variables require `constexpr`; they become Naga
constants without consuming buffer bindings.

For example, `constexpr int Width = 4 + 4;` can provide the extent in both
`int local[Width]{};` and `__shared__ float3 shared[Width];`.
Ordinary `const` integral locals also supply constant values when initialized
by constant expressions. An ordinary `const float` local does not; use
`constexpr float` when its value is needed during compilation.

Constant evaluation supports the existing scalar operators and casts with
C++20 [constant-expression rules](https://timsong-cpp.github.io/cppwp/n4861/expr.const).
Unsigned arithmetic and left shifts wrap; signed
arithmetic overflow, invalid shift counts, division by zero, and out-of-range
float-to-integer conversions cannot provide required constant values.
Floating-point constant evaluation currently requires finite results;
overflow to infinity produces an unsupported-constant diagnostic.

Only evaluated operands contribute to constant-expression eligibility, so
`true ? 3 : runtime_value` and `false && helper()` can be constant expressions.
Every operand is still parsed and type-checked. Discarding a nonvolatile local
variable, as in `(runtime_value, 3)`, does not load its value. Address
computations must still be valid; discarded array elements require an index
within the array, following the
[indirection clarification](https://cplusplus.github.io/CWG/issues/2823.html).
Reading parameters or buffers, modifying variables, and
calling the supported functions cannot supply constant values. Compute
builtins, including `blockDim`, are not C++ constant expressions.

`constexpr` arrays, vectors, structs, functions, and reference declarations remain unsupported.

A declared reference has separate constant-expression restrictions. References
to automatic objects or temporaries cannot supply constant values or be discarded
in a required constant expression. A reference bound to a fixed shared-memory
location can be discarded, as in `(alias, 3)`, when its address selection uses only
constant, in-bounds indices. Reading the shared value remains a runtime operation.
Runtime address choices, indices, and initializer side effects prevent this
discardability. The existing lazy rules still apply to unevaluated operands.
File-scope `constexpr` scalars can supply device values but cannot be bound
directly to references, following CUDA's
[host-object address restrictions](https://docs.nvidia.com/cuda/cuda-programming-guide/05-appendices/cpp-language-support.html#constexpr-variables).
An arithmetic expression or conversion to a different scalar type can instead
initialize a const reference's temporary, as in `const int &value = Width + 0;`
or `const float &converted = Width;`.

## Source subset

Aliases support `typedef` and `using` for `bool`, `int`, `unsigned int`,
`float`, and the vector and struct types below, including alias chains and `const` qualification. For example,
`typedef unsigned int Count, Index;` and `using Sample = const float;` preserve
the underlying type and its layout. Repeating an alias declaration
with the same type and qualification is allowed. Explicit duplicate `const`
qualifiers are rejected; requalifying an already const alias is allowed.

Aliases can appear at file scope before, between, or after functions, and in
blocks and `for` initializers. Local values can hide outer type names, and local
aliases can hide outer values or functions. A `using` alias in a `for` initializer
uses C++23 syntax. Pointer, array, function, and void aliases remain unsupported.

Vector values support `float2`, `float3`, `float4`, `int2`, `int3`, `int4`,
`uint2`, `uint3`, and `uint4`. These types can appear in local variables,
fixed local arrays, kernel buffer elements, and helper parameters and results.
Copies, assignments, and conditional arms require matching vector types. Const
qualification prevents writes to individual fields as well as the complete value.

The fields `.x`, `.y`, `.z`, and `.w` select one scalar component, up to the
vector's width. Field access works on locals, array elements, helper results,
and conditional references. Individual fields use the existing scalar operators;
modifiable fields also support assignments and updates.
`make_float3(x, y, z)` and the corresponding `make_` functions take exactly
one scalar argument per component, with the existing scalar conversions and
left-to-right argument evaluation. Additional user-defined overloads participate
in the same overload resolution as other helpers.

These types follow CUDA's [built-in vector definitions](https://docs.nvidia.com/cuda/archive/13.0.0/cuda-c-programming-guide/index.html#built-in-vector-types).
In particular, three-component arrays have a 12-byte element stride and
4-byte alignment. Vector comparisons, truthiness, indexing, and swizzles remain
unsupported. Vector arithmetic requires a visible source-defined operator.
Vector casts and parenthesized aggregate initialization remain unsupported. By-value kernel
parameters require the scalar types described in the kernel interface.
Typed vector temporaries such as `float2{1.0f, 2.0f}` use the same list
initialization and narrowing rules as vector declarations.

Nonmember binary operators `+`, `-`, `*`, `/`, `%`, `&`, `|`, `^`, `<<`, and `>>`
take two parameters, using the helper value, reference, and pointer binding rules above.
Prefix unary operators `+`, `-`, `~`, `!`, `*`, and
`&` take one. At least one parameter must have a supported vector or struct type;
a pointer to such a type alone does not satisfy this requirement.
For example:

```cuda
__device__ float2 operator+(float2 left, float2 right) {
    return make_float2(left.x + right.x, left.y + right.y);
}
```

Both `first + second` and `operator+(first, second)` select from declarations
visible before the call, using the existing helper overload and conversion
rules, including prototypes with later definitions. Scalar-only expressions
retain the existing built-in operators. Operators can return a supported scalar,
vector, or struct value, or `void`; void results can only
appear where an ordinary void helper call is accepted. Calls remain runtime
expressions and obey the existing helper recursion restrictions.

The frontend evaluates overloaded operands and explicit-call arguments once,
from left to right. This is the frontend's chosen order, not a general CUDA
guarantee: C++ distinguishes
[operator-notation sequencing](https://timsong-cpp.github.io/cppwp/n4861/over.match.oper#2)
from [explicit function calls](https://timsong-cpp.github.io/cppwp/n4861/expr.call#8).
Shift expressions evaluate the left operand before evaluating the right. A
by-value parameter captures the left value, while a reference parameter captures
its address and observes subsequent changes to the object.
Source expressions must still obey the C++ rules for unsequenced side effects.
Source-defined binary `operator&&` and `operator||` also take two parameters,
with at least one supported vector or struct parameter. Both operands are
evaluated once, from left to right, before the helper call; the left value is
copied before right-hand side effects when the selected parameter is by value;
reference parameters retain live aliases. These calls preserve the original operand
types for overload selection and can return any supported helper result, including
`void`. Scalar-only `&&` and `||` retain Boolean results and short-circuiting,
including when unrelated class overloads are visible. This follows the C++ rules
for [operator calls](https://eel.is/c++draft/over.match.oper),
[logical AND](https://eel.is/c++draft/expr.log.and), and
[logical OR](https://eel.is/c++draft/expr.log.or).
Overloaded unary `*` and `&` accept class values, including temporaries, and call
the declared helper. Unary `&` uses the builtin address operation when no source
overload is viable; an ambiguous overload remains an error. An explicit
`operator&(...)` call requires a viable source overload.
Overloaded comparisons, assignments, and updates remain
unsupported, as do member operators, reference results, and templates.

Standalone named struct definitions are accepted at file and block scope:

```cuda
struct Sample {
    unsigned tag;
    float3 position;
    float weights[2];
};

__device__ Sample shift_sample(Sample value) {
    value.position.x += value.weights[0];
    return value;
}
```

Fields can use `float`, `int`, `unsigned int`, the supported vector types,
previously completed structs, and fixed arrays of these types. Field-array
bounds use the existing integral constant-expression rules. Aliases of a struct
preserve its type; separate definitions remain different types even when their
fields match. Local definitions can hide outer type names.

Locals support same-type copies, assignments, and braced initialization,
including partial initialization and brace elision for nested aggregates.
Omitted fields are zero-initialized. Typed expressions such as `Sample{}` can
be passed to helpers, returned, grouped in parentheses, discarded as statements,
or read through their fields. Copying a struct
also copies its fixed-array fields. Standalone arrays still cannot be copied
or passed by value. These rules follow C++
[aggregate initialization](https://eel.is/c++draft/dcl.init.aggr).

Arbitrary field names and indexed fields use the existing value/reference
rules, including `samples[index].weights[element]`. Const objects and
`const Sample*` buffers prevent writes to every nested field. Conditional
struct lvalues support field access. Subscripting an array directly selected by
a conditional expression remains unsupported; first binding the selected struct
to a local reference supports indexing its array members.

The initial record subset rejects empty definitions, `bool` fields, const data
members, pointer/reference members, default member initializers, methods,
inheritance, nested type definitions, unions, and explicit alignment or packing.
Forward declarations and elaborated type spellings such as `struct Sample value`
remain unsupported; use the completed bare type name `Sample`. Struct kernel
parameters require pointers. Aggregate constant expressions and untyped braced
call arguments or returns remain unsupported.

Empty local vector initialization, such as `float3 value{};` or
`const float4 value = {};`, zeroes every field each time execution reaches the
declaration, including inside loops.

Mutable scalar and vector locals can omit an initializer and be assigned before
reading.
Const locals, including const-qualified aliases, require an initializer.
Local declarations can list comma-separated names, as in
`int first = 1, next = first + 1;`. Each initializer finishes before the next
declarator, and all names share the declared type and qualification.

Local `T&` and `const T&` declarations support the scalar, vector, and struct
types above in kernels and device helpers. A mutable reference requires a writable
lvalue of the same type. A const reference keeps a live view of a compatible
lvalue; it can also retain a temporary value, using existing scalar conversion
and aggregate initialization rules. Assignment through a reference changes the
bound object. The address is selected once when the declaration executes,
including conditional branches and array-index side effects:

```cuda
int values[2]{3, 5};
int index = 0;
int &element = values[index++];
const int &view = element;
element += 2;
```

Both `element` and `view` subsequently observe `values[0]`, whose value is `5`.
References require initializers and use the existing block and loop scopes.
Equals, direct-list, and copy-list initializers are accepted; a singleton list
with a matching lvalue binds that object, while const temporary lists apply the
existing narrowing checks. Each comma declarator needs its own `&`, as in
`int &alias = value, copy = alias;`. Reference results, type aliases,
array references, and reference members remain unsupported. Binding an unresolved
reference to itself is also unsupported. Reference lowering has a per-function
work limit of 65,536 units to bound generated address dispatch and projections.

Fixed local arrays support the scalar, vector, and struct types above
and their aliases, in both kernels and device helpers. Written extents
must be positive integral constant expressions. Each invocation owns its array storage;
elements use the same indexing, assignments, and updates as buffer elements.
Arrays and other locals can share a declaration, as in `int values[3], next = 1;`.

An empty initializer, `int values[3]{};` or `int values[3] = {};`, zeroes every
element each time execution reaches the declaration, including inside loops.
Const arrays require an initializer and their elements cannot be modified.
Bare mutable arrays must be assigned before reading. These rules follow C++
[array declarations](https://eel.is/c++draft/dcl.array) and
[aggregate initialization](https://eel.is/c++draft/dcl.init.aggr).
The outermost local array bound may be omitted when a nonempty initializer list
determines the element count, following C++ [array declarations](https://timsong-cpp.github.io/cppwp/n4861/dcl.array#7).
For example, `int values[]{1, 2, 3};` declares three elements.
Vector arrays count complete vector elements after brace elision and copies:
`int3 values[]{1, 2, 3, 4};` declares two elements, `(1, 2, 3)` and `(4, 0, 0)`.
Empty lists cannot determine a bound; `int values[]{};` is rejected, while
`int values[]{{}};` declares one zero element.

The array's bound remains unknown inside its initializer for constant-expression
checks. A discarded self-reference at index zero can qualify, as in
`float values[]{1.0f, (values[0], 2)};`. Nonzero self-indices cannot provide a
constant value there, even when the eventual bound contains them, following the
[unknown-bound clarification](https://cplusplus.github.io/CWG/issues/3004.html).
Ordinary expressions can still read earlier initialized elements at nonzero
indices. After the declaration, constant-address checks use the completed bound.
Multidimensional arrays use nested array types and row-major layout. Every inner
bound must be explicit. Initializers accept nested lists and brace elision;
`int matrix[][2]{{1}, 2, 3, 4};` has three rows: `(1, 0)`, `(2, 3)`, and `(4, 0)`.
Each row's omitted elements become zero before the next row's initializer runs.
Index every dimension to access a scalar or vector element, as in
`matrix[1][0]`. Borrowing a row preserves its own extent; whole-array and row
copies remain unsupported. Multidimensional helper array parameter declarators
remain unsupported. Constant-address checks require each index to fit its own dimension.

Local scalars, vectors, and fixed arrays accept direct-list and copy-list
initialization, including partial lists and a trailing comma:

```cuda
int count{3};
float3 point = {1.0f, 2.0f};
int values[3]{1, values[0] + 1};
int2 pairs[2]{1, 2, 3};
```

Omitted elements become zero when their turn to initialize is reached.
Each element finishes initialization before the next clause is evaluated, so
the example initializes `values` to `1, 2, 0` and `pairs` to `(1, 2), (3, 0)`.
Arrays of vectors accept nested lists, brace elision, and whole-value copies
from matching vector types. A vector list containing one matching vector
expression copies it, as in `int3 copied{original};`.

List initialization follows C++20
[narrowing rules](https://timsong-cpp.github.io/cppwp/n4861/dcl.init.list#7).
Float-to-integer conversions are rejected, including conversions to `bool`.
Integer conversions that cannot represent every source value require a constant
expression whose value fits the destination. Integer-to-float conversions require
a constant expression whose value is represented exactly, including when the
source type is `bool`. Explicit casts can perform an ordinary conversion first.
For example, `float value{runtime_integer};` is rejected, while
`float value{float(runtime_integer)};` is accepted.

Scalar lists contain at most one expression; extra scalar brace nesting is
rejected, following the
[nested-brace clarification](https://cplusplus.github.io/CWG/issues/cwg_closed.html#1501).
Thus `int values[2]{{1}, {}};` is valid and `int value{{1}};` is not.
Typed vector and struct temporaries also use these initialization rules.
Bare initializer lists in assignments, returns, and helper arguments remain
unsupported, as do designated initializers and shared-storage initializers.

Supported expressions include indexed buffer reads, scalar locals,
parentheses, arithmetic and comparisons with scalar conversions, and the logical operators
`!`, `&&`, and `||`. Built-in scalar `&&` and `||` preserve short-circuit evaluation.
Numeric conditions are false when equal to zero; boolean locals and `true`/`false`
literals are supported.

Conditional expressions `condition ? accept : reject` evaluate the condition
once and only evaluate the selected arm. They associate right to left. Matching
scalar or vector lvalue arms preserve their reference and support assignment.
Scalar references and vector fields also support compound assignments and
prefix/postfix updates; a const arm prevents modification.
Discarding a reference preserves its side effects without reading the selected
storage value. Other scalar arms use a common arithmetic type, with matching
boolean arms retaining `bool`. Matching vector arms retain their exact type;
other vector combinations are unsupported. Both arms may be void in discarded
expressions or void returns; mixing void and value arms is unsupported.

Statements include local declarations, expression statements, scoped blocks,
`if`/`else`, `switch`, `for`, `while`, `do`/`while`, `break`, `continue`, and `return`.
At statement start, C++ parses `int(value);` as a declaration. Parenthesized
declarators are unsupported; use `int value = ...;` for a local declaration or
`(int(value));` for a discarded cast expression.
Loop conditions are scalar expressions; `for` initializers can declare scalar
or array locals, declare scalar aliases, or evaluate an expression. Empty
`for` clauses are supported.

Switch selectors accept `int`, `unsigned int`, and `bool`; booleans promote to
`int`. The selector is evaluated once. Case labels require integral constant
expressions that fit the promoted selector type without narrowing. Duplicate
converted values and multiple defaults are rejected, following C++
[switch statements](https://timsong-cpp.github.io/cppwp/n4861/stmt.switch).
The default label can appear anywhere. Execution falls through subsequent labels
until a jump or the end of the body; an unmatched selector without a default
executes no body statements. `break` exits the nearest loop or switch, while
`continue` targets the nearest enclosing loop.

A switch can have a compound body or a single statement, including a leading
case/default label chain. An unlabeled body is checked but never executed.
Declarations share the switch body's scope, so bare locals and aliases can be
used by later cases. Entering a label that bypasses an active local initializer,
including a `constexpr`, array, or reference initializer, is rejected. Initializers inside
closed nested scopes do not block later labels, following C++
[declaration statements](https://timsong-cpp.github.io/cppwp/n4861/stmt.dcl#3).
Nested switches are supported. Switch initialization statements, declaration
conditions, and labels that enter a nested block, branch, or loop remain
unsupported.

Comma expressions evaluate left to right and preserve the final operand's type
and value category, including lvalues and void. Earlier operands are discarded.
Discarded lvalues evaluate their addresses without loading their referenced
storage. Comma sequences are supported in
expression statements, `for` updates, conditions, returns, and the middle arm of
`?:`. Commas separating declarators or helper arguments retain their list meaning.
Use parentheses for a comma expression within one initializer, helper argument,
scalar functional cast, or buffer index, as in `int((a, b))` or `out[(a, b)]`.
`static_cast<int>(a, b)` accepts a full expression. `__syncthreads()` remains a
standalone statement.
`continue` executes a `for` loop's update before testing the condition again,
and checks a `do`/`while` condition before the next iteration. A `for` initializer
is scoped to the loop. Nested blocks support local shadowing. Statements after a
guaranteed jump are still checked for source errors and then omitted.
Line and block comments are accepted. Expression nesting, conditional reference
nesting, and generated expression depth are bounded. Trigraphs are unsupported.

Source preprocessing supports object-like and fixed-arity function-like
`#define`, `#undef`, and conditional groups using `#if`, `#elif`, `#ifdef`,
`#ifndef`, `#elifdef`, `#elifndef`, `#else`, and `#endif`. Macro names use ASCII
identifiers; `defined` and alternative operator spellings cannot be macro names.
Definitions follow source order, and replacement tokens are expanded when used.
Empty replacements and nested conditional groups are accepted. Redefinitions
must preserve the macro kind, ordered parameter names, replacement token spelling,
and internal replacement whitespace separation. Expansion preserves token
boundaries and reports language errors at the original invocation.

Function-like macros follow C++23
[macro replacement](https://timsong-cpp.github.io/cppwp/n4950/cpp.replace).
Their definitions require an adjacent `(` after the name and accept zero or more
unique parameters. Invocations may have whitespace, comments, or ordinary
newlines before `(`. Arguments are collected before expansion; only parentheses
protect commas within an argument. Parameters substitute expanded tokens
textually, so repeated parameters can duplicate evaluation and unused parameters
discard their arguments. Replacement does not add parentheses. Replacement tails
can form further invocations with following source tokens. A directive between
the name and `(` leaves the name unexpanded; directives inside an open invocation
are unsupported.

An initial UTF-8 BOM and LF, CRLF, or CR line endings are accepted. Backslash
line continuations are joined before recognizing comments and tokens, including
optional whitespace before the newline. Quoted and raw literals are opaque
during preprocessing. String and character values remain unsupported in kernel
expressions.
Raw literal bodies preserve backslash-newline pairs, following C++23
[translation phases](https://eel.is/c++draft/lex.phases).

Conditions use signed and unsigned 64-bit preprocessing integers, following
C++23 [conditional inclusion](https://timsong-cpp.github.io/cppwp/n4950/cpp.cond).
Integer bases, separators, and `u`, `l`, `ll`, and `z` suffixes are accepted.
These values select source branches; kernel value types retain the limits below.
Conditions accept arithmetic, shifts, comparisons, bitwise and logical
operators, `?:`, and alternative spellings such as `and` and `not`. Left shifts
wrap, and signed right shifts are arithmetic. Comma expressions are accepted
only inside parentheses and the middle `?:` operand.
Signed arithmetic overflow, zero divisors, and invalid shift counts produce
diagnostics when evaluated. Logical operators and `?:` skip unselected value
operations while still checking their syntax and literal types.

`defined(NAME)` and `defined NAME` query a macro name without expanding its
replacement. A `defined` token returned by macro expansion is unsupported.
Unused function-macro arguments are not expanded or interpreted as conditions.
Other remaining names become zero, except `true` and `false`.
Ordinary single ASCII character constants and simple, octal, or hexadecimal
escapes with values in `0..=127` are accepted in conditions. Other character
forms, floating constants, casts, calls, and assignments remain unsupported.
Only the first selected group is processed. Inactive groups skip ordinary
directives and unevaluated conditions while checking conditional structure.

Variadic macros, stringization, token pasting, includes, predefined macro
providers, universal character names, and digraphs remain unsupported. No host
or device macros are supplied implicitly; source may explicitly define
compiler-specific names such as `__CUDACC__`. Needed provider use, query,
definition, or removal of standard predefined names such as `__LINE__`, `__cplusplus`,
`__has_include`, and `__has_cpp_attribute` produces a diagnostic, including
inside a short-circuited expression. Preparation bounds source size, condition
nesting, expansion depth, stored and emitted tokens, and expansion work.

Kernel integer literals support decimal, leading-zero octal, `0x`/`0X`
hexadecimal, and `0b`/`0B` binary notation. Apostrophes may separate digits, as in
`1'000` or `0xffff'ffff`. Unsuffixed decimal literals must fit `int`; unsuffixed nondecimal
literals use `int` when they fit and otherwise `unsigned int`. The `u`/`U` suffix
selects `unsigned int`. Values requiring more than 32 bits and long or size
suffixes remain unsupported. Decimal floating literals require the `f`/`F` suffix,
such as `2.0f`. Apostrophes may separate decimal digits in the significand and
exponent, as in `1'024.5f` or `1.25e+0'2f`. Hexadecimal floats remain unsupported.
Arithmetic promotes booleans to `int`; a `float` operand makes the common
type `float`, and mixed signed/unsigned integer operands use `unsigned int`.
Scalar assignments and ordinary `= expression` initializers convert to the
destination type; brace initializers use the narrowing rules above. C-style casts,
`static_cast`, and single-token functional casts such as `int(value)` are
supported for these scalar types. Float-to-integer conversions truncate toward
zero within the destination range; unsigned negation wraps modulo 2^32.

Remainder `%`, bitwise `~`, `&`, `|`, and `^`, and shifts `<<` and `>>` accept
integer and boolean operands, with booleans promoted to `int`. Shift operands
are promoted independently, and the result has the promoted left operand's type.
For either shift operator, the left operand is evaluated before the right.
Signed right shifts are arithmetic, matching C++20. Source operations with
undefined C++ behavior have no specified result, including integer division or
remainder by zero, `INT_MIN / -1`, `INT_MIN % -1`, and shift counts outside
`0..=31`.

Assignments are expressions and associate right to left. Compound assignments
`+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`, `^=`, `<<=`, and `>>=` evaluate the
indexed destination once; both prefix and postfix `++`/`--` are supported on
mutable numeric scalars, including vector fields. Assignment evaluates its right
operand before its left operand. Built-in scalar logical short-circuiting also
applies to assignments and updates inside the right operand.

Static `__shared__` declarations support `float`, `int`, and `unsigned int`
scalars, the numeric vector and struct types above, and fixed arrays of
those types inside the kernel. Their aliases are also accepted; const-qualified
shared types remain unsupported. Vector elements use the sizes, alignments, and
array strides listed in the kernel interface, including packed three-component
structures. Whole-value copies and scalar component updates use the same rules
as local vectors. Array extents must
be positive integral constant expressions using the scalar rules above, including
every dimension of a multidimensional array such as `__shared__ float tile[16][17];`.
Shared variables become workgroup storage without resource bindings; names obey
the surrounding block scope, and each workgroup has its own storage. Declarations
cannot contain initializers, following NVIDIA's
[shared memory constraints](https://docs.nvidia.com/cuda/cuda-programming-guide/05-appendices/cpp-language-extensions.html).
Dynamic `extern __shared__` storage remains unsupported.
Accesses to shared values from different invocations require
synchronization when any invocation writes them.

`__syncthreads();` synchronizes a thread block and makes earlier shared and
global buffer accesses visible to other invocations in that block. It lowers to
a control barrier covering both workgroup and storage memory, following
[CUDA's synchronization semantics](https://docs.nvidia.com/cuda/archive/13.0.0/cuda-c-programming-guide/index.html#synchronization-functions).
The first implementation accepts this zero-argument statement only in
straight-line kernel flow, including unconditional nested blocks. Barriers
inside `if` branches, switch cases, or loops, or after a possible earlier `return`,
produce a parse error. This conservative restriction rejects some valid CUDA programs
until control-barrier uniformity analysis is available. Conditions and loops
without returns may precede a barrier; returns after the last barrier are
allowed. Synchronization does not extend across workgroups.

Unsupported syntax produces a parse error with a source span. This includes
includes and other preprocessing forms outside the subset above, host code and launch syntax,
pointer operations outside the borrowed subset above,
dynamic shared memory, atomics, other synchronization primitives, templates,
double precision, and inline PTX. No CUDA runtime ABI or NVIDIA device code is
generated.

## Further increments

Add each capability with a failing acceptance test and rejection tests before
changing the parser or lowering. Useful next increments are more device-function
signatures, dynamic shared storage, and more general barrier placement. Each
increment must preserve validation and backend compatibility; CUDA-specific
features need an explicit portable representation or an unsupported-feature
diagnostic.
