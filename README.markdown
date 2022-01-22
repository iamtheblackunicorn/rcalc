# RCALC :checkered_flag: :abacus:

***Do computations from the command-line.*** :checkered_flag: :abacus:

![GitHub CI](https://github.com/iamtheblackunicorn/rcalc/actions/workflows/rust.yml/badge.svg)

## About :books:

I'm still learning Rust and wanted to write something that would allow me to make calculations from the command-line. Enjoy.

## Building :hammer: :pick:

You will need the following tools installed and available:

- Rust
- Git
- Make

To compile `rcalc`, follow these steps:

- 1.) Get the source code:
```bash
$ git clone https://$YOUR_GITHUB_TOKEN@github.com/iamtheblackunicorn/rcalc.git
```
- 2.) Change directory:
```bash
$ cd rcalc
```
- 3.) Build the source code:
```bash
$ make build
```

## Installation :inbox_tray:

Move the executable on the path `rcalc/target/release/rls` to the directory where you keep your binary executables. If you are on Linux or Mac OSX, you might have to change permissions like this: `chmod a+x rcalc`.

## Usage :book:

Using `rcalc` is quite simple:
- Do an addition:
```bash
$ rcalc 4 5 +
```
- Do a subtraction:
```bash
$ rcalc 4 5 -
```
- Do a multiplication:
```bash
$ rcalc 4 5 x
```
- Do a division:
```bash
$ rcalc 4 5 /
```


## Changelog :black_nib:

### Version 1.0

- initial release
- upload to GitHub

## Note :scroll:

- *RCALC :checkered_flag: :abacus:* by Alexander Abraham :black_heart: a.k.a. *"The Black Unicorn" :unicorn:*
- Licensed under the MIT license.
