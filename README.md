# Todo CLI — Rust

A simple interactive command-line todo app built in Rust. Tasks are stored in a local `todos.md` file using markdown checkbox format.

## Features

- Add tasks
- Delete tasks (completed or uncompleted)
- Toggle tasks between completed and uncompleted
- Auto-displays task list after every command
- Prevents duplicate tasks
- Persists tasks between sessions in `todos.md`

## Usage

Run the app:

```bash
cargo run
```

Or with the compiled binary:

```bash
./todo
```

### Commands

| Command         | Example           | Description                          |
| --------------- | ----------------- | ------------------------------------ |
| `add <task>`    | `add Buy milk`    | Adds a new task                      |
| `delete <task>` | `delete Buy milk` | Deletes a task                       |
| `done <task>`   | `done Buy milk`   | Toggles task between `[ ]` and `[x]` |
| `quit`          | `quit`            | Exits the app                        |

## Tasks format in todos.md

```markdown
- [ ] Buy milk
- [x] Study Rust
- [ ] Clean kitchen
```

## Build

```bash
cargo build --release
./target/release/todo
```

## What I learned building this

- Structs, enums, and pattern matching with `match`
- Vectors, HashMaps and common collection methods
- File I/O with `std::fs`
- Error handling with `Option` and `Result`
- Ownership and borrowing
- CLI input with `std::io` and `std::env`
- Closures and iterators (`.map()`, `.filter()`, `.collect()`)

## Built with

- Rust
- Standard library only — no external dependencies
