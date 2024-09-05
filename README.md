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

[![build result](https://build.opensuse.org/projects/home:ByteOtter:nazara-project/packages/thanix/badge.svg?type=default)](https://build.opensuse.org/package/show/home:ByteOtter:nazara-project/thanix)

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
cargo run -- $PATH_TO_YOUR_YAML --output $PATH_TO_YOUR_OUTPUT
```

This may look like this:

```bash
cargo run --  ./api_config.yaml --output thanix_client/
```

This step will result in your `thanix_client` being generated.

To view the next steps please scroll **down to the Usage section**.

Optional:

1. Install Thanix using `cargo install`.

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

### Install Thanix as a distribution package (TBA)

We are working on building packages for a variety of Linux distributions starting with `openSUSE Tumbleweed`.

You can install Thanix for openSUSE Tumbleweed using these steps:

1. Add Repository
```bash
sudo zypper ar https://download.opensuse.org/repositories/home:/ByteOtter:/nazara-project/openSUSE_Tumbleweed/home:ByteOtter:nazara-project.repo
8m
```

2. Refresh Repositories
```bash
sudo zypper ref
```

3. Install Thanix
```bash
sudo zypper install Thanix
```

Zypper should now install Thanix for you.

> NOTE: The repository location is temporary and will be changed once Thanix is out of beta.
> If you do not wish to deal with this, we suggest you install Thanix using one of the methods listed above.
> Thank you for understanding.

## Usage

After you have installed Thanix in a way you see fit, you use it by passing it **two mandatory parameters** like this:

```bash
thanix  $YOUR_API_YAML --output thanix_client/
```

- The `$YOUR_API_YAML` parameter is the path to your `.yaml`-file you want to use as an input. This is usually the API
  schema file your want to generate a client for.
- The `--output` parameter is optional and refers to the path where thanix' output should be put. If omitted, it will
  create a `output` directory in your current wokring directory.
- The `--workaround` flag can be set to allow Thanix to create a **strongly opinionated** version of `thanix_client`. This is
  primarily used to avoid serialization errors when handling API object responses which we have confirmed to diverge from the
  values expected according to the schema.

> [!Note]
> The `--workaround` flag is only useful when creating a client for [`NetBox`](https://netbox.dev). In other cases it might produce 
> a broken or unsafe API client by weakening response data validation.

