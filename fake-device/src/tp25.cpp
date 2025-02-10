#include <zephyr/kernel.h>

#include "tp25.h"

#include <stdexcept>

#include "bcd.h"
#include "tp25_internal.h"

TP25::TP25(const NotificationCb notification_cb) : notification_cb(notification_cb) {
}

void TP25::receive_command(const uint8_t *buffer, uint8_t length) {
  switch (static_cast<Command>(buffer[0])) {
    case Command::SETUP:
      printk("Setup command\n");
    // Normally command handlers send their own response
      this->notification_cb(getSetupResponse());
      break;

    case Command::SET_PROFILE:
      printk("Set profile command\n");
      this->set_profile(buffer, length);
      break;

    case Command::REPORT_PROFILE:
      printk("Report profile command\n");
      this->report_profile(buffer, length);
      break;

    case Command::ALT_TEMP_REPORT:
      printk("Alt temp report command - drop\n");
      break;

    case Command::UNKNOWN_A:
      printk("Unknown command (A - 26) \n");
      this->notification_cb(getTwoSixResponse());
      break;

    case Command::ALARM_ACK:
      printk("Alarm ack command - drop\n");
      break;

    case Command::TEMP_REPORT:
      printk("Temp report command - do nothing\n");
      //this->notification_cb(getTempReportResponse(this->probes));
      break;

    case Command::UNKNOWN_B:
      printk("Unknown command (B - 41)\n");
      this->notification_cb(getFourOneResponse());
      break;

    case Command::ERROR:
      printk("Error command - drop\n");
      break;

    default:
      printk("Unknown command\n");
  }
}

void TP25::receive_timer() {
  printk("Received timer\n");
  this->notification_cb(getTempReportResponse(this->probes));
}


void TP25::set_profile(uint8_t const *const buffer, const uint8_t length) {
  if (buffer[0] != static_cast<uint8_t>(Command::SET_PROFILE) || buffer[1] != 6 || length < 8) {
    printk("Invalid set profile command - sizes\n");
    return;
  }

  const uint8_t probe = buffer[2];
  if (probe == 0 || probe > 6) {
    printk("Invalid set profile command - index\n");
    return;
  }

  const uint8_t unknown = buffer[3];
  const uint16_t high_temp_alarm = buffer[4] << 8 | buffer[5];
  const uint16_t low_temp_alarm = buffer[6] << 8 | buffer[7];

  Probe &probe_ref = probes[probe];
  probe_ref.alarm_index = unknown;

  if (isBcd(high_temp_alarm)) {
    probe_ref.high_alarm_value = high_temp_alarm;
    probe_ref.low_alarm_value = isBcd(low_temp_alarm) ? low_temp_alarm : 0x0000;
  } else {
    probe_ref.high_alarm_value = isBcd(high_temp_alarm) ? high_temp_alarm : 0xffff;
    probe_ref.low_alarm_value = isBcd(low_temp_alarm) ? low_temp_alarm : 0xffff;
  }

  this->notification_cb(getSetProfileResponse(probe, unknown));
}

void TP25::report_profile(const uint8_t *const buffer, const uint8_t length) const {
  if (buffer[0] != static_cast<uint8_t>(Command::REPORT_PROFILE) || buffer[1] != 1 || length < 4) {
    return;
  }

  const uint8_t probe = buffer[2];
  if (probe == 0 || probe > 6) {
    return;
  }

  const Probe &probe_ref = probes[probe];
  this->notification_cb(getReportProfileResponse(probe, probe_ref.alarm_index, probe_ref.high_alarm_value,
                                                 probe_ref.low_alarm_value));
}
