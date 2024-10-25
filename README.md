# todo_rust
## A CLI todo app written in Rust

A simple command-line to-do list manager written in Rust. This CLI tool allows users to add, list, remove, and mark tasks as done.

## Features

- Add new tasks to the list
- List all tasks
- Remove tasks by their index
- Mark tasks as completed
- Clear all tasks

## Getting Started

### Prerequisites

To compile and run this project, you need to have Rust installed. You can download it from [Rust's official website](https://www.rust-lang.org/).

### Installation

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/yourusername/todo_cli.git
cd todo_cli
```

Then, build the project with Cargo:

```bash
cargo build --release
```

To run the program directly, you can rename the output binary:

```bash
mv target/release/todo_cli todo_rust
```

### Running the CLI

You can run the program using `todo_rust`:

```bash
./todo_rust <command> <arguments>
```

## Usage

### Commands

The following commands are available:

1. **List**: Lists all tasks.
   ```bash
   ./todo_rust list
   ```

2. **Add**: Adds a new task to the list.
   ```bash
   ./todo_rust add "Your task description here"
   ```

3. **Remove**: Removes a task by its index.
   ```bash
   ./todo_rust remove <task_index>
   ```

4. **Done**: Marks a task as completed by its index.
   ```bash
   ./todo_rust done <task_index>
   ```

5. **Clean**: Clears all tasks from the list.
   ```bash
   ./todo_rust clean
   ```

### Example

Here’s an example of how to use the CLI:

1. Add a new task:
   ```bash
   ./todo_rust add "Finish Rust project"
   ```

2. List all tasks:
   ```bash
   ./todo_rust list
   ```

3. Mark the first task as done:
   ```bash
   ./todo_rust done 1
   ```

4. Remove a task by index:
   ```bash
   ./todo_rust remove 1
   ```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to enhance functionality or fix bugs.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

---
