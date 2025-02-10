#include <cstring>
#include "tp25_internal.h"
#include "bcd.h"

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
}

RawNotification getSetupResponse() {
  RawNotification full_response{notification};
  constexpr uint8_t part_response[] = {0x01, 0x01, 0x0a};
  constexpr uint8_t part_length = sizeof(part_response);
  std::memcpy(&full_response, part_response, part_length);
  full_response.value[part_length] = getChecksum(full_response, part_length);

  return full_response;
}

RawNotification getSetProfileResponse(const uint8_t probe_index, const uint8_t unknown) {
  RawNotification full_response{notification};
  const uint8_t part_response[] = {0x023, 0x02, probe_index, unknown};
  constexpr uint8_t part_length = sizeof(part_response);
  std::memcpy(&full_response, part_response, part_length);
  full_response.value[part_length] = getChecksum(full_response, part_length);

  return full_response;
}

RawNotification getReportProfileResponse(const uint8_t probe_index, const uint8_t unknown,
                                         uint16_t const high_temp_alarm_bcd, const uint16_t low_temp_alarm_bcd) {
  RawNotification full_response{notification};
  const uint8_t part_response[] = {
    0x024, 0x06, probe_index, unknown, static_cast<uint8_t>((high_temp_alarm_bcd & 0xff00) >> 8),
    static_cast<uint8_t>(high_temp_alarm_bcd & 0xff), static_cast<uint8_t>((low_temp_alarm_bcd & 0xff00) >> 8),
    static_cast<uint8_t>(low_temp_alarm_bcd & 0xff)
  };
  constexpr uint8_t part_length = sizeof(part_response);
  std::memcpy(&full_response, part_response, part_length);
  full_response.value[part_length] = getChecksum(full_response, part_length);

  return full_response;
}

RawNotification getTwoSixResponse() {
  RawNotification full_response{notification};
  const uint8_t part_response[] = {0x26, 0x5, 0x0c, 0x0c, 0x5a, 0x03, 0x0f};
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

RawNotification getFourOneResponse() {
  RawNotification full_response{notification};
  const uint8_t part_response[] = {0x41, 0x02, 0x31, 0x11};
  constexpr uint8_t part_length = sizeof(part_response);
  std::memcpy(&full_response, part_response, part_length);
  full_response.value[part_length] = getChecksum(full_response, part_length);

  return full_response;
}
