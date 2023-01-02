# arg_parse for rust
### Description
arg_pare is a tool to simplify the processing of command line arguments. It doesn't have any dependencies and the initialization is done at compile time.

## Features
- [x] Parsing of `flags` (Values set with `--` which default is false and set to true by being used.)
- [x] Parsing of `parameters` (Values mentioned after `-` which have their value(as a string) followed)
- [ ] Parsing of `sub commands` (which only one can be used and all following arguments are related to)
- [ ] Returning results instead of throwing unfinished error messages
- [ ] Simple creation of parser
- [x] Ability to create parser as constant or static variable (at compile time)
- [ ] Ability to give of list of arguments (not using args from std::env::args)
