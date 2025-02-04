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
  uint8_t preamble[3];
  uint16_t temperatures[6];
}
```

### `preamble`:

Not understood.

### `temperatures`:

An array of [BCD temperature](../common-info.md#bcd-temperature-or-bcd-ish-temperature) details. The first 4 entries
correspond to probes 1-4 on the thermometer. I do not know what entries 5 and 6 correspond to, given that the TP25 has
4 probes.
