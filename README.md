```
████████╗██╗  ██╗ █████╗ ███╗   ██╗██╗██╗  ██╗
╚══██╔══╝██║  ██║██╔══██╗████╗  ██║██║╚██╗██╔╝
   ██║   ███████║███████║██╔██╗ ██║██║ ╚███╔╝
   ██║   ██╔══██║██╔══██║██║╚██╗██║██║ ██╔██╗
   ██║   ██║  ██║██║  ██║██║ ╚████║██║██╔╝ ██╗
   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝╚═╝  ╚═╝

```

## Welcome to Thanix!

Thanix is an experimental cli application written in Rust for generating Rust code from yaml schema files like they are
found as openAPI schemas.

## Installation

Be aware that Thanix currently is only developed on and for Linux distributions.<br>
Support for other operating systems may be available in the future, but currently though it is not guaranteed to work.

### Building from source

Building from source provides you with the most recent updates and changes to Thanix. However, be aware that these
may be unstable, so downloading a tagged release is advised.

Also, make sure you **have the Rust programming language and Cargo installed**.

To build Thanix from source you need to follow these steps:

1. Download the source code. To do so, run this command in your Terminal:

```bash
git clone git@github.com:The-Nazara-Project/Thanix.git
```

This will create a new directory called `Thanix`, move into it.

2. Run the application directly

You can now run Thanix using the `cargo run` command. However, this process may take longer and may feel less
comfortable.

**Do not forget to pass the required CLI parameters to Thanix when doing this.**

```bash
cargo run -- --input-file $PATH_TO_YOUR_YAML --uri $URI_TO_YOUR_API
```

This may look like this:

```bash
cargo run -- --input-file ./api_config.yaml --uri https://demo.netbox.dev
```

> **NOTE:** Make sure that you **do not end your URI with a slash (/)**. As this would mangle the API paths.

This step will result in your `thanix_client` being generated.

To view the next steps please scroll **down to the Usage section**.

Optional:

3. Install Thanix using `cargo install`.

You can also install the crate on your system, so you always have it available.
To do so, run this command while in the Thanix project directory:

```bash
cargo install --path .
```

This will install Thanix onto your system and it can be executed by simply running `thanix` in your terminal.

### Install Thanix using Cargo

Thanix is also published on [crates.io](https://crates.io).

To install it simply run:

```bash
cargo install thanix
```

### Install Cargo as a distribution package (TBA)

We are working on building packages for a variety of Linux distributions starting with `openSUSE Tumbleweed`.

We will update you as soon as we have news.

## Usage

After you have installed Thanix in a way you see fit, you use it by passing it **two mandatory parameters** like this:

```bash
thanix --input-file $YOUR_API_YAML --uri $YOUR_API_URI
```

- The `--input-file` parameter is a path to your `.yaml`-file you want to use as an input. This is usually the API
  schema file your want to generate a client for.
- The `--uri` is the URI to your API in a format like this: `https://demo.netbox.dev`.
  **Make sure this URI does not end with a `/`**