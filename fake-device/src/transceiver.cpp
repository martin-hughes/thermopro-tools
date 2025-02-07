/** @file
 *  @brief TP25 BLE transceiver functions
 */

#include <zephyr/types.h>
#include <cstddef>
#include <cstring>
#include <cerrno>
#include <zephyr/sys/printk.h>
#include <zephyr/sys/byteorder.h>
#include <zephyr/kernel.h>
#include <zephyr/logging/log.h>
#include <zephyr/bluetooth/bluetooth.h>
#include <zephyr/bluetooth/hci.h>
#include <zephyr/bluetooth/conn.h>
#include <zephyr/bluetooth/uuid.h>
#include <zephyr/bluetooth/gatt.h>

#include "transceiver.h"

LOG_MODULE_DECLARE(TP25_Transceiver);

namespace {
  transceiver_callbacks callbacks;
  bool notify_response_enabled;
}

/* UUID declarations */
/* 1086fff0-3343-4817-8bb2-b32206336ce8 */

/** @brief ThermoPro service, read-write characteristic for commands */
#define TP25_RW_CHARACTERISTIC_VAL \
  BT_UUID_128_ENCODE(0x1086fff1, 0x3343, 0x4817, 0x8bb2, 0xb32206336ce8)
/** @brief ThermoPro service, read-notify characteristic for responses */
#define TP25_NOTIFY_CHARACTERISTIC_VAL \
  BT_UUID_128_ENCODE(0x1086fff2, 0x3343, 0x4817, 0x8bb2, 0xb32206336ce8)

/** @brief ThermoPro service, read-write characteristic for commands */
#define TP25_RW_CHARACTERISTIC \
  BT_UUID_DECLARE_128(TP25_RW_CHARACTERISTIC_VAL)
/** @brief ThermoPro service, read-notify characteristic for responses */
#define TP25_NOTIFY_CHARACTERISTIC \
  BT_UUID_DECLARE_128(TP25_NOTIFY_CHARACTERISTIC_VAL)

/* Forward declarations */
static ssize_t read_command(struct bt_conn *conn, const struct bt_gatt_attr *attr, void *buf, uint16_t len,
                            uint16_t offset);

static ssize_t write_command(struct bt_conn *conn, const struct bt_gatt_attr *attr, const void *buf, uint16_t len,
                             uint16_t offset, uint8_t flags);

static ssize_t read_response(struct bt_conn *conn, const struct bt_gatt_attr *attr, void *buf, uint16_t len,
                             uint16_t offset);

static void response_ccc_cfg_changed(const struct bt_gatt_attr *attr, uint16_t value);

/* Data storage */
uint8_t command_buffer[20] = {};
uint8_t response_buffer[20] = {};

/* LED Button Service Declaration */
BT_GATT_SERVICE_DEFINE(
  tp25_svc, BT_GATT_PRIMARY_SERVICE(TP25_SVC_UUID),

  BT_GATT_CHARACTERISTIC(TP25_RW_CHARACTERISTIC, BT_GATT_CHRC_READ | BT_GATT_CHRC_WRITE,
    BT_GATT_PERM_READ | BT_GATT_PERM_WRITE, read_command, write_command, command_buffer),

  BT_GATT_CHARACTERISTIC(TP25_NOTIFY_CHARACTERISTIC, BT_GATT_CHRC_READ | BT_GATT_CHRC_NOTIFY, BT_GATT_PERM_READ,
    read_response, nullptr, response_buffer),

  BT_GATT_CCC(response_ccc_cfg_changed, BT_GATT_PERM_READ | BT_GATT_PERM_WRITE),
);

/* Command handlers */
static ssize_t read_command(bt_conn *conn, const bt_gatt_attr *attr, void *buf, const uint16_t len,
                            const uint16_t offset) {
  const auto value = attr->user_data;
  LOG_DBG("Command attribute read, handle: %u, conn: %p", attr->handle, (void *)conn);
  if (callbacks.read_command) {
    callbacks.read_command(static_cast<uint8_t *>(buf) + offset, len);
  }
  return bt_gatt_attr_read(conn, attr, buf, len, offset, value, len);
}

static ssize_t write_command(bt_conn *conn, const bt_gatt_attr *attr, const void *buf, const uint16_t len,
                             const uint16_t offset, uint8_t flags) {
  LOG_DBG("Command attribute write, handle: %u, conn: %p", attr->handle, (void *)conn);

  if (len < 3) {
    LOG_DBG("Write command: command too short.");
    return BT_GATT_ERR(BT_ATT_ERR_INVALID_ATTRIBUTE_LEN);
  }

  if (len > 20) {
    LOG_DBG("Write command: command too long.");
    return BT_GATT_ERR(BT_ATT_ERR_INVALID_ATTRIBUTE_LEN);
  }

  if (offset != 0) {
    LOG_DBG("Write led: Incorrect data offset");
    return BT_GATT_ERR(BT_ATT_ERR_INVALID_OFFSET);
  }

  memcpy(command_buffer, buf, len);

  if (callbacks.write_command) {
    callbacks.write_command(command_buffer, len);
  }

  return len;
}

/* Response handlers */
static ssize_t read_response(bt_conn *conn, const bt_gatt_attr *attr, void *buf, const uint16_t len,
                             const uint16_t offset) {
  const auto value = attr->user_data;
  LOG_DBG("Response attribute read, handle: %u, conn: %p", attr->handle, (void *)conn);
  if (callbacks.read_response) {
    callbacks.read_response(static_cast<uint8_t *>(buf) + offset, len);
  }
  return bt_gatt_attr_read(conn, attr, buf, len, offset, value, len);
}

int notify_response(uint8_t const *const buf, const uint8_t len) {
  if (len > 20) {
    LOG_DBG("Notify response: response too long.");
    return -EINVAL;
  }

  memcpy(response_buffer, buf, len);

  if (!notify_response_enabled) {
    return -EACCES;
  }

  // TODO: Remove the magic constant and make sure it always points at the notify chc
  return bt_gatt_notify(nullptr, &tp25_svc.attrs[4], buf, len);
}


static void response_ccc_cfg_changed(const struct bt_gatt_attr *attr, uint16_t value) {
  notify_response_enabled = (value == BT_GATT_CCC_NOTIFY);
}

/* A function to register application callbacks for the LED and Button characteristics  */
bool transceiver_init(const transceiver_callbacks &callbacks) {
  ::callbacks = callbacks;
  return true;
}
