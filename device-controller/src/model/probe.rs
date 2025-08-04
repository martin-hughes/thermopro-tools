#[derive(Clone, Copy, Debug)]
pub struct UpperLimitThreshold {
    pub max: u16,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum AlarmState {
    #[default]
    Unknown,
    NoAlarm,
    Alarm,
}

#[derive(Clone, Copy, Debug)]
pub struct RangeLimitThreshold {
    pub min: u16,
    pub max: u16,
}

#[derive(Clone, Copy, Debug)]
pub enum AlarmThreshold {
    NoneSet,
    UpperLimit(UpperLimitThreshold),
    RangeLimit(RangeLimitThreshold),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Probe {
    pub temperature: Option<u16>,
    pub alarm: AlarmState,
    pub alarm_threshold: Option<AlarmThreshold>,
}

#[derive(Clone, Copy, Debug)]
pub enum ProbeIdx {
    Probe1 = 1,
    Probe2 = 2,
    Probe3 = 3,
    Probe4 = 4,
}

impl ProbeIdx {
    // This strange looking use of `internal_` functions is because I can't figure out the correct
    // syntax to do the conversion directly in `as_(zero|one)_based` without making the parameter
    // a moved self rather than a borrowed self.
    //
    // Although maybe it's more rust-y to be a moved self...?
    fn internal_as_zero_based(idx: Self) -> u8 {
        Self::internal_as_one_based(idx) - 1
    }

    fn internal_as_one_based(idx: Self) -> u8 {
        idx as u8
    }

    pub fn as_zero_based(&self) -> u8 {
        Self::internal_as_zero_based(*self)
    }

    pub fn as_one_based(&self) -> u8 {
        Self::internal_as_one_based(*self)
    }

    pub fn try_from_zero_based(idx: u8) -> Result<Self, ()> {
        match idx {
            0 => Ok(ProbeIdx::Probe1),
            1 => Ok(ProbeIdx::Probe2),
            2 => Ok(ProbeIdx::Probe3),
            3 => Ok(ProbeIdx::Probe4),
            _ => Err(()),
        }
    }

    pub fn from_zero_based(idx: u8) -> Self {
        Self::try_from_zero_based(idx).unwrap()
    }

    pub fn try_from_one_based(idx: u8) -> Result<Self, ()> {
        match idx {
            1 => Ok(ProbeIdx::Probe1),
            2 => Ok(ProbeIdx::Probe2),
            3 => Ok(ProbeIdx::Probe3),
            4 => Ok(ProbeIdx::Probe4),
            _ => Err(()),
        }
    }

    pub fn from_one_based(idx: u8) -> Self {
        Self::try_from_one_based(idx).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn test_from_zero_based() {
        assert_matches!(ProbeIdx::from_zero_based(0), ProbeIdx::Probe1);
        assert_matches!(ProbeIdx::from_zero_based(1), ProbeIdx::Probe2);
        assert_matches!(ProbeIdx::from_zero_based(2), ProbeIdx::Probe3);
        assert_matches!(ProbeIdx::from_zero_based(3), ProbeIdx::Probe4);
        assert_matches!(ProbeIdx::try_from_zero_based(4), Err(_));
    }

    #[test]
    fn test_from_one_based() {
        assert_matches!(ProbeIdx::from_one_based(1), ProbeIdx::Probe1);
        assert_matches!(ProbeIdx::from_one_based(2), ProbeIdx::Probe2);
        assert_matches!(ProbeIdx::from_one_based(3), ProbeIdx::Probe3);
        assert_matches!(ProbeIdx::from_one_based(4), ProbeIdx::Probe4);

        assert_matches!(ProbeIdx::try_from_one_based(0), Err(_));
        assert_matches!(ProbeIdx::try_from_one_based(5), Err(_));
    }

    #[test]
    fn test_as_zero_based() {
        assert_eq!(ProbeIdx::Probe1.as_zero_based(), 0);
        assert_eq!(ProbeIdx::Probe2.as_zero_based(), 1);
        assert_eq!(ProbeIdx::Probe3.as_zero_based(), 2);
        assert_eq!(ProbeIdx::Probe4.as_zero_based(), 3);
    }

    #[test]
    fn test_as_one_based() {
        assert_eq!(ProbeIdx::Probe1.as_one_based(), 1);
        assert_eq!(ProbeIdx::Probe2.as_one_based(), 2);
        assert_eq!(ProbeIdx::Probe3.as_one_based(), 3);
        assert_eq!(ProbeIdx::Probe4.as_one_based(), 4);
    }
}
