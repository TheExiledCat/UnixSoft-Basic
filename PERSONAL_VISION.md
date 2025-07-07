
# UnixSoft Basic — Full Summary & Development Plan

---

## 1. Starting Point: Applesoft BASIC

- **Applesoft BASIC features:**
  - Line-numbered, interpreted.
  - Limited data types: mainly numeric (floats by default) and strings.
  - No static typing or type annotations.
  - Control flow uses line numbers for jumps.
  - No native scoping blocks; `END` keyword terminates program.
  - Dynamic typing with implicit casting.

---

## 2. My Vision: Modernizing Applesoft BASIC

### Key changes & improvements i want:

- **Compiled to native x64 (Linux) assembly**, no interpreter or JIT.
- **Static typing** with optional type inference and type annotations.
- **Structured control flow** using block keywords along with the original line numbers.
- Introduce block scoping **outside control flow** for local variables.
- Add **`BEGIN` / `FIN`** keywords for explicit scope blocks. (think of these like {} in modern languages)
- Use **`LET`** for variable assignments (instead of `DIM`) but enforce type annotation or assignment.
- Support **true/false keywords** for boolean values.
- Implement **type casting rules** compatible with static typing.
- Keep code output **efficient and small**.
- Target **Linux x64** (System V AMD64 ABI) with plans to possibly support ARM64 in the future.
- Add **IO improvements** (better input/output handling more similar to modern libc).
  
---

## 3. AST and Compiler Architecture

- Use an AST node called `Scope` to represent `BEGIN` / `FIN` blocks.
- Each `Scope` node holds a list of inner AST nodes (statements).
- Symbol tables are scoped per `Scope` node to handle local variables and type checking.
- Static typing allows:
  - Compile-time type inference.
  - Type annotations (`LET X = 5` or `LET X AS INTEGER` (defaults to 0)).
- Codegen:
  - Allocate stack space per scope.
  - Use native registers (integer: rax, rbx, etc; float: xmm0, xmm1, etc).
  - Compile line numbers to labels if supporting legacy syntax, but aim to move away from line-number-driven flow.
- Handle floating-point operations using SSE instructions (`movsd`, `addsd`, etc).
- Pass floats in xmm registers per calling convention.
- For printing floats, integrate or write float-to-string conversion routines.

---

## 4. Syntax Suggestions & Keywords

| Feature               | Keyword(s)          | Notes/Changes vs Applesoft              |
|-----------------------|---------------------|----------------------------------------|
| Variable declaration   | `LET`               | Enforce assignment ortype   |
| Block scoping         | `BEGIN` / `FIN`     | New keywords to define scope blocks     |
| Boolean literals       | `TRUE`, `FALSE`     | Map to 1 and 0                         |
| Control flow blocks    | `IF ... THEN BEGIN` / `FI` or `FIN` | Use blocks instead of line jumps, but leaving support for `IF ... GOTO` |
| End program           | `END`               | Same as Applesoft, terminate program   |
| Type annotations      | `LET X AS FLOAT = 5`  | Explicit static typing                  |

---

## 5. Runtime and Execution

- The compiler outputs native **x64 assembly**.
- Follow the **System V AMD64 calling convention**.
- Stack frames per scope for local variables.
- Use SSE for float arithmetic.
- Compile boolean logic with 0/1 values.
- Provide minimal runtime for IO (input, output) with improvements over Applesoft.
- Handle float-to-string conversions for printing.

---

## 6. Development Roadmap

| Step                     | Description                                  | Notes                                   |
|--------------------------|----------------------------------------------|-----------------------------------------|
| **Parser**               | Write grammar supporting `LET`, `BEGIN`/`FIN`, `IF...THEN...`, `FI`, `TRUE`, `FALSE` | Define AST nodes (Scope, Let, If, etc) |
| **AST Construction**     | Build `Scope` nodes with nested statements    | Symbol table stack per scope            |
| **Type System**          | Implement static typing and type inference    | Support explicit type annotations       |
| **Code Generation**      | Compile AST to x64 assembly                    | Manage registers, stack frames, labels  |[I
| **Floating-point Support**| Use SSE instructions for float operations     | movsd, addsd, etc                       |
| **IO Routines**          | Build or link runtime for input/output        | Float-to-string, improved console IO    |
| **Testing**              | Write BASIC test programs                      | Compare against Applesoft behaviors, new features |
| **Optimization**         | Keep compiled output small and efficient      | Reuse registers and stack space         |

---


