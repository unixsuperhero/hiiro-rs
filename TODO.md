
# TODO

- write the code to trigger the subcmd code:
  - internal subcmd
  - external subcmd
- finish a runner for an actual (internal) subcmd

- can ArgMatches be passed to a Command?
  - how do i handle nested subcommands?

- just start building subcmds for the `note` subcmd
  - to see how it can be abstracted
- add infer method to Subcommand struct
- have builtin subcmds use the lib for determining in/external subcmds
- separate the builtin subcmds from the lib...and move to the bin
  - because they will move with the bin
- subcommands for listing/finding subcommands


## To Write About:

- the difference between a functional style vs. an oop style:
  - the oop style might create an instance of a class with all the Command
    mappings in it
  - the fp style uses the fn in place of the value, and it just
    initializes the data when the function is called, not ahead of the time
  - question: is the memory being allocated either way?

