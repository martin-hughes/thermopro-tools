# 0x24 Report Probe Profile

## Description

Instructs the thermometer to report the profile associated with a probe back to the app. It appears that after startup,
the device will report profiles set previously but will not alarm for them until they have been set again using the
[0x23 command](./0x23-set-probe-profile.md). That is, profiles seem to be stored but inactive after a reboot.

## Example

* Command: `24 01 04 29`
* Response: `24 06 04fa02900250 0c`

## Command Format

```c
struct {
  uint8_t probe_index;
}
```

When the app is driving it happily queries for probes 1-6. This does match the 0x30 temperature response which sends
temperatures for 6 probes, even if two are invalid.

## Response Format

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
