# hiiro-rs

This is the wip for a reimplementation of a cli tool I've made call
`hiiro` or `h` for short.

It's basically just a cli namespace for all of my frequently used
scripts/tools.

Eventually, it will behave similar to other popular cli tools like `git`,
where each command has optional subcommands, and it also allows for
external subcommands.

## External subcommands
If the subcommand is `asdf`, then it will look for a bin in PATH
called `h-asdf`.  You can find this behavior in:
- `git`
- `cargo`
- ...many more i'm sure.
