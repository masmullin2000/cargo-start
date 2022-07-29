# cargo-start
A cargo plugin which creates a more rich starter project than cargo new

This plugin clones the repository found @ https://github.com/masmullin2000/rust_starter_project
It renames the project name and the binary name to a name you desire

## Installation
```shell
$ cargo install --git https://github.com/masmullin2000/cargo-start
```

## Usage
```shell
# Create a new Project named "my_awesome_rust_project"
$ cargo start my_awesome_rust_project

# if my_awesome_rust_project already exists use the -f flag
# for an interactive prompt to delete the old project
$ cargo start my_awesome_rust_project -f
!!WARNING!!
This will delete the current directory my_awesome_rust_project
Are you sure you wish to proceed? (type YES to proceed) : YES

# or use the -ff flag to force delete the old project
$ cargo start my_awesome_rust_project -ff
```
