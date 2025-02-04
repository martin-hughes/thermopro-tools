# App-driven TP25 data flow

The following was extracted from a packet capture of a TP25 being connected to the ThermoPro app, and then a single
probe being gently warmed up.

`handle` refers to the GATT handle being written or read from. For my TP25 they are as follows:

* 0x11: The writable characteristic's data attribute
* 0x13: The readable characteristic's data attribute
* 0x14: The readable characteristic's CCC (written to configure notifications to be sent)

"Checksum length" refers to the length of bytes which can be added together, mod 255, to produce the next byte. That is,
a checksum length of 4 implies that bytes (pseudocode) `Sum(bytes [0, 1, 2, 3]) % 255 == bytes[4]`. This is only
calculated for notifies, as commands always follow the TLVC structure.

## Basic data flow

App connects to the thermometer, receives temperature indications. No changes made in app.

| R/W/N  | Handle | Data                                          | Checksum length |
|--------|--------|-----------------------------------------------|-----------------|
| Write  | 0x14   | `0100` (0x01) - setup notifications from 0x13 |                 |
| Write  | 0x11   | `01097032e2c1799db4d1c7b1`                    |                 |
| Notify | 0x13   | `01010a0ce2c1799db4d1c7b10020c1799db4d1c7`    | 3               |
| Write  | 0x11   | `260026`                                      |                 |
| Notify | 0x13   | `26050c0c5a030faf0000071a0020480000200200`    | 7               |
| Write  | 0x11   | `23060100ffffffff26`                          |                 |
| Notify | 0x13   | `2302010026ffffff260000450200384c0200ffff`    | 4               |
| Write  | 0x11   | `23060200ffffffff27`                          |                 |
| Notify | 0x13   | `2302020027ffffff270000190020480000200200`    | 4               |
| Write  | 0x11   | `23060300ffffffff28`                          |                 |
| Notify | 0x13   | `2302030028ffffff280000190020480000200200`    | 4               |
| Write  | 0x11   | `23060400ffffffff29`                          |                 |
| Notify | 0x13   | `2302040029ffffff290000190020480000200200`    | 4               |
| Write  | 0x11   | `24010126`                                    |                 | 
| Notify | 0x13   | `24060100ffffffff2700ec190020480000200200`    | 8               |
| Write  | 0x11   | `24010227`                                    |                 |
| Notify | 0x13   | `24060200ffffffff2800001a0020480000200200`    | 8               |
| Write  | 0x11   | `24010328`                                    |                 |
| Notify | 0x13   | `24060300ffffffff2900141a0020480000200200`    | 8               |
| Write  | 0x11   | `24010429`                                    |                 | 
| Notify | 0x13   | `24060400ffffffff2a00281a0020480000200200`    | 8               |
| Write  | 0x11   | `2401052a`                                    |                 |
| Notify | 0x13   | `24060500ffffffff2b00a1190020480000200200`    | 8               |
| Write  | 0x11   | `2401062b`                                    |                 |
| Notify | 0x13   | `24060600ffffffff2c00b5190020480000200200`    | 8               |
| Write  | 0x11   | `410041`                                      |                 |
|        |        | Bad CRC - miscaptured notify?                 |                 |
| Write  | 0x11   | `250025`                                      |                 |
| Notify | 0x13   | `250e0600ffffffffffff0223ffffffff54200200`    | 16              |
|        |        | No initial 0x30 command here?                 |                 |
| Notify | 0x13   | `300f5a0c00ffffffffffff0222ffffffffbf0140`    | 17              |
| Write  | 0x11   | `300030`                                      |                 |
| Notify | 0x13   | `300f5a0c00ffffffffffff0233ffffffffd00140`    | 17              |
| Write  | 0x11   | `300030`                                      |                 |
|        |        | A connection params update happened here      |                 |
| Notify | 0x13   | `300f5a0c00ffffffffffff0258fffffffff50140`    | 17              |
| Write  | 0x11   | `300030`                                      |                 |
| Notify | 0x13   | `300f5a0c00ffffffffffff0281ffffffff1e0140`    | 17              |
| Write  | 0x11   | `300030`                                      |                 |
| Notify | 0x13   | `300f5a0c00ffffffffffff0297ffffffff340140`    | 17              |
| Write  | 0x11   | `300030`                                      |                 |
| Notify | 0x13   | `300f5a0c00ffffffffffff0310ffffffffae0140`    | 17              |
| Write  | 0x11   | `300030`                                      |                 |
| Notify | 0x13   | `300f5a0c00ffffffffffff0325ffffffffc30140`    | 17              |
| Write  | 0x11   | `300030`                                      |                 |
| Notify | 0x13   | `300f5a0c00ffffffffffff0309ffffffffa70140`    | 17              |
| Write  | 0x11   | `300030`                                      |                 |
| Notify | 0x13   | `300f5a0c00ffffffffffff0293ffffffff300140`    | 17              |
| Write  | 0x11   | `300030`                                      |                 | 
| Notify | 0x13   | `300f5a0c00ffffffffffff0281ffffffff1e0140`    | 17              |

## Setting a temperature profile

App connects to the thermometer. Temperature profile is selected, then changed, then set back to "no profile".

I've omitted the handles because they appear to be constant.

* This is the first capture where 0x30 responses come before the command - although this matches the fake app's
  observations
* 0x23 command/response seem to set the temperature profile.

| W/N?   | Data                                       | Checksum Length |
|--------|--------------------------------------------|-----------------|
| Write  | `0109b03fc1e879ee9d6d778a`                 |                 |
| Notify | `01010a0cc1e879ee9d6d778a0020e879ee9d6d77` | 3               |
| Write  | `260026`                                   |                 |
| Notify | `26050c0c5a030faf0000f3190020480000200200` | 7               |
| Write  | `23060100ffffffff26`                       |                 |
| Notify | `2302010026ffffff260000e000e0f1ffffff384c` | 4               |
| Write  | `23060200ffffffff27`                       |                 |
| Notify | `2302020027ffffff2700001a0020480000200200` | 4               |
| Write  | `23060300ffffffff28`                       |                 |
| Notify | `2302030028ffffff280000e000e0f1ffffff0000` | 4               |
| Write  | `23060400ffffffff29`                       |                 |
| Notify | `2302040029ffffff290000190020480000200200` | 4               |
| Write  | `24010126`                                 |                 |
| Notify | `24060100ffffffff2700d3190020480000200200` | 8               |
| Write  | `24010227`                                 |                 |
| Notify | `24060200ffffffff2800e7190020480000200200` | 8               |
| Write  | `24010328`                                 |                 |
| Notify | `24060300ffffffff2900fb190020480000200200` | 8               |
| Write  | `24010429`                                 |                 |
| Notify | `24060400ffffffff2a000f1a0020480000200200` | 8               |
| Write  | `2401052a`                                 |                 |
| Notify | `24060500ffffffff2b00231a0020480000200200` | 8               |
| Notify | `300f5a0c00ffffffffffff0185ffffffff210140` | 17              |
| Write  | `2401062b`                                 |                 |
| Notify | `24060600ffffffff2c00a1190020480000200200` | 8               |
| Write  | `300030`                                   |                 |
| Write  | `410041`                                   |                 |
| Notify | `410231118500917d0000c8190020480000200200` | 4               |
| Write  | `250025`                                   |                 |
| Notify | `250e0600ffffffffffff0185ffffffffb5200200` | 16              |
|        | Connection parameters update               |                 |
| Notify | `300f5a0c00ffffffffffff0186ffffffff220140` | 17              |
| Write  | `300030`                                   |                 |
| Write  | `230604050740000079` (Lamb 74C)            |                 |
|        | Miscaptured packets                        |                 |
| Write  | `23060406071000004a` (Pork 71C)            |                 |
| Notify | `230204062f1000004a0003190020480000200200` | 4               |
| Write  | `23060400ffffffff29` (No profile)          |                 |
|        | Took 5 attempts before write ack'd         |                 |
| Notify | `2302040029ffffff290003190020480000200200` | 4               |

## Setting a range profile and triggering both alarms

App connects to the thermometer. Range temperature profile named OAT is selected, with range 25-35C. Temperature raised
a little. Range changed to 25-29C. Temperature varied to allow alarm to be triggered at high and low temperature.

I forgot to note whether I cancelled the alarm in the app or on the device :roll_eyes:

> The repeated write commands are retransmissions (confirmed by looking at the relevant flag in Wireshark)

| W/N?   | Data                                       |
|--------|--------------------------------------------|
| Write  | `01094c40d13cee3a246d2985`                 |
| Notify | `01010a0cd13cee3a246d298500203cee3a246d29` |
| Write  | `260026`                                   |
| Notify | `26050c0c5a030faf0000071a0020480000200200` |
| Write  | `23060100ffffffff26`                       |
| Notify | `2302010026ffffff260000e000e0f1ffffff384c` |
| Write  | `23060200ffffffff27`                       |
| Notify | `2302020027ffffff270000190020480000200200` |
| Write  | `23060300ffffffff28`                       |
| Notify | `2302030028ffffff280000e000e0f1ffffff0000` |
| Write  | `23060400ffffffff29`                       |
| Notify | `2302040029ffffff290000190020480000200200` |
| Write  | `24010126`                                 |
| Notify | `24060100ffffffff2700ec190020480000200200` |
| Write  | `24010227`                                 |
| Notify | `24060200ffffffff2800001a0020480000200200` |
| Write  | `24010328`                                 |
| Notify | `24060300ffffffff2900141a0020480000200200` |
| Write  | `24010429`                                 |
| Notify | `24060400ffffffff2a00281a0020480000200200` |
| Write  | `2401052a`                                 |
| Notify | `24060500ffffffff2b00a1190020480000200200` |
| Write  | `2401062b`                                 |
| Notify | `24060600ffffffff2c00b5190020480000200200` |
| Write  | `410041`                                   |
| Notify | `410231118500917d0000c9190020480000200200` |
| Write  | `250025`                                   |
| Notify | `250e0600ffffffffffff0205ffffffff36200200` |
| Notify | `300f5a0c00ffffffffffff0225ffffffffc20140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c00ffffffffffff0241ffffffffde0140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c00ffffffffffff0245ffffffffe20140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c00ffffffffffff0241ffffffffde0140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c00ffffffffffff0238ffffffffd50140` |
| Write  | `300030`                                   |
| Write  | `230604fa03500250cc`                       |
| Write  | `230604fa03500250cc`                       |
| Write  | `230604fa03500250cc`                       |
| Write  | `230604fa03500250cc`                       |
| Write  | `230604fa03500250cc`                       |
| Notify | `230204fa23500250cc0003190020480000200200` |
| Notify | `300f5a0c00ffffffffffff0236ffffffffd30140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c00ffffffffffff0249ffffffffe60140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c00ffffffffffff0260fffffffffd0140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c00ffffffffffff0277ffffffff140140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c00ffffffffffff0290ffffffff2d0140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c00ffffffffffff0303ffffffffa10140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c00ffffffffffff0313ffffffffb10140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c00ffffffffffff0295ffffffff320140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c00ffffffffffff0285ffffffff220140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c00ffffffffffff0270ffffffff0d0140` |
| Write  | `300030`                                   |
| Write  | `230604fa029002500b`                       |
| Write  | `230604fa029002500b`                       |
| Write  | `230604fa029002500b`                       |
| Write  | `230604fa029002500b`                       |
| Notify | `300f5a0c00ffffffffffff0264ffffffff010140` |
| Notify | `230204fa239002500b00031a0020480000200200` |
| Write  | `300030`                                   |
| Notify | `300f5a0c00ffffffffffff0281ffffffff1e0140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c08ffffffffffff0294ffffffff390140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c08ffffffffffff0305ffffffffab0140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c08ffffffffffff0315ffffffffbb0140` |
| Write  | `270027`                                   |
| Write  | `300030`                                   |
| Notify | `270027000000917d0000da190020480000200200` |
| Notify | `300f5a0c00ffffffffffff0292ffffffff2f0140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c00ffffffffffff0282ffffffff1f0140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c00ffffffffffff0268ffffffff050140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c00ffffffffffff0256fffffffff30140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c08ffffffffffff0249ffffffffee0140` |
| Write  | `300030`                                   |
| Write  | `270027`                                   |
| Notify | `270027000000917d0000c7190020480000200200` |
| Notify | `300f5a0c00ffffffffffff0249ffffffffe60140` |
| Write  | `300030`                                   |
| Notify | `300f5a0c00ffffffffffff0241ffffffffde0140` |
| Write  | `300030`                                   |
| Write  | `230604fa029002500b`                       |
| Notify | `230204fa239002500b00001a0020480000200200` |
| Notify | `300f5a0c00ffffffffffff0235ffffffffd20140` |
| Write  | `300030`                                   |

