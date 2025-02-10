#pragma once

#include <stdexcept>

constexpr uint16_t decToBcd(const uint16_t value) {
  const uint16_t tenths = value % 10;
  const uint16_t ones = (value / 10) % 10;
  const uint16_t tens = (value / 100) % 10;
  const uint16_t hundreds = (value / 1000) % 10;

  return (hundreds << 12) + (tens << 8) + (ones << 4) + tenths;
}

constexpr bool isDigit(const uint8_t val) {
  return val <= 9;
}

constexpr bool isBcd(const uint16_t value) {
  const auto tenths = value & 0xff;
  const auto ones = (value >> 8) & 0xff;
  const auto tens = (value >> 16) & 0xff;
  const auto hundreds = value >> 24;

  return isDigit(tenths) && isDigit(ones) && isDigit(tens) && isDigit(hundreds);
}

constexpr uint16_t bcdToDec(const uint16_t value) {
  if (!isBcd(value)) {
    return 0xffff;
  }

  const auto tenths = value & 0xff;
  const auto ones = (value >> 8) & 0xff;
  const auto tens = (value >> 16) & 0xff;
  const auto hundreds = value >> 24;

  return tenths + ones * 10 + tens * 100 + hundreds * 1000;
}
