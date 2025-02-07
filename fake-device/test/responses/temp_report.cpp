#include "../../src/tp25.h"
#include "../../src/tp25_internal.h"
#include "gtest/gtest.h"

#include <cstring>

constexpr Probe probes[4] = {
  {.temp = 4321},
  {.temp = 1234},
  {.temp = 123},
  {.temp = 3210}
};

TEST(TempReportTests, BasicTest) {
  const auto response = getTempReportResponse(probes);
  constexpr auto expected = RawNotification({
    .value = {
      0x30, 0x0f, 0x5a, 0x0c, 0x00, 0x43, 0x21, 0x12, 0x34, 0x01, 0x23, 0x32, 0x10, 0xff, 0xff, 0xff, 0xff, 0xb1, 0x19,
      0x20
    }
  });
  EXPECT_EQ(memcmp(&response, &expected, sizeof(expected)), 0);
}
