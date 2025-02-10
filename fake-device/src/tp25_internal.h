#pragma once

#include "tp25.h"

RawNotification getSetupResponse();
RawNotification getSetProfileResponse(uint8_t probe_index, uint8_t unknown);
RawNotification getReportProfileResponse(uint8_t probe_index, uint8_t unknown, uint16_t high_temp_alarm_bcd,
                                         uint16_t low_temp_alarm_bcd);
RawNotification getTwoSixResponse();
RawNotification getTempReportResponse(const Probe *probes);
RawNotification getFourOneResponse();
