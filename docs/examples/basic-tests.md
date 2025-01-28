# Data Flows From Basic Tests

A collection of simple data flows from my test apps

## Sending 0x30 command with no preamble

The test connected to the thermometer and sent 0x01 as the first command.

| Write / notify? | Data                                       |
|-----------------|--------------------------------------------|
| Write           | `300030`                                   |
| Notify          | `E00230041600917D0000E9190020480000200200` |

No further notifications were generated.

## Sending a 0x01 command

0x01 was the first command sent. The thermometer generated a continuous sequence of 0x30 responses without any other
commands being sent.

| Write / notify? | Data                                       |
|-----------------|--------------------------------------------|
| Write           | `01097032e2c1799db4d1c7b1`                 |
| Notify          | `01010a0ce2c1799db4d1c7b10020c1799db4d1c7` |
| Notify          | `300f5a0c00ffffffffffff0222ffffffffbf0140` |
| Notify          | `300f5a0c00ffffffffffff0222ffffffffbf0140` |
| Notify          | `300f5a0c00ffffffffffff0222ffffffffbf0140` |
| Notify          | `300f5a0c00ffffffffffff0222ffffffffbf0140` |
| Notify          | `300f5a0c00ffffffffffff0222ffffffffbf0140` |
| Notify          | `300f5a0c00ffffffffffff0263ffffffff000140` |
| Notify          | `300f5a0c00ffffffffffff0291ffffffff2e0140` |
| Notify          | `300f5a0c00ffffffffffff0305ffffffffa30140` |
| Notify          | `300f5a0c00ffffffffffff0305ffffffffa30140` |
| Notify          | `300f5a0c00ffffffffffff0318ffffffffb60140` |
| Notify          | `300f5a0c00ffffffffffff0318ffffffffb60140` |
| Notify          | `300f5a0c00ffffffffffff0318ffffffffb60140` |
| Notify          | `300f5a0c00ffffffffffff0318ffffffffb60140` |
| Notify          | `300f5a0c00ffffffffffff0328ffffffffc60140` |
| Notify          | `300f5a0c00ffffffffffff0328ffffffffc60140` |
| Notify          | `300f5a0c00ffffffffffff0328ffffffffc60140` |
| Notify          | `300f5a0c00ffffffffffff0328ffffffffc60140` |
| Notify          | `300f5a0c00ffffffffffff0331ffffffffcf0140` |
| Notify          | `300f5a0c00ffffffffffff0331ffffffffcf0140` |
| ...             | seemingly forever                          |

# Deliberately sending a wrong checksum

Notice how the last nybble has changed from one to zero:

| Write / notify? | Data                                       |
|-----------------|--------------------------------------------|
| Write           | `01097032e2c1799db4d1c7b0`                 |
| Notify          | `e0020102e5c1799db4d1c7b00020480000200200` |

0xe0 response still appears to be an error response. The data is still not clear. No notifications follow.

# Sending a slightly different `0x01` command

This time I changed the second-to-last byte, but made sure the checksum was still correct:

| Write / notify? | Data                                       |
|-----------------|--------------------------------------------|
| Write           | `01097032e2c1799db4d1c6b0`                 |
| Notify          | `01010f11e2c1799db4d1c6b00020c1799db4d1c7` |

A seemingly valid response, but not followed by any other notifications.

# Sending `0x01` and `0x26`

I wondered if anything would change if I sent the 0x26 command after the initial command

| Write / notify? | Data                                       |
|-----------------|--------------------------------------------|
| Write           | `01097032e2c1799db4d1c7b1`                 |
| Notify          | `01010a0ce2c1799db4d1c7b10020c1799db4d1c7` |
| Write           | `260026`                                   |
| Notify          | `26050c0c5a030faf00001f1a0020480000200200` |
| Notify          | `300f5a0c00ffffffffffff0204ffffffffa10140` |                                          

It replied with a normal looking 0x26 response, but with the usual 0x30 notifications thereafter. Therefore "stop
temperature notifications" is not part of the 0x26 command.
