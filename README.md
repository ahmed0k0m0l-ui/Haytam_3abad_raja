# Rust Password Manager

A basic command-line password manager written in Rust.

## Features

* Add password entries
* Read stored password entries
* Store passwords in a local `password.txt` file
* Simple terminal interface

## Requirements

* Rust (`rustc`)

## Compile

```bash
rustc main.rs
```

## Run

### Linux / macOS

```bash
./main
```

### Windows

```bash
main.exe
```

## Project Structure

```text
password-manager/
├── main.rs
├── password.txt
└── README.md
```

## Future Improvements

* Search passwords by website
* Update passwords
* Delete passwords
* Encrypt passwords
* Use JSON for storage
* Add a master password
