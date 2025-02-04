# App-driven TP25 data flow

The following was extracted from a packet capture of a TP25 being connected to the ThermoPro app, and then a single
probe being gently warmed up.

`handle` refers to the GATT handle being written or read from. For my TP25 they are as follows:

* 0x11: The writable characteristic's data attribute
* 0x13: The readable characteristic's data attribute
* 0x14: The readable characteristic's CCC (written to configure notifications to be sent)

Throughout all these tables, all Data entries are split into TLV fields with notifications also showing the (likely)
junk suffix.

## Basic data flow

App connects to the thermometer, receives temperature indications. No changes made in app.

| R/W/N  | Handle | Data                                           |
|--------|--------|------------------------------------------------|
| Write  | 0x14   | `0100` (0x01) - setup notifications from 0x13  |
| Write  | 0x11   | `01 09 7032e2c1799db4d1c7 b1`                  |
| Notify | 0x13   | `01 01 0a 0c e2c1799db4d1c7b10020c1799db4d1c7` |
| Write  | 0x11   | `26 00 26`                                     |
| Notify | 0x13   | `26 05 0c0c5a030f af 0000071a0020480000200200` |
| Write  | 0x11   | `23 06 0100ffffffff 26`                        |
| Notify | 0x13   | `23 02 0100 26 ffffff260000450200384c0200ffff` |
| Write  | 0x11   | `23 06 0200ffffffff 27`                        |
| Notify | 0x13   | `23 02 0200 27 ffffff270000190020480000200200` |
| Write  | 0x11   | `23 06 0300ffffffff 28`                        |
| Notify | 0x13   | `23 02 0300 28 ffffff280000190020480000200200` |
| Write  | 0x11   | `23 06 0400ffffffff 29`                        |
| Notify | 0x13   | `23 02 0400 29 ffffff290000190020480000200200` |
| Write  | 0x11   | `24 01 01 26`                                  | 
| Notify | 0x13   | `24 06 0100ffffffff 27 00ec190020480000200200` |
| Write  | 0x11   | `24 01 02 27`                                  |
| Notify | 0x13   | `24 06 0200ffffffff 28 00001a0020480000200200` |
| Write  | 0x11   | `24 01 03 28`                                  |
| Notify | 0x13   | `24 06 0300ffffffff 29 00141a0020480000200200` |
| Write  | 0x11   | `24 01 04 29`                                  | 
| Notify | 0x13   | `24 06 0400ffffffff 2a 00281a0020480000200200` |
| Write  | 0x11   | `24 01 05 2a`                                  |
| Notify | 0x13   | `24 06 0500ffffffff 2b 00a1190020480000200200` |
| Write  | 0x11   | `24 01 06 2b`                                  |
| Notify | 0x13   | `24 06 0600ffffffff 2c 00b5190020480000200200` |
| Write  | 0x11   | `41 00 41`                                     |
|        |        | Bad CRC - miscaptured notify?                  |
| Write  | 0x11   | `25 00 25`                                     |
| Notify | 0x13   | `25 0e 0600ffffffffffff0223ffffffff 54 200200` |
|        |        | No initial 0x30 command here?                  |
| Notify | 0x13   | `30 0f 5a0c00ffffffffffff0222ffffffff bf 0140` |
| Write  | 0x11   | `30 00 30`                                     |
| Notify | 0x13   | `30 0f 5a0c00ffffffffffff0233ffffffff d0 0140` |
| Write  | 0x11   | `30 00 30`                                     |
|        |        | A connection params update happened here       |
| Notify | 0x13   | `30 0f 5a0c00ffffffffffff0258ffffffff f5 0140` |
| Write  | 0x11   | `30 00 30`                                     |
| Notify | 0x13   | `30 0f 5a0c00ffffffffffff0281ffffffff 1e 0140` |
| Write  | 0x11   | `30 00 30`                                     |
| Notify | 0x13   | `30 0f 5a0c00ffffffffffff0297ffffffff 34 0140` |
| Write  | 0x11   | `30 00 30`                                     |
| Notify | 0x13   | `30 0f 5a0c00ffffffffffff0310ffffffff ae 0140` |
| Write  | 0x11   | `30 00 30`                                     |
| Notify | 0x13   | `30 0f 5a0c00ffffffffffff0325ffffffff c3 0140` |
| Write  | 0x11   | `30 00 30`                                     |
| Notify | 0x13   | `30 0f 5a0c00ffffffffffff0309ffffffff a7 0140` |
| Write  | 0x11   | `30 00 30`                                     |
| Notify | 0x13   | `30 0f 5a0c00ffffffffffff0293ffffffff 30 0140` |
| Write  | 0x11   | `30 00 30`                                     | 
| Notify | 0x13   | `30 0f 5a0c00ffffffffffff0281ffffffff 1e 0140` |

## Setting a temperature profile

App connects to the thermometer. Temperature profile is selected, then changed, then set back to "no profile".

I've omitted the handles because they appear to be constant.

* This is the first capture where 0x30 responses come before the command - although this matches the fake app's
  observations
* 0x23 command/response seem to set the temperature profile.

| W/N?   | Data                                           |
|--------|------------------------------------------------|
| Write  | `01 09 b03fc1e879ee9d6d77 8a`                  |
| Notify | `01 01 0a0c c1e879ee9d6d778a0020e879ee9d6d77`  |
| Write  | `26 00 26`                                     |
| Notify | `26 05 0c0c5a030f af 0000f3190020480000200200` |
| Write  | `23 06 0100ffffffff 26`                        |
| Notify | `23 02 0100 26 ffffff260000e000e0f1ffffff384c` |
| Write  | `23 06 0200ffffffff 27`                        |
| Notify | `23 02 0200 27 ffffff2700001a0020480000200200` |
| Write  | `23 06 0300ffffffff 28`                        |
| Notify | `23 02 0300 28 ffffff280000e000e0f1ffffff0000` |
| Write  | `23 06 0400ffffffff 29`                        |
| Notify | `23 02 040029ffffff 29 0000190020480000200200` |
| Write  | `24 01 01 26`                                  |
| Notify | `24 06 0100ffffffff 27 00d3190020480000200200` |
| Write  | `24 01 02 27`                                  |
| Notify | `24 06 0200ffffffff 28 00e7190020480000200200` |
| Write  | `24 01 03 28`                                  |
| Notify | `24 06 0300ffffffff 29 00fb190020480000200200` |
| Write  | `24 01 04 29`                                  |
| Notify | `24 06 0400ffffffff 2a 000f1a0020480000200200` |
| Write  | `24 01 05 2a`                                  |
| Notify | `24 06 0500ffffffff 2b 00231a0020480000200200` |
| Notify | `30 0f 5a0c00ffffffffffff0185ffffffff 21 0140` |
| Write  | `24 01 06 2b`                                  |
| Notify | `24 06 0600ffffffff 2c 00a1190020480000200200` |
| Write  | `30 00 30`                                     |
| Write  | `41 00 41`                                     |
| Notify | `41 02 3111 85 00917d0000c8190020480000200200` |
| Write  | `25 00 25`                                     |
| Notify | `25 0e 0600ffffffffffff0185ffffffff b5 200200` |
|        | Connection parameters update                   |
| Notify | `30 0f 5a0c00ffffffffffff0186ffffffff 22 0140` |
| Write  | `30 00 30`                                     |
| Write  | `23 06 040507400000 79` (Lamb 74C)             |
|        | Miscaptured packets                            |
| Write  | `23 06 040607100000 4a` (Pork 71C)             |
| Notify | `23 02 0406 2f 1000004a0003190020480000200200` |
| Write  | `23 06 0400ffffffff 29` (No profile)           |
|        | Took 5 attempts before write ack'd             |
| Notify | `23 02 0400 29 ffffff290003190020480000200200` |

## Setting a range profile and triggering both alarms

App connects to the thermometer. Range temperature profile named OAT is selected, with range 25-35C. Temperature raised
a little. Range changed to 25-29C. Temperature varied to allow alarm to be triggered at high and low temperature.

I forgot to note whether I cancelled the alarm in the app or on the device :roll_eyes:

> The repeated write commands are retransmissions (confirmed by looking at the relevant flag in Wireshark)

| W/N?   | Data                                           |
|--------|------------------------------------------------|
| Write  | `01 09 4c40d13cee3a246d29 85`                  |
| Notify | `01 01 0a 0c d13cee3a246d298500203cee3a246d29` |
| Write  | `26 00 26`                                     |
| Notify | `26 05 0c0c5a030f af 0000071a0020480000200200` |
| Write  | `23 06 0100ffffffff 26`                        |
| Notify | `23 02 0100 26 ffffff260000e000e0f1ffffff384c` |
| Write  | `23 06 0200ffffffff 27`                        |
| Notify | `23 02 0200 27 ffffff270000190020480000200200` |
| Write  | `23 06 0300ffffffff 28`                        |
| Notify | `23 02 0300 28 ffffff280000e000e0f1ffffff0000` |
| Write  | `23 06 0400ffffffff 29`                        |
| Notify | `23 02 0400 29 ffffff290000190020480000200200` |
| Write  | `24 01 01 26`                                  |
| Notify | `24 06 0100ffffffff 27 00ec190020480000200200` |
| Write  | `24 01 02 27`                                  |
| Notify | `24 06 0200ffffffff 28 00001a0020480000200200` |
| Write  | `24 01 03 28`                                  |
| Notify | `24 06 0300ffffffff 29 00141a0020480000200200` |
| Write  | `24 01 04 29`                                  |
| Notify | `24 06 0400ffffffff 2a 00281a0020480000200200` |
| Write  | `24 01 05 2a`                                  |
| Notify | `24 06 0500ffffffff 2b 00a1190020480000200200` |
| Write  | `24 01 06 2b`                                  |
| Notify | `24 06 0600ffffffff 2c 00b5190020480000200200` |
| Write  | `41 00 41`                                     |
| Notify | `41 02 3111 85 00917d0000c9190020480000200200` |
| Write  | `25 00 25`                                     |
| Notify | `25 0e 0600ffffffffffff0205ffffffff 36 200200` |
| Notify | `30 0f 5a0c00ffffffffffff0225ffffffff c2 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c00ffffffffffff0241ffffffff de 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c00ffffffffffff0245ffffffff e2 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c00ffffffffffff0241ffffffff de 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c00ffffffffffff0238ffffffff d5 0140` |
| Write  | `30 00 30`                                     |
| Write  | `23 06 04fa03500250 cc`                        |
| Write  | `23 06 04fa03500250 cc`                        |
| Write  | `23 06 04fa03500250 cc`                        |
| Write  | `23 06 04fa03500250 cc`                        |
| Write  | `23 06 04fa03500250 cc`                        |
| Notify | `23 02 04fa 23 500250cc0003190020480000200200` |
| Notify | `30 0f 5a0c00ffffffffffff0236ffffffff d3 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c00ffffffffffff0249ffffffff e6 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c00ffffffffffff0260ffffffff fd 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c00ffffffffffff0277ffffffff 14 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c00ffffffffffff0290ffffffff 2d 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c00ffffffffffff0303ffffffff a1 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c00ffffffffffff0313ffffffff b1 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c00ffffffffffff0295ffffffff 32 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c00ffffffffffff0285ffffffff 22 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c00ffffffffffff0270ffffffff 0d 0140` |
| Write  | `30 00 30`                                     |
| Write  | `23 06 04fa02900250 0b`                        |
| Write  | `23 06 04fa02900250 0b`                        |
| Write  | `23 06 04fa02900250 0b`                        |
| Write  | `23 06 04fa02900250 0b`                        |
| Notify | `30 0f 5a0c00ffffffffffff0264ffffffff 01 0140` |
| Notify | `23 02 04fa 23 9002500b00031a0020480000200200` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c00ffffffffffff0281ffffffff 1e 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c08ffffffffffff0294ffffffff 39 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c08ffffffffffff0305ffffffff ab 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c08ffffffffffff0315ffffffff bb 0140` |
| Write  | `27 00 27`                                     |
| Write  | `30 00 30`                                     |
| Notify | `27 00 27 000000917d0000da190020480000200200`  |
| Notify | `30 0f 5a0c00ffffffffffff0292ffffffff 2f 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c00ffffffffffff0282ffffffff 1f 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c00ffffffffffff0268ffffffff 05 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c00ffffffffffff0256ffffffff f3 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c08ffffffffffff0249ffffffff ee 0140` |
| Write  | `30 00 30`                                     |
| Write  | `27 00 27`                                     |
| Notify | `27 00 27 000000917d0000c7190020480000200200`  |
| Notify | `30 0f 5a0c00ffffffffffff0249ffffffff e6 0140` |
| Write  | `30 00 30`                                     |
| Notify | `30 0f 5a0c00ffffffffffff0241ffffffff de 0140` |
| Write  | `30 00 30`                                     |
| Write  | `23 06 04fa02900250 0b`                        |
| Notify | `23 02 04fa 23 9002500b00001a0020480000200200` |
| Notify | `30 0f 5a0c00ffffffffffff0235ffffffff d2 0140` |
| Write  | `30 00 30`                                     |

