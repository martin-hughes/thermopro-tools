# 0x26 Unknown

## Description

A command with unknown purpose. Only appears once per session when the app drives the thermometer.

## Example

* Command: `26 00 26`
* Response: `26 05 0c0c5a030f af`

## Command Format

No parameters are sent with this command.

## Response Format

```c
struct {
  uint8_t unknown[5];
}
```

I have no insights into the response.
