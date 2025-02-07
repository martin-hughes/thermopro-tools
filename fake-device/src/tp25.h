#pragma once

#include <cstdint>

struct RawNotification {
  uint8_t value[20];
};

using NotificationCb = void (*)(RawNotification);

struct Probe {
  /// Temperature in tenths of a degree
  uint16_t temp = 200;

  /// Alarm profile index
  uint8_t alarm_index = 0x00;

  /// Low alarm temp in tenths of degree
  uint16_t low_alarm_value = 0xffff;

  /// High alarm temp in tenths of degree
  uint16_t high_alarm_value = 0xffff;
};

enum class Command : uint8_t {
  SETUP = 0x01,
  SET_PROFILE = 0x23,
  REPORT_PROFILE = 0x24,
  ALT_TEMP_REPORT = 0x25,
  UNKNOWN_A = 0x26,
  ALARM_ACK = 0x27,
  TEMP_REPORT = 0x30,
  UNKNOWN_B = 0x41,
  ERROR = 0xe0,
};

class TP25 {
public:
  explicit TP25(NotificationCb notification_cb);

  ~TP25() = default;

  void receive_command(const uint8_t *buffer, uint8_t length);

  void set_temperature(uint8_t probe_index, uint16_t temperature);

private:
  const NotificationCb notification_cb;

  Probe probes[4];
};
