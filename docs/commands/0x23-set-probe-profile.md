# 0x23 Set Probe Profile

## Description

Sets the "profile" associated with a probe.

A profile is the temperature it will alarm at. There are two broad types of profile:

* High temperature - alarms only when the temperature is higher than the threshold.
* Range - alarms if the temperature is higher than the high temperature alarm, or lower than the low temperature
  alarm. (that is, the temperature should remain within the range)

## Example

* Command: `23 06 04fa02900250 0b`
* Response: `23 02 04fa 23`

## Command Format

```c
struct {
  uint8_t probe_index;
  uint8_t unknown;
  uint16_t high_temp_alarm;
  uint16_t low_temp_alarm;
}
```

### `probe_index`:

A normal [probe_index](../common-info.md#probe_index).

### `unknown`:

Unknown, but does vary depending on which profile is being set. May correspond to an index for the name of the profile
in the app?

Always set to zero if the profile is being cleared - that is, there are no longer alarms associated with the probe.

### `high_temp_alarm`:

A [BCD coded temperature](../common-info.md#bcd-temperature-or-bcd-ish-temperature). Set for both high temperature and
range profiles. Set to `0xffff` is the profile is being
cleared.

### `low_temp_alarm`:

A [BCD coded temperature](../common-info.md#bcd-temperature-or-bcd-ish-temperature). Set only for range alarms. If a
high temperature alarm is being set, this field is set to `0x0000`. If the profile is being cleared, it is set to
`0xffff`.

## Response Format

```c
struct {
  uint8_t probe_index;
  uint8_t unknown;
}
```

### `probe_index`

A normal [probe_index](../common-info.md#probe_index). Corresponds to the probe index in the original command.

### `unknown`

Seems to always match `unknown` as sent in the command.
