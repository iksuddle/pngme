# `pngme`

A CLI tool for encoding, decoding, and manipulating messages hidden in PNG files using custom chunks.
This is an implementation of the [pngme](https://jrdngr.github.io/pngme_book) project.

## Usage

```
encode - hide messages in PNG files using custom chunk types
decode - extract hidden messages from PNG files
remove - remove custom chunks from PNG files
print  - list all chunks in a PNG file
```

See `pngme help` for details.

## Installation

Clone the project and run
```bash
cargo install --path .
```

## Examples

```bash
// Encode a secret message
pngme encode image.png "ruSt" "This is a secret message" --output-file secret.png

// Decode the message
pngme decode secret.png "ruSt"

// Remove the chunk
pngme remove secret.png "ruSt"

// List all chunks
pngme print image.png
```
