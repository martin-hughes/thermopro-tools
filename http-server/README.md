# Server providing an HTTP interface to a ThermoPro TP25

This crate provides a simple server giving an HTTP interface to a ThermoPro TP25.

## Contents

## Prerequisites

The server requires access to a Bluetooth Low Energy interface. It expects to find an unpaired ThermoPro TP25 via that
interface.

## Basic usage

From the project root (the directory above this one) run:

```
cargo run -p http-server
```

This will connect to the first compatible ThermoPro thermometer it finds, and then provide the interface described
below.

If you only want to run a test mode without access to Bluetooth, use the `dummy_device` feature:

```
cargo run -p http-server --features dummy_device
```

## HTTP interface summary

Each HTTP `GET` or `POST` is executed and returns instantly, not necessarily waiting for the thermometer to receive the
command. There is no mechanism for sending a command to the thermometer and waiting for a response. For example, POSTing
to `/alarm` and then GETting from `/state` may still return a previous value for the alarm.

(This should sound familiar to anyone who has done multi-threading programming in the past.)

### Thermometer state in JSON format

This "JSON state object" represents the state of the thermometer, as currently understood by the app.

Format:

```json
{
  "connected": true,
  // one of "celsius", "fahrenheit", "unknown",
  "temp_mode": "celsius",
  "probes": [
    {
      // one of "alarm" (the temperature is in the alarm range), "no_alarm" (the opposite), "unknown" (the app doesn't 
      // know)
      "alarm": "no_alarm",
      // Temperature in tenths of a degrees celsius. Always in degrees C. Set to "unknown" if no probe connected.
      "temp": "27.3",
      "alarm_threshold": {
        // one of:
        // - "unknown" - The app does not know what alarm profile is set for this probe
        // - "none_set" - No alarm profile is set
        // - "upper_only" - The device will alarm when the temperature crosses "upper"
        // - "range" - The device will alarm when the temperature is outside of "lower" and "upper"
        "mode": "range",
        // Temperatures are in tenths of degrees Celsius (never Fahrenheit)
        // Not supplied if "mode" is "unknown" or "none_set"
        "upper": "30.0",
        // Only supplied if "mode" is "range"
        "lower": "25.5"
      }
    }
    // repeated for each probe
  ]
}
```

> If `connected` is set to `false`, then the response will not contain the other fields.

### GET `/state`

Returns a JSON state object containing the current state of the thermometer. It does not trigger any commands to be
sent to the thermometer, it relies entirely on the state this app has stored.

### GET `/ws`

This endpoint expects the user to upgrade to a websocket connection. Once upgraded, thermometer state updates will
trigger a message to be sent over the websocket. Each message is simply a JSON state object as described above.

There are no commands that can currently be sent to the thermometer using the websocket interface.

### POST `/mode`

Set either Celsius or Fahrenheit mode:

```
POST http://localhost:8080/mode
Accept: application/json
Content-Type: application/json

{"celsius": true}
```

Naturally, set `"celsius": false` for Fahrenheit mode.

### POST `/alarm`

Set a temperature threshold alarm on the device.

```
POST http://localhost:8080/alarm
Accept: application/json
Content-Type: application/json

{"probe_idx": 1, "alarm_high": "35.1", "alarm_low": "28.0"}
```

* `probe_idx` - mandatory. *Zero-based* index of the probe to set an alarm for. (Must be <= 3, as the TP25 has 4
  probes.)
  * This is zero-based to be consistent with the JSON state object, which uses a plain array (and arrays are zero-based)
* `alarm_high` - optional. The high temperature of the alarm to set, in celsius. If the temperature goes above this
  value, the alarm is triggered
* `alarm_low` - optional. The low temperature of the alarm to set, in celsius. If the temperature goes below this
  value, the alarm is triggered

If `alarm_low` is set, then `alarm_high` *must* be set. (`alarm_low` by itself is invalid.)

If neither alarm field is set, the alarm thresholds are cleared and no temperature will trigger an alarm on that probe.

### POST `/alarm_ack`

Acknowledge an alarm on the device - this causes the device to stop flashing and beeping.

```
POST http://localhost:8080/alarm_ack
Accept: application/json
Content-Type: application/json

{}
```

Simply sending empty JSON is sufficient.

### POST `/custom_cmd`

Send a hex-formatted sequence of bytes to the thermometer.

```
POST http://localhost:8080/custom_cmd
Accept: application/json
Content-Type: application/json

{"cmd": "330033", "allow_wrong_checksum": true}
```

* `cmd` - mandatory. A hex formatted string of bytes to be sent to the thermometer. Do not include the leading '0x'. Any
  string that is not valid hex will be rejected.
* `allow_wrong_checksum` - optional. If set to true, any string of bytes can be sent to the thermometer. If false, the
  checksum byte is checked according to the normal Thermopro rules. If it is wrong, the command is rejected and not
  sent.
