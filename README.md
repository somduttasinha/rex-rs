# Rex

A fast and simple CLI tool to kickstart your latex project, using your own
templates, written in Rust. Rex is still in development, and many desired
features are not yet available. A full list of features and planned features
can be found in the [features](#features) section.

## Installation

### From source

Before building from source, please ensure that you have [installed cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).
Clone the repository and run the following command in the root directory of the
repository:

```bash
cargo install --path .
```

### From binary

TODO

## Usage

Rex looks for templates in `$REX_ROOT/templates`. Please set the `REX_ROOT`
environment variable to the directory where your templates are stored.

To create a new latex project, run the following command:

```
rex new -n <project_name> -t <template_name> <output_directory>
```

Where `<project_name>` is the name of the project, `<template_name>` is the name
of the template to use, and `<output_directory>` is the directory where the
project will be created.

## Example

To create a new project named `example-assignment` using the template
`assignment` in the same directory, run the following command:

```
rex new -n example-assignment -t assignment .
```

## Templates

TODO: create separate repository for templates.

## Features

- [ ] Read values such as the user's name and user's email from environment
      variables.
- [ ] Command to install templates from a git repository and intelligently
      configure it for use with rex.
