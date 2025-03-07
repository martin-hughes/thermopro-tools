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

* [Tools](#tools) - a quick description of the various parts of this package
* [Documentation](#documentation) - a link to more detailed docs about the thermometer

# Tools

* `tlv-check` - a tool I wrote to test my assumptions about the format of TP-25 data packets
* `checksum_test` - This takes a hex string and attempts to find any bytes that could be the checksum of the previous
  bytes.
* `controller` - attempts to program and get data back from a TP-25

# Documentation

[Basic documentation](docs/index.md)

# Acknowledgements

This project builds upon several open source libraries as noted [here](docs/os-acknowledgements.md)
