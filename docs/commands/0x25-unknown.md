# 0x25 - Unknown

## Description

A command with unknown purpose. Only appears once per session when the app drives the thermometer. Has a
temperature-like response.

## Example

* Command: `25 00 25`
* Response: `25 0e 0600ffffffffffff0205ffffffff 36`

## Command Format

No parameters are sent with this command.

## Response Format

```c
struct {
  uint8_t unknown[2];
  uint16_t temperature[6]; /* very speculative */
}
```

Much like the [0x30 command](./0x30-send-temp-report.md) response there appears to be space for 6 temperatures. This is
untested though.
