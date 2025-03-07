# 0x30 Send Temperature Report

## Description

The command is sent by the app periodically. The thermometer shortly afterwards responds with the response.

However, the thermometer will also send 0x30 responses periodically even if the command is not sent. It's possible the
command can prompt a quicker response, but I don't know for sure - I don't know when the thermometer sends unsolicited
temperature responses.

## Example

* Command: `30 00 30`
* Response: `30 0f 5a0c00ffffffffffff0325ffffffff c3`

## Command Format

No parameters are sent with this command.

## Response Format

```c
struct {
  uint8_t unknown;
  uint8_t temperature_mode;
  uint8_t alarm_status;
  uint16_t temperatures[6];
}
```

### `unknown`:

Not understood. This value does seem to change in different traces.

### `temperature_mode`:

If `0x0c` is sent, the thermometer is currently displaying degrees C. If `0x0f` is sent, the device is displaying
degrees F. No other values have been observed.

### `alarm_status`:

Takes one of the following two values:

* `0x00` - temperature is in the normal range
* `0x08` - alarm has been triggered

The thermometer will set `alarm_status` back to zero if the alarm is cancelled by pushing the button on the thermometer.

### `temperatures`:

An array of [BCD temperature](../common-info.md#bcd-temperature-or-bcd-ish-temperature) details. The first 4 entries
correspond to probes 1-4 on the thermometer. I do not know what entries 5 and 6 correspond to, given that the TP25 has
4 probes. So far it appears that probes five and six always report `0xffff`.
