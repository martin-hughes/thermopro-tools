# 0x24 Unknown

## Description

A command with unknown purpose. Sent 6 times per session, during startup, when the app drives the thermometer.

## Example

* Command: `24 01 02 27`                                  |
* Response: `24 06 0200ffffffff 28`

Since the response structure looks a bit like 0x23 set temperature profile, I suspect that this command may be "query
probe profile".

## Command Format

> This is speculative and untested.

```c
struct {
  uint8_t probe_index;
}
```

This value appears to be a probe index - although it runs up to 6 when the app is driving, although the thermometer
itself only has 4 probes. It therefore may not be a probe index.

## Response Format

> This is speculative and untested.

```c
struct {
  uint8_t probe_index;
  uint8_t unknown;
  uint16_t high_temp_alarm;
  uint16_t low_temp_alarm;
}
```

### `probe_index`

Always corresponds to `probe_index` in the command.

### Other fields

See the [command section for command 0x23](./0x23-set-probe-profile.md#command-format) for further details.
