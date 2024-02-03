# Todo 

## Summary

This is a simple todo list app build to help learn the basics of Rust.
Purposefully not using a framework like Clap, want to solidify the basics first.

## Features

This is a simple CLI based Todo app
The commands are
- new (add a new todo)
- list (list all todos)
- complete

Example
```
$ todo add "Clean the garage"

$ todo list
[1] Go for a walk
[2] Clean the garage

$ todo done 1

$ todo list
[2] Clean the garage
```

The todos are stored in a file called `todos.json` in the current directory.