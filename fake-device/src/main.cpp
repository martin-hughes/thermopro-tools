#include <zephyr/kernel.h>
#include <zephyr/logging/log.h>
#include <zephyr/bluetooth/bluetooth.h>
#include <zephyr/bluetooth/gap.h>
#include <zephyr/bluetooth/uuid.h>
#include <zephyr/bluetooth/conn.h>
#include <dk_buttons_and_leds.h>

#include "transceiver.h"
#include "tp25.h"

LOG_MODULE_REGISTER(Fake_ThermoPro, LOG_LEVEL_INF);

namespace {
  //
  // Helper functions
  //
  void tp25_notify_request(RawNotification response);
  void tp25_command_write(const uint8_t *buffer, uint8_t length);

  void on_connected(bt_conn *conn, uint8_t err);
  void on_disconnected(bt_conn *conn, uint8_t reason);

  //
  // Application specific data.
  //
  const bt_le_adv_param *adv_param = BT_LE_ADV_PARAM(
    (BT_LE_ADV_OPT_CONNECTABLE |
      BT_LE_ADV_OPT_USE_IDENTITY), /* Connectable advertising and use identity address */
    800, /* Min Advertising Interval 500ms (800*0.625ms) */
    801, /* Max Advertising Interval 500.625ms (801*0.625ms) */
    NULL); /* Set to NULL for undirected advertising */

  const auto DEVICE_NAME = CONFIG_BT_DEVICE_NAME;
  constexpr auto DEVICE_NAME_LEN = sizeof(CONFIG_BT_DEVICE_NAME) - 1;

  constexpr auto RUN_STATUS_LED = DK_LED1;
  constexpr auto CON_STATUS_LED = DK_LED2;

  constexpr auto STACKSIZE = 4096;
  constexpr auto PRIORITY = 7;

  constexpr auto RUN_LED_BLINK_INTERVAL = 1000;
  constexpr auto NOTIFY_INTERVAL = 3000;

  const bt_data ad[] = {
    BT_DATA_BYTES(BT_DATA_FLAGS, (BT_LE_AD_GENERAL | BT_LE_AD_NO_BREDR)),
    BT_DATA(BT_DATA_NAME_COMPLETE, DEVICE_NAME, DEVICE_NAME_LEN),
  };

  const bt_data sd[] = {
    BT_DATA_BYTES(BT_DATA_UUID128_ALL, TP25_SVC_UUID_VAL),
  };

  transceiver_callbacks app_callbacks = {
    .read_command = [](uint8_t *_, const uint8_t _2) { LOG_INF("Received read command request"); },
    .write_command = [](uint8_t *buffer, const uint8_t length) {
      LOG_INF("Received command write request");
      tp25_command_write(buffer, length);
    },
    .read_response = [](uint8_t *_, const uint8_t _2) { LOG_INF("Received read response request"); },
  };

  bt_conn_cb connection_callbacks = {
    .connected = on_connected,
    .disconnected = on_disconnected,
  };

  TP25 *tp25;
}

extern "C"
[[noreturn]] void thermopro_thread() {
  while (true) {
    k_sleep(K_MSEC(NOTIFY_INTERVAL));
    if (tp25) { tp25->receive_timer(); }
  }
}

K_THREAD_DEFINE(thermopro_thread_id, STACKSIZE, thermopro_thread, NULL, NULL, NULL, PRIORITY, 0, 0);

extern "C"
int main() {
  int blink_status = 0;
  tp25 = new TP25{tp25_notify_request};

  LOG_INF("Starting Fake ThermoPro TP25\n");

  auto err = dk_leds_init();
  if (err) {
    LOG_ERR("LEDs init failed (err %d)\n", err);
    return -1;
  }

  err = bt_enable(nullptr);
  if (err) {
    LOG_ERR("Bluetooth init failed (err %d)\n", err);
    return -1;
  }
  bt_conn_cb_register(&connection_callbacks);

  if (!transceiver_init(app_callbacks)) {
    printk("Failed to init transceiver (err:%d)\n", err);
    return -1;
  }

  LOG_INF("Bluetooth initialized\n");

  err = bt_le_adv_start(adv_param, ad, ARRAY_SIZE(ad), sd, ARRAY_SIZE(sd));
  if (err) {
    LOG_ERR("Advertising failed to start (err %d)\n", err);
    return -1;
  }

  LOG_INF("Advertising successfully started\n");
  for (;;) {
    dk_set_led(RUN_STATUS_LED, (++blink_status) % 2);
    k_sleep(K_MSEC(RUN_LED_BLINK_INTERVAL));
  }
}

namespace {
  void on_connected(struct bt_conn *conn, uint8_t err) {
    if (err) {
      printk("Connection failed (err %u)\n", err);
      return;
    }

    printk("Connected\n");

    dk_set_led_on(CON_STATUS_LED);
  }

  void on_disconnected(bt_conn *conn, uint8_t reason) {
    printk("Disconnected (reason %u)\n", reason);

    dk_set_led_off(CON_STATUS_LED);
  }

  void tp25_notify_request(RawNotification response) {
    LOG_HEXDUMP_INF(response.value, 20, "Notification");
    notify_response(response.value, sizeof(response.value));
  }

  void tp25_command_write(const uint8_t *buffer, const uint8_t length) {
    LOG_HEXDUMP_INF(buffer, static_cast<uint32_t>(length > 20? 20: length), "Command");
    if (tp25) {
      tp25->receive_command(buffer, length);
    }
  }
}
