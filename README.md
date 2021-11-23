# Slidmk

Create presentations from a very simple file format
with styles and templates for the slides specified in a .hjson file.

# Motivation

Big applications like Libreoffice/Powerpoint are complex and
hard to use - the many options are distracting.
Similar apps (like Latex Beamer) don't let you customize the 
presentation as much as I would like to, so I took it into my own hands.

# Installation

1. install [rust](https://www.rust-lang.org/tools/install)
1. install the nightly version of rust with `rustup default nightly`
1. clone this repository
1. run `cargo install --path=.` inside the repository folder
1. make sure .cargo/bin is inside your PATH

# Usage

Create a presentation file (a little introduction to the format can be found at `example/introduction.present`),
then run `slidmk <present_file>` to convert it to a _.pdf_. The .pdf file can be found at `./out.pdf`.

If you want to change the name or directory of the output file, use the -o or --output argument like this:
`slidmk example.present -o /this/is/the/output.pdf`.

To use another style config, use the -s or --style argument like this:
`slidmk -s /path/to/style.hjson example.present`

To add more templates, use the -t or --templates flag to add multiple files like this:
`slidmk -t template.hjson /path/to/another.hjson ./and/another/one.hjson -- example.present`

For more information about the templates and styles do look inside the `example` directory for examples.

# License

All the code in this repository is licensed under the **apache-2.0** license,
however, distributed binaries need to respect all the licenses of the used libraries,
due to rust's static linking.
