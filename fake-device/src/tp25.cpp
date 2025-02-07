#include <zephyr/kernel.h>

#include "tp25.h"
#include "tp25_internal.h"

TP25::TP25(const NotificationCb notification_cb) : notification_cb(notification_cb) {
}

void TP25::receive_command(const uint8_t *buffer, uint8_t length) {
  switch (static_cast<Command>(buffer[0])) {
    case Command::SETUP: {
      printk("Setup command\n");
      this->notification_cb(getSetupResponse());
    }
    break;

    case Command::SET_PROFILE:
      printk("Set profile command - drop\n");
      break;

    case Command::REPORT_PROFILE:
      printk("Report profile command - drop\n");
      break;

    case Command::ALT_TEMP_REPORT:
      printk("Alt temp report command - drop\n");
      break;

    case Command::UNKNOWN_A:
      printk("Unknown command (A) - drop\n");
      break;

    case Command::ALARM_ACK:
      printk("Alarm ack command - drop\n");
      break;

    case Command::TEMP_REPORT: {
      printk("Temp report command\n");
      this->notification_cb(getTempReportResponse(this->probes));
    }
    break;

    case Command::UNKNOWN_B:
      printk("Unknown command (B) - drop\n");
      break;

    case Command::ERROR:
      printk("Error command - drop\n");
      break;

    default:
      printk("Unknown command\n");
  }
}
