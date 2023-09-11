[![Stars](https://img.shields.io/github/stars/proxin187/argin.svg?style=for-the-badge)](https://github.com/proxin187/argin/stargazers)
[![Forks](https://img.shields.io/github/forks/proxin187/argin.svg?style=for-the-badge)](https://github.com/proxin187/argin/forks)

# Argin

A very simple command line argument parser.

<li>
    <a href="#Description">Description</a>
</li>
<li>
    <a href="#getting-started">Getting Started</a>
    <ul>
    <li><a href="#Usage">Usage</a></li>
    <li><a href="#Example">Example</a></li>
    <li><a href="#Functions">Functions</a></li>
    </ul>
</li>
<li><a href="#Help">Help</a></li>
<li><a href="#Authors">Authors</a></li>
<li><a href="#Versions">Versions</a></li>
<li><a href="#License">License</a></li>

## Description

Argin is a very simple library with only 5 functions but its still able to do all the expected argument parsing

## Getting Started

### Usage


#### Example
`./program [file] [-format (name)]`
would look like this:
```rust
use argin::Argin;

let arg = Argin::new();
arg.add_positional_arg();
arg.add_value("-format");
let args = arg.parse();

let file = args.pos_arg.get(0).unwrap();
let format = arg.values.get("-format").unwrap();
```

#### Functions

new:
```
pub fn new() -> Argin
```
add_flag:
```
pub fn add_flag(&mut self, flag: &str)
```
add_value:
```
pub fn add_value(&mut self, name: &str)
```
add_positional_arg:
```
pub fn add_positional_arg(&mut self)
```
parse:
```
pub fn parse(&self) -> Argin
```

## Help

Its common to forget .parse() at the end

## Authors

Contributors names and contact info

* [Proxin](https://github.com/proxin187)

## Versions

* 0.1
    * Initial Release

## License

Currently there is no license, this may change in the future


