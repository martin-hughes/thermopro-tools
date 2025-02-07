#pragma once

#include <cstdint>

/** @brief ThermoPro service UUID */
#define TP25_SVC_UUID_VAL \
BT_UUID_128_ENCODE(0x1086fff0, 0x3343, 0x4817, 0x8bb2, 0xb32206336ce8)

/** @brief ThermoPro service UUID */
#define TP25_SVC_UUID \
BT_UUID_DECLARE_128(TP25_SVC_UUID_VAL)

using read_command_callback = void (*)(uint8_t *buffer, uint8_t length);
using read_response_callback = void (*)(uint8_t *buffer, uint8_t length);
using write_command_callback = void (*)(uint8_t *buffer, uint8_t length);

struct transceiver_callbacks {
  read_command_callback read_command;
  write_command_callback write_command;
  read_response_callback read_response;
};

bool transceiver_init(const transceiver_callbacks &callbacks);

int notify_response(uint8_t const *const buf, const uint8_t len);
