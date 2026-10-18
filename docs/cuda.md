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
`extern "C"`, after any device helper definitions. Parameter types are `float`,
`int`, or `unsigned int` (`unsigned` is also accepted), either by value or as
flat pointers. Pointer parameters also accept `float2`, `float3`, `float4`,
`int2`, `int3`, `int4`, `uint2`, `uint3`, and `uint4`. Aliases and pointee
`const` qualification preserve the underlying element type.
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

## Device helpers

`__device__` functions defined before the kernel can take `float`, `int`,
`unsigned int`, or `bool` scalars, or the vector value types described below,
and return a supported value or `void`. Helpers can use
local declarations, arithmetic, control flow, and calls to earlier helpers:

```cuda
__device__ float scale(float value, float factor) {
    value *= factor;
    return value;
}
__global__ void apply(float *out, const float *input) {
    unsigned i = threadIdx.x;
    out[i] = scale(input[i], 2.0f);
}
```

Helpers may also carry `__host__`. Translation checks and emits their device
implementation, including calls to earlier device-only helpers; the host
implementation is not checked or emitted. Host-only definitions remain unsupported.

The frontend evaluates arguments left to right, converts them to the parameter
types, and copies them into per-call locals. Assignments affect only those copies;
`const` parameters cannot be assigned. Return values convert to the declared
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
signatures. Calls use the overloads visible at their definition point, and a
call selecting the current helper is rejected as recursion. These rules follow
C++ [conversion ranking](https://eel.is/c++draft/over.ics.rank) and
[overload selection](https://eel.is/c++draft/over.match.best.general).

Helpers have no resource bindings. Prototypes, recursion, forward
calls, pointers, references, shared storage, barriers, and
compute builtins in helpers remain unsupported.

Both helpers and kernels accept `static` and standard `inline` in the function
header. `static` cannot be combined with the direct `extern "C"` prefix.
Device helpers, including dual-qualified helpers, also accept `__forceinline__`
or `__noinline__`. These CUDA specifiers cannot be combined with each other or
with standard `inline`, and cannot be applied to kernels. Their optimization
behavior depends on the target backend and compiler; forced inlining or preserved
calls are not guaranteed. Functions and calls use the existing Naga lowering.
These constraints follow NVIDIA's [inlining rules](https://docs.nvidia.com/cuda/archive/13.0.0/cuda-c-programming-guide/index.html#noinline-and-forceinline)
and C++ [linkage rules](https://eel.is/c++draft/dcl.link#8).
Execution-space, function, return-type, and `const` specifiers may appear in any
valid order; `unsigned` and `int` may be interleaved with those specifiers.
Static variables and the `__inline_hint__` specifier remain unsupported.
Top-level return `const`, including `const void`, does not change the result type.
The frontend preserves ordinary function calls; `inline` does not force substitution.

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

Target floating-point rules still apply. In particular, WGSL permits
[finite-math assumptions, changes to zero signs, and flushing subnormals](https://gpuweb.github.io/gpuweb/wgsl/#floating-point-evaluation),
so exceptional values are not guaranteed to retain these bits on every target.

All four names use the same argument sequencing and overload ranking as device
helpers.
Additional user-defined overloads participate alongside the predefined ones;
definitions repeating a predefined parameter signature are rejected, including
through aliases or top-level `const`. Local values and type names can hide these
functions. Kernels and file-scope type aliases named `min`, `max`, `fminf`, or
`fmaxf` remain unsupported.

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

| Function | Parameter | Result | Operation |
| --- | --- | --- | --- |
| `__clz` | `int` | `int` | Count leading zero bits; zero returns 32. |
| `__ffs` | `int` | `int` | Return the first set bit's position, starting at 1; zero returns 0. |
| `__popc` | `unsigned int` | `int` | Count set bits. |
| `__brev` | `unsigned int` | `unsigned int` | Reverse all 32 bits. |

Negative `int` arguments use their two's-complement representation. For example,
`__clz(-1)` is 0, `__ffs(-2)` is 2, and `__brev(1u)` is `0x80000000u`.
Arguments convert to the declared parameter type before the operation, so
`__popc(3.75f)` is 2. The signed result of `__popc` is preserved: `__popc(3u) - 3`
is -1. These signatures and results follow NVIDIA's
[integer intrinsics](https://docs.nvidia.com/cuda/archive/13.0.0/cuda-math-api/cuda_math_api/group__CUDA__MATH__INTRINSIC__INT.html).

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
reference, as in `(runtime_value, 3)`, does not load its value. Address
computations must still be valid; discarded array elements require an index
within the array, following the
[indirection clarification](https://cplusplus.github.io/CWG/issues/2823.html).
Reading parameters or buffers, modifying variables, and
calling the supported functions cannot supply constant values. Compute
builtins, including `blockDim`, are not C++ constant expressions.

`constexpr` arrays, vectors, and functions remain unsupported.

## Source subset

Aliases support `typedef` and `using` for `bool`, `int`, `unsigned int`,
`float`, and the vector types below, including alias chains and `const` qualification. For example,
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
4-byte alignment. Vector arithmetic, comparisons, truthiness, indexing, and
swizzles are unsupported; use individual fields. Vector casts and parenthesized
aggregate initialization remain unsupported. By-value kernel parameters require
the scalar types described in the kernel interface.

Empty local vector initialization, such as `float3 value{};` or
`const float4 value = {};`, zeroes every field each time execution reaches the
declaration, including inside loops.

Mutable scalar and vector locals can omit an initializer and be assigned before
reading.
Const locals, including const-qualified aliases, require an initializer.
Local declarations can list comma-separated names, as in
`int first = 1, next = first + 1;`. Each initializer finishes before the next
declarator, and all names share the declared type and qualification.

Fixed one-dimensional local arrays support the scalar and vector types above
and their aliases, in both kernels and device helpers. Extents
must be positive integral constant expressions. Each invocation owns its array storage;
elements use the same indexing, assignments, and updates as buffer elements.
Arrays and other locals can share a declaration, as in `int values[3], next = 1;`.

An empty initializer, `int values[3]{};` or `int values[3] = {};`, zeroes every
element each time execution reaches the declaration, including inside loops.
Const arrays require an initializer and their elements cannot be modified.
Bare mutable arrays must be assigned before reading. These rules follow C++
[array declarations](https://eel.is/c++draft/dcl.array) and
[aggregate initialization](https://eel.is/c++draft/dcl.init.aggr).
Inferred bounds, multidimensional local arrays, and array-to-pointer conversions
remain unsupported.

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
List initialization is currently limited to declarations of these existing
types. Designated initializers, braced temporaries, assignments, returns, and
helper arguments remain unsupported. Shared-storage initializers also remain
unsupported.

Supported expressions include indexed buffer reads, scalar locals,
parentheses, arithmetic and comparisons with scalar conversions, and the logical operators
`!`, `&&`, and `||`. Logical operators preserve short-circuit evaluation. Numeric
conditions are false when equal to zero; boolean locals and `true`/`false`
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
`if`/`else`, `for`, `while`, `do`/`while`, `break`, `continue`, and `return`.
At statement start, C++ parses `int(value);` as a declaration. Parenthesized
declarators are unsupported; use `int value = ...;` for a local declaration or
`(int(value));` for a discarded cast expression.
Loop conditions are scalar expressions; `for` initializers can declare scalar
or array locals, declare scalar aliases, or evaluate an expression. Empty
`for` clauses are supported.
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
nesting, and generated expression depth are bounded; line continuations and
trigraphs are unsupported.

Integer literals support decimal, leading-zero octal, `0x`/`0X` hexadecimal, and
`0b`/`0B` binary notation. Apostrophes may separate digits, as in `1'000` or
`0xffff'ffff`. Unsuffixed decimal literals must fit `int`; unsuffixed nondecimal
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
operand before its left operand. Logical short-circuiting also applies to assignments and updates inside
the right operand.

Static `__shared__` declarations support `float`, `int`, and `unsigned int`
scalars, the numeric vector types above, and fixed one-dimensional arrays of
those types inside the kernel. Their aliases are also accepted; const-qualified
shared types remain unsupported. Vector elements use the sizes, alignments, and
array strides listed in the kernel interface, including packed three-component
structures. Whole-value copies and scalar component updates use the same rules
as local vectors. Array extents must
be positive integral constant expressions using the scalar rules above.
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
inside `if` branches or loops, or after a possible earlier `return`, produce a
parse error. This conservative restriction rejects some valid CUDA programs
until control-barrier uniformity analysis is available. Conditions and loops
without returns may precede a barrier; returns after the last barrier are
allowed. Synchronization does not extend across workgroups.

Unsupported syntax produces a parse error with a source span. This includes
preprocessor directives and includes, host code and launch syntax,
general pointer operations, switches,
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
