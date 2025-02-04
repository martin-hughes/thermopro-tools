# 0x25 Alternative temperature report

## Description

Causes the thermometer to respond with a temperature report, but not in 0x30 format. It's not clear why this is needed.

## Example

* Command: `25 00 25`
* Response: `25 0e 0600ffffffffffff0205ffffffff 36`

## Command Format

No parameters are sent with this command.

## Response Format

```c
struct {
  uint8_t unknown[2];
  uint16_t temperature[6];
}
```

### `unknown`

So far, always seems to be set to `{0x06, 0x00}`

### `temperature`

Much like the [0x30 command](./0x30-send-temp-report.md) response there appears to be space for 6 temperatures.
