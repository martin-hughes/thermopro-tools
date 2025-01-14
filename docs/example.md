# TP25 example data flow

The following was extracted from a packet capture of a TP25 being connected to the ThermoPro app, and then a single
probe being gently warmed up.

`handle` refers to the GATT handle being written or read from. For my TP25 they are as follows:

* 0x11: The writable characteristic's data attribute
* 0x13: The readable characteristic's data attribute
* 0x14: The readable characteristic's CCC (written to configure notifications to be sent)

| R/W/N  | Handle | Data                                        |
|--------|--------|---------------------------------------------|
| Write  | 0x14   | 0100 (0x01) - setup notifications from 0x13 |
| Write  | 0x11   | 01097032e2c1799db4d1c7b1                    |
| Notify | 0x13   | 01010a0ce2c1799db4d1c7b10020c1799db4d1c7    |
| Write  | 0x11   | 260026                                      |
| Notify | 0x13   | 26050c0c5a030faf0000071a0020480000200200    |
| Write  | 0x11   | 23060100ffffffff26                          |
| Notify | 0x13   | 2302010026ffffff260000450200384c0200ffff    |
| Write  | 0x11   | 23060200ffffffff27                          |
| Notify | 0x13   | 2302020027ffffff270000190020480000200200    |
| Write  | 0x11   | 23060300ffffffff28                          |
| Notify | 0x13   | 2302030028ffffff280000190020480000200200    |
| Write  | 0x11   | 23060400ffffffff29                          |
| Notify | 0x13   | 2302040029ffffff290000190020480000200200    |
| Write  | 0x11   | 24010126                                    |
| Notify | 0x13   | 24060100ffffffff2700ec190020480000200200    |
| Write  | 0x11   | 24010227                                    |
| Notify | 0x13   | 24060200ffffffff2800001a0020480000200200    |
| Write  | 0x11   | 24010328                                    |
| Notify | 0x13   | 24060300ffffffff2900141a0020480000200200    |
| Write  | 0x11   | 24010429                                    |
| Notify | 0x13   | 24060400ffffffff2a00281a0020480000200200    |
| Write  | 0x11   | 2401052a                                    |
| Notify | 0x13   | 24060500ffffffff2b00a1190020480000200200    |
| Write  | 0x11   | 2401062b                                    |
| Notify | 0x13   | 24060600ffffffff2c00b5190020480000200200    |
| Write  | 0x11   | 410041                                      |
|        |        | Bad CRC - miscaptured notify?               |
| Write  | 0x11   | 250025                                      |
| Notify | 0x13   | 250e0600ffffffffffff0223ffffffff54200200    |
|        |        | No initial 0x30 command here?               |
| Notify | 0x13   | 300f5a0c00ffffffffffff0222ffffffffbf0140    |
| Write  | 0x11   | 300030                                      |
| Notify | 0x13   | 300f5a0c00ffffffffffff0233ffffffffd00140    |
| Write  | 0x11   | 300030                                      |
|        |        | A connection params update happened here    |
| Notify | 0x13   | 300f5a0c00ffffffffffff0258fffffffff50140    |
| Write  | 0x11   | 300030                                      |
| Notify | 0x13   | 300f5a0c00ffffffffffff0281ffffffff1e0140    |
| Write  | 0x11   | 300030                                      |
| Notify | 0x13   | 300f5a0c00ffffffffffff0297ffffffff340140    |
| Write  | 0x11   | 300030                                      |
| Notify | 0x13   | 300f5a0c00ffffffffffff0310ffffffffae0140    |
| Write  | 0x11   | 300030                                      |
| Notify | 0x13   | 300f5a0c00ffffffffffff0325ffffffffc30140    |
| Write  | 0x11   | 300030                                      |
| Notify | 0x13   | 300f5a0c00ffffffffffff0309ffffffffa70140    |
| Write  | 0x11   | 300030                                      |
| Notify | 0x13   | 300f5a0c00ffffffffffff0293ffffffff300140    |
| Write  | 0x11   | 300030                                      |
| Notify | 0x13   | 300f5a0c00ffffffffffff0281ffffffff1e0140    |
