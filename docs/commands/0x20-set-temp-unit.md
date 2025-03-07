# 0x20 Set Temperature Unit

## Description

Sets the temperature unit (degrees C or F) that is displayed on the device.

The unit being displayed has no impact on the data being transmitted over the radio, which is always in degrees C.

## Example

* Command: `20 01 0c 2d`
* Response: `20 00 20`

## Command Format

```c
struct {
  uint8_t temp_units;
}
```

### `temp_units`:

One of:

* `0x0c`: Display degrees C on the device
* `0x0f`: Display degrees F on the device

Other values have not been seen so it is unknown what impact they would have on the device.

## Response Format

No data is sent in response to this command.
