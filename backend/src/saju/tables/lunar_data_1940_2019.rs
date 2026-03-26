//! 양력↔음력 변환 테이블 (Solar-Lunar Calendar Conversion) 1940-2019
//!
//! This module provides lookup tables for converting between the Gregorian (solar)
//! calendar and the traditional Korean lunar calendar (음력) for years 1940-2019.
//!
//! Data source: korean_lunar_calendar Python library (based on KASI data).
//! Month lengths determined by solar date gap method for accuracy.

use super::lunar_calendar::LunarYearData;

/// Lunar calendar data for years 1940-2019.
///
/// month_lengths encoding:
///   Bit 0 (LSB) = month 1, Bit 1 = month 2, ..., Bit 11 = month 12
///   Bit 12 = leap month (inserted after the month indicated by leap_month)
///   1 = 30 days (대월), 0 = 29 days (소월)
pub static LUNAR_YEARS_1940_2019: [LunarYearData; 80] = [
    // 1940 (경진년) - no leap
    LunarYearData {
        year: 1940,
        new_year_solar: (2, 8),
        leap_month: 0,
        month_lengths: 0b0101_0010_1011,
        total_days: 354,
    },
    // 1941 (신사년) - leap 6월
    LunarYearData {
        year: 1941,
        new_year_solar: (1, 27),
        leap_month: 6,
        month_lengths: 0b1_0101_0001_1011,
        total_days: 384,
    },
    // 1942 (임오년) - no leap
    LunarYearData {
        year: 1942,
        new_year_solar: (2, 15),
        leap_month: 0,
        month_lengths: 0b1001_0110_1101,
        total_days: 355,
    },
    // 1943 (계미년) - no leap
    LunarYearData {
        year: 1943,
        new_year_solar: (2, 5),
        leap_month: 0,
        month_lengths: 0b1011_0110_1010,
        total_days: 355,
    },
    // 1944 (갑신년) - leap 4월
    LunarYearData {
        year: 1944,
        new_year_solar: (1, 26),
        leap_month: 4,
        month_lengths: 0b1_1101_1010_0100,
        total_days: 384,
    },
    // 1945 (을유년) - no leap
    LunarYearData {
        year: 1945,
        new_year_solar: (2, 13),
        leap_month: 0,
        month_lengths: 0b1011_1010_0100,
        total_days: 354,
    },
    // 1946 (병술년) - no leap
    LunarYearData {
        year: 1946,
        new_year_solar: (2, 2),
        leap_month: 0,
        month_lengths: 0b1011_0100_1001,
        total_days: 354,
    },
    // 1947 (정해년) - leap 2월
    LunarYearData {
        year: 1947,
        new_year_solar: (1, 22),
        leap_month: 2,
        month_lengths: 0b0_1101_0100_1011,
        total_days: 384,
    },
    // 1948 (무자년) - no leap
    LunarYearData {
        year: 1948,
        new_year_solar: (2, 10),
        leap_month: 0,
        month_lengths: 0b1010_1001_0101,
        total_days: 354,
    },
    // 1949 (기축년) - leap 7월
    LunarYearData {
        year: 1949,
        new_year_solar: (1, 29),
        leap_month: 7,
        month_lengths: 0b0_1010_1010_1011,
        total_days: 384,
    },
    // 1950 (경인년) - no leap
    LunarYearData {
        year: 1950,
        new_year_solar: (2, 17),
        leap_month: 0,
        month_lengths: 0b0101_0010_1101,
        total_days: 354,
    },
    // 1951 (신묘년) - no leap
    LunarYearData {
        year: 1951,
        new_year_solar: (2, 6),
        leap_month: 0,
        month_lengths: 0b1010_1010_1101,
        total_days: 355,
    },
    // 1952 (임진년) - leap 5월
    LunarYearData {
        year: 1952,
        new_year_solar: (1, 27),
        leap_month: 5,
        month_lengths: 0b1_1010_1010_1010,
        total_days: 384,
    },
    // 1953 (계사년) - no leap
    LunarYearData {
        year: 1953,
        new_year_solar: (2, 14),
        leap_month: 0,
        month_lengths: 0b1101_1011_0010,
        total_days: 355,
    },
    // 1954 (갑오년) - no leap
    LunarYearData {
        year: 1954,
        new_year_solar: (2, 4),
        leap_month: 0,
        month_lengths: 0b1101_1010_0100,
        total_days: 354,
    },
    // 1955 (을미년) - leap 3월
    LunarYearData {
        year: 1955,
        new_year_solar: (1, 24),
        leap_month: 3,
        month_lengths: 0b1_1110_1010_0001,
        total_days: 384,
    },
    // 1956 (병신년) - no leap
    LunarYearData {
        year: 1956,
        new_year_solar: (2, 12),
        leap_month: 0,
        month_lengths: 0b1101_0100_1010,
        total_days: 354,
    },
    // 1957 (정유년) - leap 8월
    LunarYearData {
        year: 1957,
        new_year_solar: (1, 31),
        leap_month: 8,
        month_lengths: 0b0_1101_1001_0101,
        total_days: 384,
    },
    // 1958 (무술년) - no leap
    LunarYearData {
        year: 1958,
        new_year_solar: (2, 19),
        leap_month: 0,
        month_lengths: 0b1010_1001_0110,
        total_days: 354,
    },
    // 1959 (기해년) - no leap
    LunarYearData {
        year: 1959,
        new_year_solar: (2, 8),
        leap_month: 0,
        month_lengths: 0b0101_0101_0110,
        total_days: 354,
    },
    // 1960 (경자년) - leap 6월
    LunarYearData {
        year: 1960,
        new_year_solar: (1, 28),
        leap_month: 6,
        month_lengths: 0b0_0101_0111_0101,
        total_days: 384,
    },
    // 1961 (신축년) - no leap
    LunarYearData {
        year: 1961,
        new_year_solar: (2, 15),
        leap_month: 0,
        month_lengths: 0b1010_1101_0101,
        total_days: 355,
    },
    // 1962 (임인년) - no leap
    LunarYearData {
        year: 1962,
        new_year_solar: (2, 5),
        leap_month: 0,
        month_lengths: 0b0110_1101_0010,
        total_days: 354,
    },
    // 1963 (계묘년) - leap 4월
    LunarYearData {
        year: 1963,
        new_year_solar: (1, 25),
        leap_month: 4,
        month_lengths: 0b0_0111_0101_0101,
        total_days: 384,
    },
    // 1964 (갑진년) - no leap
    LunarYearData {
        year: 1964,
        new_year_solar: (2, 13),
        leap_month: 0,
        month_lengths: 0b1110_1010_0101,
        total_days: 355,
    },
    // 1965 (을사년) - no leap
    LunarYearData {
        year: 1965,
        new_year_solar: (2, 2),
        leap_month: 0,
        month_lengths: 0b1110_0100_1010,
        total_days: 354,
    },
    // 1966 (병오년) - leap 3월
    LunarYearData {
        year: 1966,
        new_year_solar: (1, 22),
        leap_month: 3,
        month_lengths: 0b0_0110_0100_1110,
        total_days: 383,
    },
    // 1967 (정미년) - no leap
    LunarYearData {
        year: 1967,
        new_year_solar: (2, 9),
        leap_month: 0,
        month_lengths: 0b1010_1001_1011,
        total_days: 355,
    },
    // 1968 (무신년) - leap 7월
    LunarYearData {
        year: 1968,
        new_year_solar: (1, 30),
        leap_month: 7,
        month_lengths: 0b0_1010_1101_0110,
        total_days: 384,
    },
    // 1969 (기유년) - no leap
    LunarYearData {
        year: 1969,
        new_year_solar: (2, 17),
        leap_month: 0,
        month_lengths: 0b0101_0110_1010,
        total_days: 354,
    },
    // 1970 (경술년) - no leap
    LunarYearData {
        year: 1970,
        new_year_solar: (2, 6),
        leap_month: 0,
        month_lengths: 0b1011_0101_1001,
        total_days: 355,
    },
    // 1971 (신해년) - leap 5월
    LunarYearData {
        year: 1971,
        new_year_solar: (1, 27),
        leap_month: 5,
        month_lengths: 0b0_1011_1011_0010,
        total_days: 384,
    },
    // 1972 (임자년) - no leap
    LunarYearData {
        year: 1972,
        new_year_solar: (2, 15),
        leap_month: 0,
        month_lengths: 0b0111_0101_0010,
        total_days: 354,
    },
    // 1973 (계축년) - no leap
    LunarYearData {
        year: 1973,
        new_year_solar: (2, 3),
        leap_month: 0,
        month_lengths: 0b0111_0010_0101,
        total_days: 354,
    },
    // 1974 (갑인년) - leap 4월
    LunarYearData {
        year: 1974,
        new_year_solar: (1, 23),
        leap_month: 4,
        month_lengths: 0b0_1011_0010_1011,
        total_days: 384,
    },
    // 1975 (을묘년) - no leap
    LunarYearData {
        year: 1975,
        new_year_solar: (2, 11),
        leap_month: 0,
        month_lengths: 0b1010_0100_1011,
        total_days: 354,
    },
    // 1976 (병진년) - leap 8월
    LunarYearData {
        year: 1976,
        new_year_solar: (1, 31),
        leap_month: 8,
        month_lengths: 0b0_1001_1010_1011,
        total_days: 384,
    },
    // 1977 (정사년) - no leap
    LunarYearData {
        year: 1977,
        new_year_solar: (2, 18),
        leap_month: 0,
        month_lengths: 0b0010_1010_1101,
        total_days: 354,
    },
    // 1978 (무오년) - no leap
    LunarYearData {
        year: 1978,
        new_year_solar: (2, 7),
        leap_month: 0,
        month_lengths: 0b0101_0110_1011,
        total_days: 355,
    },
    // 1979 (기미년) - leap 6월
    LunarYearData {
        year: 1979,
        new_year_solar: (1, 28),
        leap_month: 6,
        month_lengths: 0b1_0101_1010_1001,
        total_days: 384,
    },
    // 1980 (경신년) - no leap
    LunarYearData {
        year: 1980,
        new_year_solar: (2, 16),
        leap_month: 0,
        month_lengths: 0b1101_1010_1001,
        total_days: 355,
    },
    // 1981 (신유년) - no leap
    LunarYearData {
        year: 1981,
        new_year_solar: (2, 5),
        leap_month: 0,
        month_lengths: 0b1101_1001_0010,
        total_days: 354,
    },
    // 1982 (임술년) - leap 4월
    LunarYearData {
        year: 1982,
        new_year_solar: (1, 25),
        leap_month: 4,
        month_lengths: 0b0_1101_1001_0101,
        total_days: 384,
    },
    // 1983 (계해년) - no leap
    LunarYearData {
        year: 1983,
        new_year_solar: (2, 13),
        leap_month: 0,
        month_lengths: 0b1101_0010_0101,
        total_days: 354,
    },
    // 1984 (갑자년) - leap 10월
    LunarYearData {
        year: 1984,
        new_year_solar: (2, 2),
        leap_month: 10,
        month_lengths: 0b0_1110_0100_1101,
        total_days: 384,
    },
    // 1985 (을축년) - no leap
    LunarYearData {
        year: 1985,
        new_year_solar: (2, 20),
        leap_month: 0,
        month_lengths: 0b1010_0101_0110,
        total_days: 354,
    },
    // 1986 (병인년) - no leap
    LunarYearData {
        year: 1986,
        new_year_solar: (2, 9),
        leap_month: 0,
        month_lengths: 0b0010_1011_0110,
        total_days: 354,
    },
    // 1987 (정묘년) - leap 6월
    LunarYearData {
        year: 1987,
        new_year_solar: (1, 29),
        leap_month: 6,
        month_lengths: 0b0_1010_1110_1101,
        total_days: 385,
    },
    // 1988 (무진년) - no leap
    LunarYearData {
        year: 1988,
        new_year_solar: (2, 18),
        leap_month: 0,
        month_lengths: 0b0110_1101_0100,
        total_days: 354,
    },
    // 1989 (기사년) - no leap
    LunarYearData {
        year: 1989,
        new_year_solar: (2, 6),
        leap_month: 0,
        month_lengths: 0b1101_1010_1001,
        total_days: 355,
    },
    // 1990 (경오년) - leap 5월
    LunarYearData {
        year: 1990,
        new_year_solar: (1, 27),
        leap_month: 5,
        month_lengths: 0b0_1110_1101_0010,
        total_days: 384,
    },
    // 1991 (신미년) - no leap
    LunarYearData {
        year: 1991,
        new_year_solar: (2, 15),
        leap_month: 0,
        month_lengths: 0b1110_1001_0010,
        total_days: 354,
    },
    // 1992 (임신년) - no leap
    LunarYearData {
        year: 1992,
        new_year_solar: (2, 4),
        leap_month: 0,
        month_lengths: 0b1101_0010_0110,
        total_days: 354,
    },
    // 1993 (계유년) - leap 3월
    LunarYearData {
        year: 1993,
        new_year_solar: (1, 23),
        leap_month: 3,
        month_lengths: 0b0_0101_0010_1110,
        total_days: 383,
    },
    // 1994 (갑술년) - no leap
    LunarYearData {
        year: 1994,
        new_year_solar: (2, 10),
        leap_month: 0,
        month_lengths: 0b1010_0101_0111,
        total_days: 355,
    },
    // 1995 (을해년) - leap 8월
    LunarYearData {
        year: 1995,
        new_year_solar: (1, 31),
        leap_month: 8,
        month_lengths: 0b0_1001_1011_0110,
        total_days: 384,
    },
    // 1996 (병자년) - no leap
    LunarYearData {
        year: 1996,
        new_year_solar: (2, 19),
        leap_month: 0,
        month_lengths: 0b1011_0101_1010,
        total_days: 355,
    },
    // 1997 (정축년) - no leap
    LunarYearData {
        year: 1997,
        new_year_solar: (2, 8),
        leap_month: 0,
        month_lengths: 0b0110_1101_0100,
        total_days: 354,
    },
    // 1998 (무인년) - leap 5월
    LunarYearData {
        year: 1998,
        new_year_solar: (1, 28),
        leap_month: 5,
        month_lengths: 0b0_0111_0110_1001,
        total_days: 384,
    },
    // 1999 (기묘년) - no leap
    LunarYearData {
        year: 1999,
        new_year_solar: (2, 16),
        leap_month: 0,
        month_lengths: 0b0111_0100_1001,
        total_days: 354,
    },
    // 2000 (경진년) - no leap
    LunarYearData {
        year: 2000,
        new_year_solar: (2, 5),
        leap_month: 0,
        month_lengths: 0b0110_1001_0011,
        total_days: 354,
    },
    // 2001 (신사년) - leap 4월
    LunarYearData {
        year: 2001,
        new_year_solar: (1, 24),
        leap_month: 4,
        month_lengths: 0b0_1010_1001_0111,
        total_days: 384,
    },
    // 2002 (임오년) - no leap
    LunarYearData {
        year: 2002,
        new_year_solar: (2, 12),
        leap_month: 0,
        month_lengths: 0b0101_0010_1011,
        total_days: 354,
    },
    // 2003 (계미년) - no leap
    LunarYearData {
        year: 2003,
        new_year_solar: (2, 1),
        leap_month: 0,
        month_lengths: 0b1010_0101_1011,
        total_days: 355,
    },
    // 2004 (갑신년) - leap 2월
    LunarYearData {
        year: 2004,
        new_year_solar: (1, 22),
        leap_month: 2,
        month_lengths: 0b0_1010_1010_1110,
        total_days: 384,
    },
    // 2005 (을유년) - no leap
    LunarYearData {
        year: 2005,
        new_year_solar: (2, 9),
        leap_month: 0,
        month_lengths: 0b0011_0110_1010,
        total_days: 354,
    },
    // 2006 (병술년) - leap 7월
    LunarYearData {
        year: 2006,
        new_year_solar: (1, 29),
        leap_month: 7,
        month_lengths: 0b0_1101_1101_0101,
        total_days: 385,
    },
    // 2007 (정해년) - no leap
    LunarYearData {
        year: 2007,
        new_year_solar: (2, 18),
        leap_month: 0,
        month_lengths: 0b1011_1010_0100,
        total_days: 354,
    },
    // 2008 (무자년) - no leap
    LunarYearData {
        year: 2008,
        new_year_solar: (2, 7),
        leap_month: 0,
        month_lengths: 0b1011_0100_1001,
        total_days: 354,
    },
    // 2009 (기축년) - leap 5월
    LunarYearData {
        year: 2009,
        new_year_solar: (1, 26),
        leap_month: 5,
        month_lengths: 0b0_1101_0101_0011,
        total_days: 384,
    },
    // 2010 (경인년) - no leap
    LunarYearData {
        year: 2010,
        new_year_solar: (2, 14),
        leap_month: 0,
        month_lengths: 0b1010_1001_0101,
        total_days: 354,
    },
    // 2011 (신묘년) - no leap
    LunarYearData {
        year: 2011,
        new_year_solar: (2, 3),
        leap_month: 0,
        month_lengths: 0b0101_0010_1101,
        total_days: 354,
    },
    // 2012 (임진년) - leap 3월
    LunarYearData {
        year: 2012,
        new_year_solar: (1, 23),
        leap_month: 3,
        month_lengths: 0b1_0101_0010_1101,
        total_days: 384,
    },
    // 2013 (계사년) - no leap
    LunarYearData {
        year: 2013,
        new_year_solar: (2, 10),
        leap_month: 0,
        month_lengths: 0b1010_1010_1101,
        total_days: 355,
    },
    // 2014 (갑오년) - leap 9월
    LunarYearData {
        year: 2014,
        new_year_solar: (1, 31),
        leap_month: 9,
        month_lengths: 0b0_1011_1010_1010,
        total_days: 384,
    },
    // 2015 (을미년) - no leap
    LunarYearData {
        year: 2015,
        new_year_solar: (2, 19),
        leap_month: 0,
        month_lengths: 0b0101_1101_0010,
        total_days: 354,
    },
    // 2016 (병신년) - no leap
    LunarYearData {
        year: 2016,
        new_year_solar: (2, 8),
        leap_month: 0,
        month_lengths: 0b1101_1010_0101,
        total_days: 355,
    },
    // 2017 (정유년) - leap 5월
    LunarYearData {
        year: 2017,
        new_year_solar: (1, 28),
        leap_month: 5,
        month_lengths: 0b0_1110_1010_1010,
        total_days: 384,
    },
    // 2018 (무술년) - no leap
    LunarYearData {
        year: 2018,
        new_year_solar: (2, 16),
        leap_month: 0,
        month_lengths: 0b1101_0100_1010,
        total_days: 354,
    },
    // 2019 (기해년) - no leap
    LunarYearData {
        year: 2019,
        new_year_solar: (2, 5),
        leap_month: 0,
        month_lengths: 0b1010_1001_0101,
        total_days: 354,
    },
];

/// Get lunar year data for a given Gregorian year (1940-2019).
pub fn get_lunar_year_1940_2019(year: u16) -> Option<&'static LunarYearData> {
    if year < 1940 || year > 2019 {
        return None;
    }
    Some(&LUNAR_YEARS_1940_2019[(year - 1940) as usize])
}
