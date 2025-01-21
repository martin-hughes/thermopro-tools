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
