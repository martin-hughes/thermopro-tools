# ThermoPro TP25 tools

A selection of things I've found useful whilst trying to reverse-engineer a TP25.

> For the full story, [see my blog](https://martys.blog/posts/thermopro)

For various reasons, I really want to be able to communicate with my
[ThermoPro TP25 meat thermometer](https://buythermopro.com/product/tp25/) without needing to use the app. This repo
contains various tools and docs that I've been writing as I go along.

Currently, the code part of the repo is a bit of a mess. It's because I've been hacking it together as I go along!
Eventually I'll make it tidier - or please feel free to submit a PR!

# Disclaimer

I have gathered this information by reverse engineering packet captures of data sent between the thermometer and its
companion app. I think this is acceptable in most jurisdictions, but in your jurisdiction it may not be acceptable to
use this information to build new products.

I accept no liability if you get in trouble for reading further - you have been warned!

# Contents

* [Tools](#tools--executables) - a quick description of the various executables in this workspace
* [Libraries](#libraries) - a quick description of the library in this workspace
* [Documentation](#protocol-documentation) - a link to more detailed docs about the thermometer

# Tools / Executables

* `cursive-ui` - A text based UI that can display temperatures and control alarms on the thermometer.
* `http-server` - A HTTP and Websockets interface to the thermometer
* `checksum_test` - This takes a hex string and attempts to find any bytes that could be the checksum of the previous
  bytes.
* `tlv-check` - a tool I wrote to test my assumptions about the format of TP-25 data packets

## `cursive-ui`

This is the default workspace member. Use a straightforward `cargo run` to execute it.

Cursive provides mouse support, so the menu items are clickable.

If no temperatures are being displayed, try making the terminal window larger (there is no message to say that it is too
small)

You may wish to set the environment variable `RUST_LOG` to `debug` or stricter to avoid log spam. Use the tilde `~` key
to view debug logs.

On Powershell that looks like `$env:RUST_LOG = 'debug'`

## `http-server`

```shell
cargo run -p http-server
```

Exposes the following HTTP endpoints:

> Further details can be found in the source, until I get around to better docs

> There is no security implemented here. If you're going to use this code, I assume you'll be integrating it into some
> kind of homelab setup, where you have already implemented access control.

* `GET /state` - returns a JSON formatted copy of the state of the thermometer.
* `POST /mode` - Set the temperature mode (degrees C or F)
* `POST /alarm` - Set a temperature alarm
* `POST /alarm_ack` - Acknowledge an alarm after it has been triggered.
* `POST /custom_cmd` - Send a custom command to the thermometer
* `GET /ws` - Upgrade to Websockets. This sends the same data as `/state` each time something changes on the device.

## `checkum_test`

> For now, details are in the source code

```shell
cargo run -p tp25-tlv-check --bin checksum_test
```

## `tlv-check`

> For now, details are in the source code.

```shell
cargo run -p tp25-tlv-check --bin tlv-check
```

# Libraries

* `device-controller` - An Actor-like controller for the TP25, and associated functionality such as methods for finding
  and connecting to the device.

  > Further documentation (hopefully!) to follow.

# Protocol Documentation

[Basic documentation](docs/index.md)

# Acknowledgements

This project builds upon several open source libraries as noted [here](docs/os-acknowledgements.md)
