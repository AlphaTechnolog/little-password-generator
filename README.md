# password generator

Simple password generate in which im not sure if it should work as how it's working right now, but it worsk so imma leave it
like that (implemented in rust).

Like right now it does the next process:

- asks for a little phrase to the user
- uses that phrase to generate the md5 hash using a secret "SECRET" (todo: should read from .env)
- asks for a description for the password to set it as a key for the password on the "database"
- stores the md5 on the "database" (which is just a plain text file)

## Installation

Make sure to have rust installed (obviously ._.)

```sh
git clone https://github.com/alphatechnolog/little-password-generator.git ~
cd ~/little-password-generator
cargo build --release
cargo install --path .
```

## Usage

```sh
password-generator
```

> Yeah, just that, no flags, nothing.

## HELP ME :despair:

So im learning rust, and i wanna improve, so feel free to add PRs or issues to suggest me how to improve
the code, or how to make it efficient asf anyways, im accepting every type of contributions to this little learning project :P
