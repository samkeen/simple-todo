# Todo 

## Summary
This is an extremely simple todo list app build to help learn the basics of Rust.

Purposefully not using a CLI framework like Clap.  Rather I want to solidify the basics first.

## Usage

Example
```
❯ todo
No command given
USAGE: todo <command> [arg]
Known commands:
todo add "todo description"
todo list
done <id of todo>


❯ todo list
== Existing todos ==
[afce43] this is a test todo

❯ todo add "learn Rust"
Added Todo[633d00] 'learn Rust'

❯ todo list
== Existing todos ==
[afce43] this is a test todo
[633d00] learn Rust
```

The todos are stored in a file called `todos.json` in the current directory.