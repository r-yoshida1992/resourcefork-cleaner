# Resource Fork Cleaner
This Rust program monitors a specified directory for any changes and removes files with a certain prefix. It uses the notify crate to watch for changes in the directory and the chrono crate to log the time of the changes.

## Functionality
The program sets up a watcher on the specified directory (in this case /Volumes/T7) and continuously listens for changes. When a change event such as file creation, modification, or renaming is detected, it checks for files in the parent directory of the changed file that start with the ._ prefix, and removes them. It also logs the time of the removal and the path of the file removed.

## Usage
Make sure you have Rust installed.
Clone the repository and navigate to the project directory.
Run cargo build --release to compile the program.
Run the executable found in the target/release directory.
```sh
cargo build --release
cd target/release
./resourcefork-cleaner
```

Note: The program will start monitoring the /Volumes/T7 directory by default. Modify the source code to monitor a different directory.


