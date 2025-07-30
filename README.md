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
* [Acknowledgements](#acknowledgements) - OS library acknowledgements

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

On PowerShell that looks like `$env:RUST_LOG = 'debug'`

## `http-server`

```shell
cargo run -p http-server
```

Exposes the following HTTP endpoints:

> There is no security implemented here. If you're going to use this code, I assume you'll be integrating it into some
> kind of homelab setup, where you have already implemented access control.

* `GET /state` - returns a JSON formatted copy of the state of the thermometer.
* `POST /mode` - Set the temperature mode (degrees C or F)
* `POST /alarm` - Set a temperature alarm
* `POST /alarm_ack` - Acknowledge an alarm after it has been triggered.
* `POST /custom_cmd` - Send a custom command to the thermometer
* `GET /ws` - Upgrade to Websockets. This sends the same data as `/state` each time something changes on the device.

Further details can be seen in the [http-server Readme](./http-server/README.md)

## `checkum_test` and `tlv-check`

Some checks / tests on checksum bytes.

```shell
> cargo run -p tp25-tlv-check --bin checksum_test 3300330044
Length: 2, checksum: 33, expected: 33, match? true
Length: 3, checksum: 66, expected: 0, match? false
Length: 4, checksum: 66, expected: 44, match? false
# or
> cargo run -p tp25-tlv-check --bin tlv-check 3300330044
Valid.          Type: 51, length: 0, data: []
Remainder.      Bytes: [0, 68]
```

The two programs are similar in that they are useful for investigating whether the thermometer has provided a valid TLVC
response. Both commands take a hex formatted string as a parameter, which is assumed to be TLVC.

`tlv-check` assumes that the 'length' byte is correct and tells you details about the TLVC string provided.

`checksum_test` progresses through the string, telling you whether the following byte would be a valid checksum for all
previous bytes (so in the example seen, the second 0x33 is a valid checksum for the preceding `0x3300`, but 0x44 is not
a valid checksum for the string `0x33003300`)

# Libraries

* `device-controller` - An Actor-like controller for the TP25, and associated functionality such as methods for finding
  and connecting to the device.

  > Further documentation (hopefully!) to follow.

# Protocol Documentation

I have written up my understanding of the TP25's protocol [here](docs/index.md)

# Acknowledgements

This project builds upon several open source libraries as noted [here](docs/os-acknowledgements.md)
