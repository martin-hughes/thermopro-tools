#include <cstring>
#include "tp25_internal.h"

constexpr auto notification = RawNotification{
  {
    0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x17, 0x18, 0x19,
    0x20
  }
};

namespace {
  constexpr uint8_t getChecksum(const RawNotification &notification, const uint8_t length) {
    uint8_t checksum = 0;
    for (uint8_t i = 0; i < length; i++) {
      checksum = checksum + notification.value[i];
    }
    return checksum;
  }

  constexpr uint16_t decToBcd(const uint16_t value) {
    const uint16_t tenths = value % 10;
    const uint16_t ones = (value / 10) % 10;
    const uint16_t tens = (value / 100) % 10;
    const uint16_t hundreds = (value / 1000) % 10;

    return (hundreds << 12) + (tens << 8) + (ones << 4) + tenths;
  }
}

RawNotification getSetupResponse() {
  RawNotification full_response{notification};
  constexpr uint8_t part_response[] = {0x01, 0x01, 0x0a};
  constexpr uint8_t part_length = sizeof(part_response);
  std::memcpy(&full_response, part_response, part_length);
  full_response.value[part_length] = getChecksum(full_response, part_length);

  return full_response;
}

RawNotification getTempReportResponse(const Probe *const probes) {
  RawNotification full_response{notification};
  full_response.value[0] = 0x30; // Code
  full_response.value[1] = 0x0f; // Length
  full_response.value[2] = 0x5a; // Unknown
  full_response.value[3] = 0x0c; // unknown
  full_response.value[4] = 0x00; // Alarms (not implemented)

  for (uint8_t i = 0; i < 4; i++) {
    const uint16_t bcd = decToBcd(probes[i].temp);
    full_response.value[5 + (i * 2)] = bcd >> 8;
    full_response.value[5 + (i * 2) + 1] = static_cast<uint8_t>(bcd);
  }
  full_response.value[13] = 0xff;
  full_response.value[14] = 0xff;
  full_response.value[15] = 0xff;
  full_response.value[16] = 0xff;
  full_response.value[17] = getChecksum(full_response, 17);

  return full_response;
}
