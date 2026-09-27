# CA3 — the complete compiler

**CSC 312 / SEN 313 · Compiler Construction · 10% of your grade, marked out of 10**

| | |
|---|---|
| **Out** | Week 10, Mon 2 Nov 2026 — posted on the portal, because SEN 313 does not meet that week |
| **Due** | **Sun 29 Nov 2026, 23:59** (West Africa Time) |
| **You write** | `src/resolver.rs`, `src/interpreter.rs`, `src/compiler.rs`, `src/vm.rs`: fifteen functions, plus one extension of your own |
| **Self-check** | `tool/run-golden phase-3` and `tool/run-golden phase-4` |
| **Submit** | a zip of your repository, with its `.git` folder and `REFLECTION.md`, to **CA3 submission** on the portal |

You have two copies of this brief: this one, in `kobo/docs/`, and the one on the portal.
**The portal's is the current one.** They are the same today, and if anything ever has
to change, the portal changes and an announcement says what moved.

## What you are building

Everything that is left. By the end of this CA your program takes a Kobo source file and
runs it — twice over, on two different machines you also wrote.

```kobo
fun fib(n) {
  if (n < 2) return n;
  return fib(n - 1) + fib(n - 2);
}
print fib(20);
```

```
$ ./target/debug/kobo fib.kobo
6765
$ ./target/debug/kobo --vm fib.kobo
6765
```

The first run walks the tree your parser built and evaluates it as it goes. The second
compiles that tree to bytecode — a flat array of instructions and a constant table — and
runs the bytecode on a stack machine. **The two must print exactly the same thing**, down to
the error messages, the line numbers and the exit code. That requirement is the entire point
of building the second engine: it is what makes the bytecode a *translation* rather than a
different language that happens to look similar.

This is what Python, Java and JavaScript do. Yours is smaller. It is not different in kind.

## Before anything else: take the published parser if you need it

CA3 is marked by running your compiler end to end — scanner, then parser, then the engine.
**A broken front end will sink CA3 no matter how good your interpreter is.**

A working scanner has been on the portal since Wed 7 Oct, and a working parser goes up on
**Wed 4 Nov**, once CA2's late window has closed. If either of yours is not passing, take
mine as soon as it is up:

```
tool/run-golden phase-1     # must say 14/14
tool/run-golden phase-2     # must say 34/34
```

CA1's and CA2's marks are already recorded and CA3 does not re-mark them. Doing this costs
you nothing and it is what the published solutions exist for.

## Where the code is

The same repository, and again nothing to download: all seven files have been in it since
week 2. Four of them are yours.

| File | What it is |
|---|---|
| `src/environment.rs` | given — the scope stack, complete |
| `src/resolver.rs` | **yours** — static resolution: which declaration is each name? |
| `src/interpreter.rs` | **yours** — the tree-walking engine |
| `src/chunk.rs` | given — the instruction set and the constant table |
| `src/compiler.rs` | **yours** — the tree, translated to bytecode |
| `src/disassembler.rs` | given — prints bytecode, for reading with `--disasm` |
| `src/vm.rs` | **yours** — the stack machine |

`environment.rs` is given, complete, and worth reading properly rather than skipping. It is
a `Vec<HashMap<String, Value>>` with a frame base: scopes are pushed and popped on a single
flat stack, which is possible only because Kobo has no closures (section 4.3). Week 9 explains what
that buys and what it costs. Designing this yourself in Rust is a week of fighting the
borrow checker for no marks, so you are not asked to.

`chunk.rs` and `disassembler.rs` are given for the same reason `ast.rs` and `printer.rs`
were: an instruction set is a data structure, and the thing that prints your bytecode is how
you *read* your bytecode, not a thing to be marked on.

## What you write

**Fifteen functions across four files.**

| File | Functions | Specification |
|---|---|---|
| `src/resolver.rs` | `resolve` `declare` `define` `lookup` `end_scope` | sections 4.2, 4.3 and 7 |
| `src/interpreter.rs` | `execute` `evaluate` `call` | sections 2 and 4 |
| `src/compiler.rs` | `statement` `expression` `declare` `end_scope` `patch` | section 4, and week 11 |
| `src/vm.rs` | `Vm::run` `Vm::call` | sections 4 and 6.1, and week 12 |

Three of them are where the difficulty actually lives:

**`evaluate`** is most of phase 3 on its own. Truthiness (section 2.1), equality across kinds
(section 4.1), short-circuiting `and` and `or` that evaluate to *one of their operands* rather than
to a boolean, division by zero, and every runtime error message in section 5.1.

**`lookup`** is what makes `GET_LOCAL 3` possible. It answers, once and before anything
runs, which declaration each name refers to — the interpreter uses the depth to pick a scope
without searching, and the bytecode compiler uses the slot. section 7 is the specification and it
is short. Two things in it are worth reading twice: a global is *deliberately* not resolved,
because a global may be used before its `var` has run and mutual recursion depends on that;
and a binding is invisible while its own initialiser is being resolved, which is what makes
`var a = a + 1;` read the enclosing `a` (section 4.2) rather than itself.

**The check on your resolver is that nothing changes.** It reports no errors and it alters
no output, no line number and no exit code. If adding it changes what any program prints,
it is wrong — so write it with all five suites green, and they must still be green after.

**`patch`** is jump patching. When you emit the jump for an `if`, you do not yet know where
it lands, because you have not compiled the body. Emit a placeholder, remember its address,
and fill it in when you get there. Every off-by-one in this function is a jump that lands one
instruction late, and phase 4 is built to catch exactly that.

Everything else — the arithmetic helpers, the error constructors, the stack helpers — is
given complete, and you must not change it. The same goes for the files you were given in
CA1 and CA2.

## How your work is tested

Two suites this time, in the format you know.

| Suite | Marks | Programs | Run with |
|---|---|---|---|
| `phase-3` | the tree-walking interpreter | **29** in your repository, **45** marked | no flag |
| `phase-4` | the bytecode compiler and the VM | **11** in your repository, **18** marked | `--vm`, applied for you |

**Phase 4 is not phase 3 run again.** It is a different set of programs, chosen for what the
bytecode path can get wrong and the tree walk gets for free: stack discipline, jump patching,
local slot allocation, frame setup, a constant table that grows past its first page. That is
measurable, not a claim — a VM that forgets to pop a scope's locals passes all 45 phase-3
tests on the VM and fails two phase-4 tests, one of them `jumps_while.kobo`, which is in
your own copy.

```
tool/run-golden phase-3
tool/run-golden phase-4
```

**The published tests are a subset. Hidden tests cover the same concepts with different inputs.**
These run your tests: 29 in phase 3 and 11 in phase 4. Your mark is taken over the marking
tests: 45 in phase 3 and 18 in phase 4. The hidden tests check the same rules with inputs you
have not seen, so the mark reflects understanding rather than transcription.

Every rule they check is in `docs/language-spec.md` — for CA3 that is section 2 (the five kinds of
value, truthiness and how a number prints), section 4 (evaluation, scope and calls) and section 5.1 (the
runtime messages, word for word). The interpreter that passes tests it has never seen is the
one written from section 2 and section 4. Read them again before you submit; passing all your tests is a
floor.

## Marks

CA3 is out of 10: **eight from the tests, two from your extension.**

### The eight automated marks

The two suites are weighted **phase 3 × 0.6 + phase 4 × 0.4**, computed from each suite's
*pass fraction* — not by pooling the tests — then scaled to 8 and rounded to the nearest
half mark. Pooling would have weighted the interpreter
against the VM 71 to 29 for no better reason than that one suite happens to hold 45 programs
and the other 18, and every test I added later would silently have moved your mark.

The interpreter carries more weight because there is no VM without it: a working front end
and correct semantics are the prerequisite for everything phase 4 tests. The VM carries a
real share because it is taught in week 12 and it is where the hard mistakes are.

**There are 45 phase-3 and 18 phase-4 marking tests. Your repository holds 29 and 11 of
them.** What `tool/run-golden` prints on your machine is a floor and not a mark. The hidden
tests check the same rules of `docs/language-spec.md` — sections 2 and 4 and section 5.1 — with different
inputs.

The table is over 45 and 18 because those are what the mark is computed from.

| phase-3, out of 45 | phase-4, out of 18 | Automated mark |
|---|---|---|
| 45 / 45 | 18 / 18 | 8.0 |
| 45 / 45 | 9 / 18 | 6.5 |
| 45 / 45 | 0 / 18 | 5.0 |
| 40 / 45 | 13 / 18 | 6.5 |
| 27 / 45 | 18 / 18 | 6.0 |
| 23 / 45 | 9 / 18 | 4.0 |
| 0 / 45 | 0 / 18 | 0.0 |

A flawless interpreter and no VM at all is 5 out of 8. That is deliberately a pass, and it is
deliberately not full marks: it is not the complete compiler.

A submission that does not build scores zero. `todo!()` compiles; check `cargo build` before you
submit.

### The two extension marks

See below. One mark for it working, one for it demonstrably being yours.

### Late work

Up to two days late, 10% of the mark per day, no part days. After two days, zero.

## The extension

Every student adds **one feature to Kobo that the specification does not have**, and which
feature is not a choice: it is fixed by your matriculation number.

**Take the last digit of your matriculation number.** If it ends in a letter, take the last
digit that appears in it. That row is yours.

| Last digit | Your extension | Files it touches |
|---|---|---|
| **0** | **`for` loops.** `for (var i = 0; i < 3; i = i + 1) print i;` — desugared in the parser into the statement nodes you already have. All three clauses may be empty. | `token.rs`, `parser.rs` |
| **1** | **`break`.** `break;` inside a `while` leaves the loop immediately. `break` outside a loop is an error whose message you define and document. | `token.rs`, `parser.rs`, `interpreter.rs`, `compiler.rs`, `vm.rs` |
| **2** | **`continue`.** `continue;` inside a `while` jumps to the loop's condition. Outside a loop, an error whose message you define and document. | `token.rs`, `parser.rs`, `interpreter.rs`, `compiler.rs`, `vm.rs` |
| **3** | **`??`, nil-coalescing.** `a ?? b` is `a` unless `a` is `nil`, in which case `b`. It short-circuits — `b` is not evaluated when `a` is not nil. It is not `or`: `false ?? 1` is `false`. Binds looser than `or` and tighter than `=`. | `token.rs`, `parser.rs`, `interpreter.rs`, `compiler.rs`, `vm.rs` |
| **4** | **The conditional expression, `c ? a : b`.** Right-associative, binds tighter than `=` and looser than `or`, and evaluates exactly one of the two branches. | `token.rs`, `parser.rs`, `interpreter.rs`, `compiler.rs`, `vm.rs` |
| **5** | **`do { … } while (…);`** The body runs once before the condition is tested for the first time. | `token.rs`, `parser.rs`, `interpreter.rs`, `compiler.rs`, `vm.rs` |
| **6** | **`^`, exponentiation.** Right-associative and binds tighter than `*` and `/`, so `2 ^ 3 ^ 2` is `512`. Both operands must be numbers. Your reflection must say what `-2 ^ 2` does in your implementation and why. | `token.rs`, `parser.rs`, `interpreter.rs`, `compiler.rs`, `vm.rs` |
| **7** | **String escape sequences** — `\n`, `\t`, `\"` and `\\`, which section 1.5 says Kobo does not have. The `--tokens` dump must not change: section 6.1 prints the lexeme as it appears in the source, so `tests/phase-1/valid/strings.kobo` must still pass untouched. | `scanner.rs`, `parser.rs` |
| **8** | **`const` declarations.** `const a = 1;` binds a name that cannot then be assigned to; an initialiser is required, and assigning to one is an error whose message you define and document. | `token.rs`, `parser.rs`, `interpreter.rs`, `compiler.rs` |
| **9** | **`assert(e);`** Evaluates `e` and, if it is falsey, stops the program with a runtime error naming the line. If it is truthy it does nothing at all. | `token.rs`, `parser.rs`, `interpreter.rs`, `compiler.rs`, `vm.rs` |

They are not all the same size — row 0 needs no back-end change at all, and that is a fact
worth being able to explain rather than a discount. **The two marks are for the feature
working and for it being yours, not for how many lines it took.**

Swapping rows with someone is swapping two people's marks. Do not.

### What the extension has to satisfy

**It must work on both engines.** Whatever you add must behave identically with no flag and
with `--vm`. If your row's file list stops before `vm.rs`, that is because the back end gets
your feature for free — check that it actually does.

**It must come with its own tests.** Write at least **three** golden-file tests in the format
you have used all semester and put them in a new directory, `tests/extension/`. Both of these
must pass:

```
tool/run-golden extension
tool/run-golden extension --arg=--vm
```

**Your tests must actually test your feature.** I run your extension suite against the
published CA3 solution, which does not have your feature. If it passes there, your tests are
not testing anything and the mark for the extension is zero. A test whose program does not
contain your new syntax is not a test of your extension.

**It must not break anything.** Phases 1 to 4 are marked as they always were. If adding your
extension drops phase 3 from 29 to 25, you have lost four times what the extension is worth.
Run all four suites before you submit.

### Editing the given files

The extension is the one place where you may touch a file marked do-not-edit, and the rule is
**add, never change**:

- adding a token to `token.rs` is an addition. Renaming `BANG_EQUAL` is not.
- adding a variant to `ast.rs` will stop `printer.rs` compiling until you add the matching
  arm — exhaustive matching is doing its job. Add the arm. That is an addition.
- adding an `Op` to `chunk.rs` will stop `disassembler.rs` compiling. Same thing, same
  answer.
- adding a new error message for your extension is fine. **Changing any message in section 5.1 is
  not**, and it will cost you tests in three suites.

No existing token name, lexeme, error message or output format may move. Everything phases 1
to 4 read is contractual and stays exactly as it is.

## The reflection

Create **`REFLECTION.md`** at the top level of your repository. Answer all three questions in
about 150 words each.

**Every answer must cite line numbers in your own code**, in the form `src/vm.rs:118`, as
they stand in the commit you submit. An answer with no citation, or one whose citation points
at a line that does not do what the answer says, scores nothing for that question.

> **1.** Cite the lines in `interpreter.rs` where `and` and `or` short-circuit. section 4.1 says
> they evaluate to one of their operands rather than to a boolean — explain what your code
> returns for `nil or "fallback"` and for `0 and 1`, and why returning `true`/`false` instead
> would pass some tests and fail others.
>
> **2.** Cite `lookup` in `resolver.rs` and `end_scope` in `compiler.rs`. Explain in your own words why
> resolving a name to a slot at compile time makes the VM faster than the tree walk, and say
> what happens at run time if `end_scope` emits one `Pop` too few — which suite catches it,
> and which one does not.
>
> **3.** Your extension. Cite the lines you added, in every file you added them to. Then
> answer one question: what was harder in the bytecode path than in the tree walk — or, if
> your row needed no bytecode change, why did the back end get your feature for free?
>
> **Anchor it in your own history.** Give the **two commit hashes** — the commit where
> that line was still wrong, and the commit where you fixed it — and quote the line as it
> stood in each. `git log -p --follow src/interpreter.rs` will find them, and `git log --oneline` gives
> you the hashes. Both commits must be your own and both must be at or before the
> deadline. This is checked against your repository, so a hash that is not in your history
> makes the answer incomplete.

Question 3 is half of the extension mark. It is also the question that is obvious to answer
if you built the thing and obvious to spot if you did not. The two hashes are what make it
*yours*: nobody and nothing outside your own repository can supply them.

## Submitting

Upload a zip of your repository to **CA3 submission** on the portal by **Sun 29 Nov 23:59**.
Make it as you did for CA1 and CA2. On macOS or Linux, from the folder above your repository:

```bash
zip -r YOURMATRIC-ca3.zip kobo -x 'kobo/target/*'
unzip -l YOURMATRIC-ca3.zip | grep .git/
```

The second command must list your `.git` folder. On Windows, or if you have no `zip`
command, or if that check prints nothing, upload a git bundle instead. Commit everything
first, then run this inside your repository and upload the `.bundle` file:

```bash
git bundle create YOURMATRIC-ca3.bundle --all
```

Your repository must contain the crate as given, with `resolver.rs`, `interpreter.rs`,
`compiler.rs` and
`vm.rs` filled in, `tests/extension/` holding at least three tests, and `REFLECTION.md` at
the top level. A repository with no `REFLECTION.md` is an incomplete submission: it is not
marked until the file is there, and the late penalty runs from the deadline until it arrives.

**Commit as you go.** Four files, fifteen functions and an extension is four weeks of work.
The participation mark is counted from weeks in which you committed something, and week 13's
Period 3 is a clinic — bring the compiler you have and finish it in class.

If the portal itself is down when you try to submit, email your zip or bundle to
**fokocha-ojeah@pau.edu.ng** before the deadline. Same deadline, same marks, same late
policy.

## Honesty

CA3 is individual work. Discuss ideas with anyone; type your own code. Submissions are
checked for similarity against each other, and your commit history is part of your
submission.

The extension is keyed to your matriculation number for a reason, and the reason is worth
stating plainly: at a hundred and fifty students it is the cheapest way of asking each of you
a different question. Two identical `do`-`while` implementations from two people whose
matriculation numbers both end in 5 is an ordinary thing that happens. A `do`-`while` from
someone whose number ends in 8 is not.
