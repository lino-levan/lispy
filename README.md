# Lispy

A lisp-like language with an implementation written in Rust.

## Usage

Open a repl with:

```bash
$ lispy repl
```

Run a file with:

```bash
$ lispy run ./examples/hello_world.l
```

Format all files in the local directory with:

```bash
$ lispy fmt
```

## Syntax

This language is very inspired by lisp, so the syntax is very simple. I find
lisp syntax far too terse for my liking so I generally tried to simplify it if
possible. You can check out the `examples` folder for how to use the language,
but here is a short demo:

```lisp
(var first 0)
(var second 1)
(while (< first 610)
	(print "[fib] " first)
	(var temp (+ first second))
	(var first second)
	(var second temp)
)
```
