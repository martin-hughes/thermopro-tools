# 0x41 Unknown

## Description

A command with unknown purpose. Only appears once per session when the app drives the thermometer.

This command must be replied to, or the ThermoPro App will refuse to communicate with the thermometer.

## Example

* Command: `41 00 41`
* Response: `41 02 3111 85`

## Command Format

No parameters are sent with this command.

## Response Format

```c
struct {
  uint8_t unknown[2];
}
```

I have no insights into the response.
