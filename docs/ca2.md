# CA2 — the parser

**CSC 312 / SEN 313 · Compiler Construction · 5% of your grade, marked out of 5**

| | |
|---|---|
| **Out** | Week 7, Mon 12 Oct 2026 |
| **Due** | **Sun 1 Nov 2026, 23:59** (West Africa Time) |
| **You write** | `src/parser.rs` — nineteen functions |
| **Self-check** | `tool/run-golden phase-2` |
| **Submit** | a zip of your repository, with its `.git` folder and `REFLECTION.md`, to **CA2 submission** on the portal |

You have two copies of this brief: this one, in `kobo/docs/`, and the one on the portal.
**The portal's is the current one.** They are the same today, and if anything ever has
to change, the portal changes and an announcement says what moved.

## What you are building

The second stage: the part that turns your token stream into a tree. Given this file,

```kobo
print 2 + 3 * 4;
var a = 1;
{ a = 2; }
```

your program must produce this, exactly:

```
(print (+ 2 (* 3 4)))
(var a 1)
(block (expr (= a 2)))
```

The shape of that first line is the whole point of CA2. Nothing in the token stream says
that `*` binds tighter than `+`; the grammar in section 3.2 says it, and your parser is what turns
a grammar into a tree. A parser that produces `(+ (* 2 3) 4)` has scanned perfectly and
understood nothing.

CA2 also has to fail well. Twenty-two of the fifty marking tests are programs with syntax
errors, and a parser that reports one missing semicolon as forty errors is a parser nobody
would use.

## Before anything else: your scanner is now load-bearing

CA2 is marked by running your compiler end to end — your scanner feeds your parser. **A
broken scanner will sink CA2 no matter how good your parser is.**

I published a working scanner on Wed 7 Oct, once CA1's late window had closed. If yours is
not passing `14/14`, take mine.
That is exactly what it is published for, and doing so costs you nothing here: CA1's mark is
already recorded and CA2 does not re-mark your scanner. Do this in week 7, not in the last
week of October.

```
tool/run-golden phase-1     # must still say 14/14 before you start CA2
```

## Where the code is

The same repository you have been working in all semester. There is nothing to download:
these files have been in it since week 2, and `src/parser.rs` has been sitting there as
nineteen `todo!()`s since the day you unzipped the starter. You keep one repository for the
whole course, and its history is part of your submission.

Four files carry CA2. Three are given to you complete, under a do-not-edit banner:

| File | What it is |
|---|---|
| `src/value.rs` | the runtime value type. A literal in the tree carries one |
| `src/ast.rs` | the tree itself: the `Expr` and `Stmt` types every rule builds |
| `src/printer.rs` | the `--ast` renderer, which is how phase 2 is marked |
| `src/parser.rs` | **yours** |

`ast.rs` is given for a reason worth understanding rather than resenting. A tree you designed
yourself would be a set of Rust lifetime problems you designed yourself, and this course is
not about the borrow checker. `Box<Expr>`, no lifetimes, one enum per node kind, and every
consumer matching exhaustively — that is the shape the whole compiler is built on, and it is
why a missing case in your parser is a compile error rather than a bug you find in December.

`printer.rs` is given because `--ast` is how a parser is marked at all, and marking a correct
tree as wrong because its printer had a typo would be marking two things as one.

## What you write

**One file**, `src/parser.rs`. Nineteen functions, each currently a `todo!()` carrying the
grammar rule it implements and the section that defines it.

| Group | Functions | Specification |
|---|---|---|
| Declarations and statements | `declaration` `fun_declaration` `var_declaration` `statement_inner` `return_statement` `if_statement` `while_statement` `block` | section 3.1 |
| The precedence ladder | `assignment` `or` `and` `equality` `comparison` `term` `factor` `unary` `call` `primary` | section 3.2 |
| Recovery | `synchronise` | section 5.2 |

Nineteen sounds like a great deal and is not. Six of them — `or`, `and`, `equality`,
`comparison`, `term`, `factor` — are the same three lines with a different set of operators:
call the rule below, then loop while the next token is one of mine. Write `term` carefully
and the other five follow it in an hour.

The token bookkeeping (`matches`, `consume`, `check`, `peek`), the nesting guard and the
error reporting are given complete, and you must not change them. The same goes for the
files you were given in CA1.

**This is checked, not trusted.** Every given file in your submission is compared against
what was published, and one that has been altered is reported on your mark sheet and costs
you. Your submission is built and marked from those files as they were handed out, so
changing one is not a shortcut.

## The specification is the contract

| Section | What it settles |
|---|---|
| section 3.1 | every declaration and statement, and that `if`/`while` take a *statement* as a body |
| section 3.2 | the precedence ladder, associativity, and why assignment is an expression |
| section 5 | the two parse error shapes, and that `at end` reports the last real token's line |
| section 5.1 | all twenty-one parse lines, character for character — twenty distinct messages, since `Expect expression.` appears in two shapes |
| section 5.2 | synchronisation, and the nesting limit |
| section 6.1 | the `--ast` output format |

Two rules in section 3.2 are where the marks actually are, and both are places where the obvious
reading of the grammar is wrong:

**The dangling `else`.** `if (false) if (true) print "a"; else print "b";` — which `if`
does the `else` belong to?
section 3.1 says the nearest unmatched one. Notice that the grammar does not say so; your recursion
does, for free, and being able to explain why is worth more than the two marks it earns.

**Assignment is not LL(1).** By the time you have read `a` you cannot yet tell whether you
are parsing a variable or the left side of an assignment, and one token of lookahead does
not save you. section 3.2 tells you the answer: parse the left side as an ordinary expression, then
check afterwards what it turned out to be, and report *Cannot assign to this expression.* if it is
not a bare identifier.

## How your work is tested

The format is exactly CA1's — a Kobo program carrying its own expected output.

```kobo
// a test of precedence
print 2 + 3 * 4;
// expect: (print (+ 2 (* 3 4)))
```

```kobo
// tests/phase-2/invalid/expect_semicolon_after_value.kobo
print 1
print 2;
// [line 2] Error at 'print': Expect ';' after value.
```

Read that second one twice. The semicolon is missing from line 1, and the error is reported
at line 2 — because the parser only discovers the omission when it reaches the token that
should have been a `;` and finds `print` instead. Where an error is reported is a decision,
and section 5 has already made it.

**The published tests are a subset. Hidden tests cover the same concepts with different inputs.**
You are marked on 50 tests, the marking tests: 28 valid and 22 invalid. Your repository holds
34 of them in `tests/phase-2/`, 16 valid and 18 invalid, and these are your tests. The other
16 are the hidden tests. `tool/run-golden` applies `--ast` for you, because a parser prints
nothing of its own.

That so many of them are invalid is deliberate. Getting a right program right is the easy
half of parsing; the difference between a parser and a compiler people can use is what
happens when the input is wrong.

## Checking your own mark

```
tool/run-golden phase-2
```

This runs your 34 tests. Your mark is taken over the 50 marking tests: your 34 and the 16
hidden tests. The hidden tests check the same rules with inputs you have not seen, so the
mark reflects understanding rather than transcription.

They are not a trick — **every rule they check is in `docs/language-spec.md`**, and for CA2
that means section 3.1 and section 3.2 in particular. The grammar there is written with `( … )*` loops for
a reason, and the reason is the shape of the tree the loop builds. A parser written from section 3.2
passes tests it has never seen; a parser written until your tests pass passes your tests and
no more. Run this before every commit, and read section 3 at least once more before you submit.

## The reflection

Create **`REFLECTION.md`** at the top level of your repository — replace CA1's, do not append
to it; its history is in git. Answer all three questions in about 150 words each.

**Every answer must cite line numbers in your own code**, in the form `src/parser.rs:87`, as
they stand in the commit you submit. An answer with no citation, or one whose citation points
at a line that does not do what the answer says, scores nothing for that question.

> **1.** Cite the lines in your `if_statement` that resolve the dangling `else`. Explain why
> the recursion decides it rather than the grammar, and what your parser prints for
> `if (false) if (true) print "a"; else print "b";`, and what it would print if the `else`
> bound to the outer `if` instead.
>
> **2.** Cite the line in `assignment` where you check what the left side turned out to be.
> Explain why that check cannot happen before the left side is parsed, and name the message
> from section 5.1 that `1 = 2;` produces.
>
> **3.** Cite `synchronise`. Say exactly which tokens you stop on and why those. Then name
> one test in `tests/phase-2/invalid/` whose output changes if you delete the call to it, and
> say what its output becomes.
>
> **Anchor it in your own history.** Give the **two commit hashes** — the commit where
> that line was still wrong, and the commit where you fixed it — and quote the line as it
> stood in each. `git log -p --follow src/parser.rs` will find them, and `git log --oneline` gives
> you the hashes. Both commits must be your own and both must be at or before the
> deadline. This is checked against your repository, so a hash that is not in your history
> makes the answer incomplete.

## Submitting

Upload a zip of your repository to **CA2 submission** on the portal by **Sun 1 Nov 23:59**.
Make it as you did for CA1. On macOS or Linux, from the folder above your repository:

```bash
zip -r YOURMATRIC-ca2.zip kobo -x 'kobo/target/*'
unzip -l YOURMATRIC-ca2.zip | grep .git/
```

The second command must list your `.git` folder. On Windows, or if you have no `zip`
command, or if that check prints nothing, upload a git bundle instead. Commit everything
first, then run this inside your repository and upload the `.bundle` file:

```bash
git bundle create YOURMATRIC-ca2.bundle --all
```

Your repository must contain the crate as given, with `src/parser.rs` filled in, and
`REFLECTION.md` at its top level. A repository with no `REFLECTION.md` is an incomplete
submission: it is not marked until the file is there, and the late penalty runs from the
deadline until it arrives.

**Commit as you go** — nineteen functions is four weeks of work, not one weekend, and the
participation mark is counted from weeks in which you committed something.

If the portal itself is down when you try to submit, email your zip or bundle to
**fokocha-ojeah@pau.edu.ng** before the deadline. Same deadline, same marks, same late
policy.

## Marks

CA2 is out of 5, and all five come from the 50 marking tests, run by the same
`tool/run-golden` you have: your pass fraction over them, scaled to 5 and rounded to the
nearest half mark.

**There are 50 marking tests. Your repository holds 34 of them.** The `34/34` that
`tool/run-golden` prints on your machine is a floor and not a mark. The 16 hidden tests are
run when I mark; they test the same rules of `docs/language-spec.md` with different inputs,
and a parser written from section 3 and section 5 passes tests it has never seen.

The table is over 50 because 50 is what the mark is computed from.

| Tests passed, out of 50 | Mark |
|---|---|
| 50 / 50 | 5.0 |
| 45 / 50 | 4.5 |
| 40 / 50 | 4.0 |
| 35 / 50 | 3.5 |
| 28 / 50 | 3.0 |
| 13 / 50 | 1.5 |
| 0 / 50 | 0.0 |

A submission that does not build scores zero. `todo!()` compiles; check `cargo build` before
you submit.

### Late work

Up to two days late, 10% of the mark per day, no part days. After two days, zero.

### After the deadline

I publish a working parser on **Wed 4 Nov**, once the two-day late window has closed. CA3 is
the whole compiler and it stands on the parser, so if yours is not finished, take mine on the
fourth of November and start CA3 on time. That is what it is for.

## Honesty

CA2 is individual work. Discuss ideas with anyone; type your own code. Submissions are
checked for similarity against each other, and your commit history is part of your
submission.
