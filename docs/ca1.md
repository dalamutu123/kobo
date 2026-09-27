# CA1 — the scanner

**CSC 312 / SEN 313 · Compiler Construction · 5% of your grade, marked out of 5**

| | |
|---|---|
| **Out** | Week 3 |
| **Due** | **Sun 4 Oct 2026, 23:59** (West Africa Time) |
| **You write** | `src/scanner.rs` — five functions |
| **Self-check** | `tool/run-golden phase-1` |
| **Submit** | a zip of your repository, with its `.git` folder and `REFLECTION.md`, to **CA1 submission** on the portal |

You have two copies of this brief: this one, in `kobo/docs/`, and the one on the portal.
**The portal's is the current one.** They are the same today, and if anything ever has
to change, the portal changes and an announcement says what moved.

## What you are building

The first stage of a compiler: the part that turns a flat run of characters into a list of
tokens. Given this file,

```kobo
print 2 + 3 * 4;
```

your program must produce this, exactly:

```
[line 1] PRINT 'print'
[line 1] NUMBER '2'
[line 1] PLUS '+'
[line 1] NUMBER '3'
[line 1] STAR '*'
[line 1] NUMBER '4'
[line 1] SEMICOLON ';'
[line 1] EOF ''
```

Thirty-four token types, correct line numbers, and the two scanning error messages. That is
the whole of CA1. Nothing parses, nothing runs — a scanner that reaches the end of the file
having recognised every token has done its job completely.

## Where the code is

Download the **Kobo starter** zip from the portal and unzip it. It makes a folder called
`kobo` that holds everything: the crate, the specification, your tests and
`tool/run-golden`, which runs them. There is nothing else to download and nothing to
install beyond Rust itself (the **Rust Setup Guide** on the portal).

It is the whole compiler, and that is deliberate. Every function you will write between
now and the end of November is already there, each one a `todo!()` carrying the rule it
implements and the section of the specification that defines it. You will not download
code again. CA1 is the five functions in `src/scanner.rs`; the rest have their own weeks,
and a CA is the window in which a part of it is marked, not the day its files arrive.

Make that folder a git repository and commit the starter as you were given it:

```bash
cd kobo
git init
git add -A
git commit -m "Kobo starter"
```

**This is the only repository you use all semester.** CA2 and CA3 are marking windows on it,
not new downloads: every file they assess is already here, waiting as `todo!()`. Your
commit history runs unbroken from week 3 to the end of November. Do not start a second
repository, and do not rename this one.

```
cargo build
tool/run-golden phase-1
```

On Windows, the second command is `python tool/run-golden phase-1`.

The first command must succeed on the day you unzip the starter. If it does not, that is a
toolchain problem, not a CA1 problem: email me at **fokocha-ojeah@pau.edu.ng**, or bring it
to a Period 3 class, rather than losing hours to it. The second command will say
`0/14`. That is correct: nothing is written yet.

## What you write, and what you must not touch

You write **one file**, `src/scanner.rs`, and in it **five functions**. Each is currently a
`todo!()` with a comment saying what it must do and which section of the specification says
so.

| Function | What it must do |
|---|---|
| `run` | drive the scan until the source runs out, then add the `EOF` token |
| `scan_token` | recognise one token, or report an unrecognised character |
| `string` | scan a string literal, including one that spans lines |
| `number` | scan a number literal, with its optional fractional part |
| `identifier` | scan a word, then decide whether it is a keyword |

Everything around them is given to you complete, and you must not change it: `Cargo.toml`,
`Cargo.lock`, `src/main.rs`, `src/token.rs`, and the character primitives at the bottom of
`src/scanner.rs` (`advance`, `peek`, `matches` and the rest). Those files are the compiler
you are handed, not the compiler you write. Read them anyway — what you write has to fit
them. `src/token.rs` in particular already holds all thirty-four token names and the keyword
lookup, so you never have to retype them and never have to guess how one is spelled.

**Changing a given file is not a shortcut, it is a mark you lose.** Your submission is built
and marked from those files as they were handed out.

## The specification is the contract

`docs/language-spec.md` in your repository decides every question CA1 can raise. It is the
same document as the **Kobo Language Specification** on the portal. Where your code and that
document disagree, the document is right; where a test and that document disagree, the
document is right and the test is a bug — tell me and I will fix the test.

The sections CA1 depends on:

| Section | What it settles |
|---|---|
| section 1.1 | whitespace, comments, and when the line counter moves |
| section 1.2 | all thirty-four tokens, and why the operators need one character of lookahead and numbers need two |
| section 1.3 | identifiers, and why a keyword is read as a word first and looked up second |
| section 1.4 | numbers — and why `.5` and `5.` are not numbers |
| section 1.5 | strings, including multi-line ones, and where an unterminated one is reported |
| sections 5 and 5.1 | the two scanning error messages, character for character, on standard error |
| section 6.1 | the `--tokens` output format, and which line the `EOF` token carries |

Read section 6.1 before you write `run`. It is short, it is the format every test reads, and the
`EOF` line rule is the single most common way a working scanner fails all ten of your valid
tests at once.

## How your work is tested

A test is a Kobo program that carries its own expected output in comments.

```kobo
// tests/phase-1/valid/arithmetic.kobo — the opening of it
1 + 2;
3 - 4;
5 * 6;
7 / 8;

// expect: [line 1] NUMBER '1'
// expect: [line 1] PLUS '+'
// expect: [line 1] NUMBER '2'
```

```kobo
// tests/phase-1/invalid/unterminated_string.kobo
var name = "Kobo;
print name;
// [line 1] Error: String is never closed.
```

`// expect:` lines are standard output, in order and complete — anything extra fails.
`// [line N] …` lines must each appear on standard error, and the exit code follows from
them (65 if any scan error is expected, 0 otherwise).

**The published tests are a subset. Hidden tests cover the same concepts with different inputs.**
You are marked on 21 tests, the marking tests: 15 valid and 6 invalid. Your repository holds
14 of them in `tests/phase-1/`, 10 valid and 4 invalid, and these are your tests. The other 7
are the hidden tests. They are run only when your work is marked, and nothing in them is
outside `docs/language-spec.md`. `tool/run-golden` applies `--tokens` for you, because a
scanner prints nothing of its own and there would otherwise be nothing to mark.

`tool/run-golden` never looks inside your compiler. It runs your binary and reads what came
out. That is deliberate: a hundred and fifty people will write a hundred and fifty different
scanners, and the only fair question to ask all of them is what they printed.

## Checking your own mark

```
tool/run-golden phase-1
```

This command runs your 14 tests. Your mark is taken over the 21 marking tests: your 14 and
the 7 hidden tests. The hidden tests check the same rules with inputs you have not seen, so
the mark reflects understanding rather than transcription.

There is no trick in them, because **every rule they check is written down in
`docs/language-spec.md`**. That document is the contract for this course and the tests are
one reading of it, not the thing itself. A scanner built by reading section 1 and section 5 and then
checked against your tests passes tests it has never seen. A scanner built by changing code
until your tests pass passes exactly your tests and no more. The hidden tests exist to tell
those two apart, and that is the only thing they measure.

So passing all your tests is a floor, not a total. What to do about it is one sentence:
**read section 1.1 to sections 1.5 and 5.1 and section 6.1 and check your scanner against each rule by hand** —
especially the rules none of your tests happens to reach.

Each failure names the test, what was expected and what it got, so the output is a to-do
list as much as a score:

```
FAIL  invalid/unterminated_string.kobo
      stderr: expected '[line 1] Error: String is never closed.'
```

Run it before every commit and always before you submit. If it says `14/14` on your machine
it says `14/14` on mine, and then I run the 7 hidden tests.

## The reflection

Create **`REFLECTION.md`** at the top level of your repository. Answer all three questions
below in about 150 words each — 500 words for the whole file is plenty.

**Every answer must cite line numbers in your own code**, in the form `src/scanner.rs:42`,
as they stand in the commit you submit. An answer with no citation, or one whose citation
points at a line that does not do what the answer says, is not a complete submission.

> **1.** Cite the line where you decide that a `.` begins a fractional part rather than
> being a stray character. What does your scanner do with the input `5.`, and why is that
> what section 1.4 requires?
>
> **2.** Cite every line where you change the line counter. Explain which line your `EOF`
> token ends up carrying for a file that ends in two blank lines, and why section 6.1 asks for
> that rather than the file's last line.
>
> **3.** Name one test in `tests/phase-1/` that you failed at some point. Cite the line you
> changed to fix it, and say plainly what you had misunderstood. "I made a typo" is not an
> answer; "I was incrementing `line` before adding the token, so the token was reported on
> the next line" is.
>
> **Anchor it in your own history.** Give the **two commit hashes** — the commit where
> that line was still wrong, and the commit where you fixed it — and quote the line as it
> stood in each. `git log -p --follow src/scanner.rs` will find them, and `git log --oneline` gives
> you the hashes. Both commits must be your own and both must be at or before the
> deadline. This is checked against your repository, so a hash that is not in your history
> makes the answer incomplete.

Question 3 is the one that matters. It is asking what you learned, and it is easy to answer
honestly and obvious when it is not. The two hashes are what make it *yours*: nobody and
nothing outside your own repository can supply them.

## Submitting

Upload a zip of your repository to **CA1 submission** on the portal by **Sun 4 Oct 23:59**.
How to make the zip is below.

Your repository must contain, at its top level:

- the crate as you were given it, with `src/scanner.rs` filled in;
- `REFLECTION.md`.

Do not commit `target/`. The starter carries a `.gitignore` that excludes it; if yours has
no such file, create one at the top level of your repository holding the single line
`/target`, and commit it.

**Commit as you go, and at least once a week.** A commit every time something starts working
is worth more to you than one commit at the end: it is how the participation mark is counted
(5% of the course), it is what you fall back on when you break something, and a history that
shows the work is a history nobody has to ask you about.

### The reflection is part of the submission

A repository with no `REFLECTION.md` is an incomplete submission. It is not marked until the
file is there, and the late penalty below runs from the deadline until it arrives. This is
not a formality — the reflection is one of the ways I satisfy myself that the code is yours.

### Making the zip

On macOS or Linux, run this from the folder **above** your repository, with your
matriculation number in place of `YOURMATRIC`:

```bash
zip -r YOURMATRIC-ca1.zip kobo -x 'kobo/target/*'
```

**The `.git` folder must be inside the zip.** That folder is your history, and your history
is where the participation mark and the honesty checks come from — a zip of source files
alone is worth fewer marks than the same work with its history attached. Check it before you
upload:

```bash
unzip -l YOURMATRIC-ca1.zip | grep .git/
```

If that prints nothing, your zip tool has dropped hidden folders.

### On Windows, or if the zip fails: a git bundle

On Windows, or if you have no `zip` command, or if the check above printed nothing, upload a
git bundle instead. It packs your whole history into one file and works anywhere git does.
A bundle holds only what you have committed, so commit everything first, `REFLECTION.md`
included. Then run this inside your repository:

```bash
git bundle create YOURMATRIC-ca1.bundle --all
```

Upload the `.bundle` file in place of the zip.

### If the portal is unavailable

If the portal itself is down when you try to submit, email your zip or bundle to
**fokocha-ojeah@pau.edu.ng** before the deadline. Same deadline, same marks, same late
policy.

## Marks

CA1 is out of 5, and all five come from the 21 marking tests, run by the same
`tool/run-golden` you have: your pass fraction over them, scaled to 5 and rounded to the
nearest half mark.

**There are 21 marking tests. Your repository holds 14 of them.** So the `14/14` that
`tool/run-golden` prints on your machine is a floor and not a mark. The 7 hidden tests are
run when I mark; they are not harder and they are not tricks, they test the same rules of
`docs/language-spec.md` with different inputs, and a scanner written from that document
passes tests it has never seen.

The table is over 21 because 21 is what the mark is computed from.

Any number of passes is marked the same way, by the formula above; these rows are
landmarks, not the whole scale.

| Tests passed, out of 21 | Mark |
|---|---|
| 21 / 21 | 5.0 |
| 19 / 21 | 4.5 |
| 17 / 21 | 4.0 |
| 15 / 21 | 3.5 |
| 13 / 21 | 3.0 |
| 11 / 21 | 2.5 |
| 7 / 21 | 1.5 |
| 0 / 21 | 0.0 |

A submission that does not build scores zero, because a compiler that does not compile
cannot be run against anything. **`cargo build` is the one thing to check before you
submit**, and `todo!()` compiles perfectly well — a half-finished scanner that builds will
always score more than a nearly-finished one that does not.

### Late work

Up to two days late, 10% of the mark per day, no part days: one day late multiplies your
mark by 0.9, two days by 0.8. After two days, zero. The clock is the deadline, not the day
you started.

### After the deadline

I publish a working scanner on **Wed 7 Oct**, once the two-day late window has closed. Read it,
compare it with yours, and take it forward into CA2 if you need to. Failing CA1 costs you
CA1's marks. It does not cost you the course.

## Honesty

CA1 is individual work. Discuss ideas with anyone you like; type your own code.

Every submission is checked for similarity against every other submission, and your commit
history is part of your submission. A compiler that appears in a single commit a few hours
before the deadline is not evidence of anything on its own — plenty of people work in one
long sitting — but it is the kind of history I open and look at. The reflection is read
alongside the code.

## If you get stuck

Bring the exact error text to a Period 3 class. Not a description of it — the text.
A scanner bug that takes five minutes in class can take hours on your own.
