# Jiyunet [![Travis CI Builds](https://travis-ci.org/jiyunet/jiyunet.svg?branch=master)](https://travis-ci.org/jiyunet/jiyunet)

Jiyunet is a distributed messageboard platform aiming to provide a safe, stable
platform for censorship-resistant communication and discussion.

Further documentation can be found in my brain.

*// TODO Fix this.*

## Components

* `core` : Basic primitives for IO, cryptographic signing, etc.

* `dag` : Defines blockchain structures and utils

* `db` : Storage abstraction layer

* `dht` : Distributed Hash Table for node discovery, etc.

* `node` : Actual node software (see below)

* `validation` : Validation engine used to verify new blocks, etc.

## Usage

### Building

```
cargo build --release node
```

This will build everything that you need to run `jiyud`.  It'll live somewhere
in the `target/` directory structure, but I still need to put together something
to install it in `/usr/bin`.  I have no plans for actively supporting Windows,
but there isn't (currently) any reason that it shouldn't find a way to run.

### Options

* `-p <port>`, `--bind-port=<port>` : Port to listen on.  Default: 8200

* `-d <dir>`, `--data-dir=<dir>` : Block data directory.  Default: `/var/lib/jiyunet`

* `-c <dir>`, `--cache-dir=<dir>` : Artifact cache directory.  Default: `/var/cache/jiyunet/artifact`

## Work Needed

* Validation engine is incomplete, currently only targeting to make sure that
	signatures are actually correct, not validating credit counts.

* None of the DHT code is complete.

* None of the node daemon is working.

* There is no way to limit the number of blocks published.

* There's no interface yet, so you actually can't create artifacts or blocks or
	anything without doing it by hand.

* We should have a Python wrapper around all the code in `dag` to make that a
	bit nicer to work with.

* Pretty much everything else, too.
