# Data Flows From Basic Tests

A collection of simple data flows from my test apps

All commands and responses are split with spaces according to the TLVC format.

## Sending 0x30 command with no preamble

The test connected to the thermometer and sent 0x01 as the first command.

| Write / notify? | Data                                           |
|-----------------|------------------------------------------------|
| Write           | `30 00 30`                                     |
| Notify          | `E0 02 3004 16 00917D0000E9190020480000200200` |

No further notifications were generated.

## Sending a 0x01 command

0x01 was the first command sent. The thermometer generated a continuous sequence of 0x30 responses without any other
commands being sent.

| Write / notify? | Data                                           |
|-----------------|------------------------------------------------|
| Write           | `01 09 7032e2c1799db4d1c7 b1`                  |
| Notify          | `01 01 0a 0c e2c1799db4d1c7b10020c1799db4d1c7` |
| Notify          | `30 0f 5a0c00ffffffffffff0222ffffffff bf 0140` |
| Notify          | `30 0f 5a0c00ffffffffffff0222ffffffff bf 0140` |
| Notify          | `30 0f 5a0c00ffffffffffff0222ffffffff bf 0140` |
| Notify          | `30 0f 5a0c00ffffffffffff0222ffffffff bf 0140` |
| Notify          | `30 0f 5a0c00ffffffffffff0222ffffffff bf 0140` |
| Notify          | `30 0f 5a0c00ffffffffffff0263ffffffff 00 0140` |
| Notify          | `30 0f 5a0c00ffffffffffff0291ffffffff 2e 0140` |
| Notify          | `30 0f 5a0c00ffffffffffff0305ffffffff a3 0140` |
| Notify          | `30 0f 5a0c00ffffffffffff0305ffffffff a3 0140` |
| Notify          | `30 0f 5a0c00ffffffffffff0318ffffffff b6 0140` |
| Notify          | `30 0f 5a0c00ffffffffffff0318ffffffff b6 0140` |
| Notify          | `30 0f 5a0c00ffffffffffff0318ffffffff b6 0140` |
| Notify          | `30 0f 5a0c00ffffffffffff0318ffffffff b6 0140` |
| Notify          | `30 0f 5a0c00ffffffffffff0328ffffffff c6 0140` |
| Notify          | `30 0f 5a0c00ffffffffffff0328ffffffff c6 0140` |
| Notify          | `30 0f 5a0c00ffffffffffff0328ffffffff c6 0140` |
| Notify          | `30 0f 5a0c00ffffffffffff0328ffffffff c6 0140` |
| Notify          | `30 0f 5a0c00ffffffffffff0331ffffffff cf 0140` |
| Notify          | `30 0f 5a0c00ffffffffffff0331ffffffff cf 0140` |
| ...             | seemingly forever                              |

# Deliberately sending a wrong checksum

Notice how the last nybble has changed from one to zero:

| Write / notify? | Data                                           |
|-----------------|------------------------------------------------|
| Write           | `01 09 7032e2c1799db4d1c7 b0`                  |
| Notify          | `e0 02 0102 e5 c1799db4d1c7b00020480000200200` |

0xe0 response still appears to be an error response. The data is still not clear. No notifications follow.

# Sending a slightly different `0x01` command

This time I changed the second-to-last byte, but made sure the checksum was still correct:

| Write / notify? | Data                                           |
|-----------------|------------------------------------------------|
| Write           | `01 09 7032e2c1799db4d1c6 b0`                  |
| Notify          | `01 01 0f 11 e2c1799db4d1c6b00020c1799db4d1c7` |

A seemingly valid response, but not followed by any other notifications.

# Sending `0x01` and `0x26`

I wondered if anything would change if I sent the 0x26 command after the initial command

| Write / notify? | Data                                           |
|-----------------|------------------------------------------------|
| Write           | `01 09 7032e2c1799db4d1c7 b1`                  |
| Notify          | `01 01 0a 0c e2c1799db4d1c7b10020c1799db4d1c7` |
| Write           | `26 00 26`                                     |
| Notify          | `26 05 0c0c5a030f af 00001f1a0020480000200200` |
| Notify          | `30 0f 5a0c00ffffffffffff0204fffffff fa 10140` |                                          

It replied with a normal looking 0x26 response, but with the usual 0x30 notifications thereafter. Therefore "stop
temperature notifications" is not part of the 0x26 command.
