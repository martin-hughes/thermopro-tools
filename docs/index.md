# ThermoPro TP25 BLE Protocol documentation

> This has been entirely reverse-engineered by observing a TP25 in action. There may be mistakes! There's also quite
> a bit that I haven't understood yet. Please feel free to submit corrections or additions via a PR.
>
> Expect this document to expand significantly.

## The basics

The TP25 communicates via Bluetooth Low Energy ('BLE'). It exposes two characteristics, one which is writeable and one
which produces notifications. By writing to the writeable one we can control which data comes out by the notifications.

## Terminology

I use BLE terminology liberally without explaining it here. If you're not familiar, I recommend the Nordic Semiconductor
[Bluetooth Low Energy Fundamentals course](https://academy.nordicsemi.com/courses/bluetooth-low-energy-fundamentals/).

* Packet: A specific *command* written to the writeable characteristic, or *value* read from the notifiable one.
  I guess these aren't exactly packets in the normal networking sense, but I think it gives us the right idea.
* Command: A value written to the writable characteristic. I describe this as a command because it appears to tell the
  thermometer what data to send back.
* Value: If used in the context of the notifiable characteristic, the data that is read from the characteristic, which
  has been sent by the thermometer in response to a command.
* Response: Another shorthand for the value sent by the thermometer.
* TLVC, TLVC format: Shorthand for "type, length, value, checksum", as detailed [below](#tlvc-format).

## Data flow

As far as I can tell, the thermometer will normally only send notifications in response to a command. So, broadly
speaking, the normal data flow is:

* Send command
* Wait for notification
* Read response
* Repeat from top

The first byte of each response is always the same as the first byte of the preceding command, probably to make it
easier to manage the state of the controlling app.

An example data flow can be seen in [this example](./examples/driven-by-app)

The exception is to the normal data flow is that the thermometer appears to periodically send temperature updates (0x30
response) without needing a 0x30 command first.

## Commands

All commands appear to be sent in [TLVC format](#tlvc-format).

The following commands are *partially* understood:

| Command | Example                    | Notes                                                                                                                                |
|---------|----------------------------|--------------------------------------------------------------------------------------------------------------------------------------|
| 0x01    | `01097032e2c1799db4d1c7b1` | Some kind of setup instruction. This must be sent or the device will not send 0x30 notifications                                     |
| 0x23    | `23060406071000004a`       | Set probe "profile" (which corresponds to alarm temperatures in the app)                                                             | 
| 0x30    | `300030`                   | Send temperature data. Although it's not clear why this command is needed, the thermometer will send 0x30 without any 0x30 commands. |

The following commands are not understood yet:

| Command | Example    | Notes                                                               |
|---------|------------|---------------------------------------------------------------------|
| 0x26    | `260026`   | Mode setting instruction? Has no data                               |
| 0x24    | `24010126` | Appears 6 times.                                                    |
| 0x41    | `410041`   | No data, so perhaps a mode setting command?                         |
| 0x25    | `250025`   | Another mode setting command? Gets back a temperature-like response |
| 0x27    | `270027`   | Seems to be sent when alarm is triggered, but not yet tested.       |

### Set probe profile command format 0x23

The 0x23 set probe profile command seems to be:

```c
struct {
  uint8_t command;          /* 0x23 */
  uint8_t length;           /* 6 */
  uint8_t probe_index;      /* 1-based */
  uint8_t unknown;          /* see below */
  uint16_t high_temp_alarm; /* Alarm at this temperature */
  uint16_t low_temp_alarm;       /* Most likely the low temperature alarm for ranges, but not yet known */
  uint8_t checksum;
```

* For `unknown` I've observed two things. 0x00 is followed by both the temperature fields being filled with 0xff. Other
  values are followed by sensible temperatures in the temperature fields.
* `high_temp_alarm` and `low_temp_alarm` follow the same BCD-ish format described
  in [Temperature data format](#temperature-data-0x30-format).
* `0x0000` in `low_temp_alarm` may mean "no low temperature set", or it may be that some flag in `unknown` distinguishes
  range temperature profiles from single temperature ones.
* If no "profile" (of alarms) is associated with a probe, both `high_temp_alarm` and `unknown_2` are filled with 0xffff.
  * and `unknown` is set to 0x00.

## Responses

All responses as received over the air appear to be 20 bytes long. Generally all are received after a command with the
same first byte is sent. The exceptions are:

* 0x30 temperature data notifications can be sent any time.
* 0xe0 seems to be sent if an invalid command has been sent.

Each response appears to have a valid checksum byte at the same position. Assuming the [TVC format](#tvc-format) below
is correct, the `length` field for each command is given in this table. I suspect that the part after the checksum is
junk, but I don't know for sure.

| Command | Example                                    | Data Length | Notes                                               |
|---------|--------------------------------------------|-------------|-----------------------------------------------------|
| 0x01    | `01010a0ce2c1799db4d1c7b10020c1799db4d1c7` | 2           | Not understood, but interestingly repetitive.       |
| 0x26    | `26050c0c5a030faf0000071a0020480000200200` | 6           | Not understood                                      |
| 0x23    | `2302010026ffffff260000450200384c0200ffff` | 3           | Set probe profile response. Not understood.         |
| 0x24    | `24060200ffffffff2800001a0020480000200200` | 7           | Not understood. Weird that it's sent 6 times        |
| 0x25    | `250e0600ffffffffffff0223ffffffff54200200` | 15          | Possibly a different format of temperature response |
| 0x27    | `270027000000917d0000da190020480000200200` | 1           | Acknowledgement of alarm being triggered?           |
| 0x30    | `300f5a0c00ffffffffffff0325ffffffffc30140` | 16          | Temperature data                                    |
| 0xe0    | `E00230041600917D0000E9190020480000200200` | 0?          | Error?                                              |

### Temperature data (0x30) format

The temperature data sent in an 0x30 response appears to be:

```c
struct {
  uint8_t command;          /* 0x30 */
  uint32_t preamble[4];     /* 0f5a0c00 - not understood */
  uint16_t temperatures[6];
  uint8_t checksum;
  uint16_t suffix;          /* 0140 - seems to be constant, but probably junk */
};
```

* `temperatures` are in BCD format as tenths of degrees Celsius - so `0325` in the trace means 32.5 degrees C.
* `checksum` is calculated the same way as for commands, for all preceding bytes (i.e. not including the suffix)

I do not know why there is space for 6 temperatures when it's a 4-probe thermometer.

## TLVC format

TLVC is shorthand for "type, length, value, checksum". It seems to only be used for commands. The format is (in a
pseudo-C):

```c
struct {
  uint8_t type;          /* The command being sent */
  uint8_t length;        /* The length of the data being sent, may be zero */
  uint8_t value[length]; /* omitted if length == 0 */
  uint8_t checksum;      /* described below */
}
```

`checksum` is the mod-256 addition of each other byte in the data structure. That is, something like:

```c
uint8_t checksum = 0;
uint8_t *ptr = (uint8_t *)(&command);
for (uint8_t i = 0; i < command->length + 2; i++, ptr++) {
  checksum += *ptr; /* Assuming your compiler wraps integers like most do */
}
```

## TVC Format

TVC is shorthand for "type, value, checksum". It appears to be in use for notifications. The format is (in a
pseudo-C):

```c
struct {
  uint8_t type;                 /* The command being sent */
  uint8_t value[length];        /* length is only known from the `type` value, it is not sent over the wire */
  uint8_t checksum;             /* described below */
  uint8_t unknown[18 - length]; /* rest of response appears to be garbage... */
}
```

Checksum has the same meaning as for commands - mod 255 addition.
