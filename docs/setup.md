# Rust Setup Guide

**CSC 312 / SEN 313 · Compiler Construction**

You need three things installed: **Rust**, **git** and **Python 3**. No libraries, no
frameworks, no editors you have to buy. Everything after the install works with the network
switched off.

**Install all three before you next bring a laptop to Period 3.** The Rust download is
300–500 MB and it fails part-way on a bad connection — it failed twice on a good one while
these notes were being written. Starting early costs you nothing; starting late costs you
the class.

If your connection cannot manage it, **USB drives with the Rust toolchain will be passed
round in class**. Ask. That is what they are for — there is no penalty and no queue.

## 1 · Install Rust

### The normal way, if you have a usable connection

**macOS and Linux** — open a terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Press Enter to accept the default when it asks. Then close the terminal and open a new one,
so your `PATH` picks up the change.

**Windows** — download and run `rustup-init.exe` from <https://rustup.rs>.

> Windows asks whether you want the **MSVC** or the **GNU** toolchain. **Choose GNU.** MSVC
> then needs Visual Studio Build Tools, which is several more gigabytes. GNU is
> self-contained and does everything this course needs.

If the download dies half-way, run the same command again — it resumes. If it fails three
times, stop and use the USB.

### The offline way, from the USB

The drive holds one installer per platform. Copy the one for your machine onto your own disk
first — installing straight from a USB is slow and fails more often.

| Your machine | File |
|---|---|
| Windows | `rust-1.98.0-x86_64-pc-windows-gnu.msi` (358 MB) |
| Mac, Apple Silicon (M1–M4) | `rust-1.98.0-aarch64-apple-darwin.tar.gz` (283 MB) |
| Mac, Intel | `rust-1.98.0-x86_64-apple-darwin.tar.gz` (219 MB) |
| Linux | `rust-1.98.0-x86_64-unknown-linux-gnu.tar.gz` (364 MB) |

Not sure which Mac you have? Apple menu > About This Mac. "Apple M1/M2/M3/M4" means Apple
Silicon; "Intel" means Intel.

**Windows:** double-click the `.msi` and follow it. Nothing else to do.

**macOS and Linux:**

```bash
tar xzf rust-1.98.0-<your-platform>.tar.gz
cd rust-1.98.0-<your-platform>
./install.sh --prefix="$HOME/.local"
```

Then add it to your `PATH` — put this line at the end of `~/.zshrc` (macOS) or `~/.bashrc`
(Linux), then open a new terminal:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

## 2 · Install git and Python

**git** keeps the history of your work. **Python 3** runs `tool/run-golden`, the command that
tests your compiler. You may have both already: if `git --version` and `python3 --version`
each print a version, go on to step 3.

**macOS** — run this, and accept the install it offers:

```bash
xcode-select --install
```

That installs Apple's command line developer tools, which include git and Python 3.

**Linux** (Ubuntu or Debian):

```bash
sudo apt install git python3
```

**Windows** — install **Git for Windows** from <https://git-scm.com/install/windows>, and
Python with the **Python install manager** from <https://www.python.org/downloads/windows/>.
Accept the defaults in both, then open a new terminal.

## 3 · Check it worked

```bash
rustc --version
cargo --version
git --version
python3 --version
```

All four must print a version. `rustc 1.98.0` is what these instructions install and what I
mark with. **You need at least 1.78**: older toolchains cannot read the starter's `Cargo.lock`. If
`rustc --version` prints anything older, run `rustup update`, or install from the USB.

**If the terminal says "command not found":** the install worked but your shell has not found
it yet. Open a brand-new terminal window. If `rustc` or `cargo` still fails, your `PATH` line
is missing — see step 1.

## 4 · Get the code

Download the **Kobo starter** zip from the portal and unzip it. It makes a folder called
`kobo`. Make that folder a git repository and commit the starter as you were given it:

```bash
cd kobo
git config --global user.name "Your Name"      # once per machine, if you never have
git config --global user.email "you@pau.edu.ng"
git init
git add -A
git commit -m "Kobo starter"
```

This is the repository you keep for the whole course. No `npm install`, no `pip install`,
no dependencies to fetch. The repository is everything.

## 5 · Build

```bash
cargo build
```

The first build takes a second or two, and ends with a line reading `Finished `dev` profile`.
If it finishes without an error,
your toolchain works and you are ready to start.

**It does not do anything yet, and that is the point.** What you have unzipped is the whole
compiler: every function you will write between now and the end of November is already in it,
each one replaced by a `todo!()` and a line saying what it must do and which section of
`docs/language-spec.md` says so. Everything around them — the driver, the token table, the
character primitives — is given to you complete, and you must not change it.

Run it and it stops at the first function nobody has written, naming that function:

```
thread 'main' panicked at src/scanner.rs:31:
not yet implemented: run
```

That is the to-do list, and it is why you can build from the first day. **CA1 is the five
functions in `src/scanner.rs`.** Do not read the other thirty-four today; each has its own
week, and the week's third period tells you which one it is.

To see everything still to write, search the folder for `todo!` — `Ctrl+Shift+F` in your
editor, or `grep -rn "todo!" src/` in a terminal.

## 6 · Check your own work

```bash
tool/run-golden phase-1
```

On Windows, type `python tool/run-golden phase-1` instead (or `py` in place of `python`).

This runs the tests you were given, with the code I mark with. **Today it will say `0/14`**
— nothing is written yet. Every failure it lists names the test, the output that was wanted
and the output it got, so it is a to-do list as much as a score. **Run it before every
submission.**

The tests in your repository are a subset of the marking tests: `tool/run-golden` says so on
its last line every time it runs. Everything the hidden tests check is written down in
`docs/language-spec.md`, which is the contract — so the way to score above what you can see
is to read it, not to guess at it.

Once your scanner works, you can watch it directly:

```bash
echo 'print 2 + 3 * 4;' > hello.kobo
./target/debug/kobo --tokens hello.kobo        # on Windows: target\debug\kobo.exe --tokens hello.kobo
```

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

That is your compiler cutting a source file into tokens. Running a file *without* `--tokens`
does nothing until CA3, when the interpreter arrives and `print 2 + 3 * 4;` starts answering
`14`. By week 13 it compiles to bytecode and runs on a virtual machine you wrote.

## Working offline

Once step 5 has succeeded once, **you never need the internet again** — not to build, not to
test, not to run. The compiler has zero external dependencies, which is a deliberate choice
made for exactly this reason.

If `cargo` ever tries to reach the network, stop it:

```bash
cargo build --offline
```

You need a connection only to download the starter and to upload your work to the portal.
Committing needs no connection at all. If even that is genuinely difficult for you, tell me
in week 3. It is not a problem, but only if I know.

## When something goes wrong

| Symptom | What to do |
|---|---|
| `command not found: cargo` | Open a new terminal. Still failing? Your `PATH` line is missing — see step 1. |
| `command not found: git` or `python3` | Open a new terminal. Still failing? Install it — see step 2. |
| Download stops part-way | Run the same command again; it resumes. Three failures: use the USB. |
| Windows asks for Visual Studio / `link.exe` not found | You installed the MSVC toolchain. Re-run `rustup-init.exe` and choose **GNU**. |
| `cargo build` tries to download something | Add `--offline`. If it still fails, you have added a dependency — remove it. **This course allows none.** |
| `error: linker not found` (Linux) | `sudo apt install build-essential` |
| `lock file version 4 requires -Znext-lockfile-bump` | Your Rust is older than 1.78. Run `rustup update`. **Do not edit `Cargo.lock`** — it is a given file. |
| Anything else | Bring the exact error text to class. Not a description of it — the text. |

Do not spend hours stuck. Bring it to a Period 3 class; setup problems are the one
thing that is genuinely faster to fix in class.
