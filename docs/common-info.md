# TP-25 Common data structures and names

This is a list and description of things that appear often in this documentation

## BCD temperature or BCD-ish temperature:

A two byte value where each nybble represents one base-ten digit of the temperature. Seems to always be in tenths of a
degree.

For example, `0x0354` sent over the wire corresponds to 35.4 Celsius.

## Checksum or `checksum`:

The mod-256 addition of each preceding byte in the data structure. That is, something like:

```c
uint8_t checksum = 0;
uint8_t *ptr = (uint8_t *)(&data_structure);

for (uint8_t i = 0; i < length_of_data_structure_without_checksum]; i++, ptr++) {
  checksum += *ptr; /* Assuming your compiler wraps integers like most do */
}
```

## `probe_index`:

A 1-based index of the probe to assign a profile to. Appears to match the numbers on the physical body of the
thermometer.

## TLVC format

TLVC is shorthand for "type, length, value, checksum". The format is (in a pseudo-C):

```c
struct {
  uint8_t type;          /* The command being sent or reponse being received */
  uint8_t length;        /* The length of the data being sent, may be zero */
  uint8_t value[length]; /* omitted if length == 0 */
  uint8_t checksum;      /* as described above */
  
  /* Only for responses, not included for commands: */
  uint8_t suffix[17-length]; /* Most likely junk */
}
```
