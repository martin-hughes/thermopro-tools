# 0xe0 Error

## Description

An `0xe0` command has never been seen in any traces - only `0xe0` responses. It's possible the app never generates an
`0xe0` command.

The response seems to be generated if an invalid command is sent to the thermometer.

## Example

* Command: Unknown
* Response: `E0 02 3004 16`

## Command Format

Unknown - this has never been observed.

## Response Format

```c
struct {
  uint8_t unknown[2];
}
```

I have no insights into the response.
