use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, Utc};

use crate::{hidden::GeneratorContext, FromNet, Net, ToNet};

const NANOS_PER_TICK: u64 = 100;
const TICKS_PER_SECOND: u64 = 1_000_000_000 / NANOS_PER_TICK;
const TICKS_PER_DAY: u64 = TICKS_PER_SECOND * 60 * 60 * 24;

unsafe impl Net for NaiveDateTime {
    type Raw = i64;

    fn gen_type(_ctx: &mut GeneratorContext) -> Box<str> {
        "DateTime".into()
    }

    fn gen_raw_type(_ctx: &mut GeneratorContext) -> Box<str> {
        "long".into()
    }

    fn is_nullable(_ctx: &mut GeneratorContext) -> bool {
        false
    }
}

unsafe impl FromNet for NaiveDateTime {
    unsafe fn from_raw(arg: Self::Raw) -> Self {
        let days = ((arg as u64) / TICKS_PER_DAY + 1) as i32;
        let nanos = ((arg as u64) % TICKS_PER_DAY) * NANOS_PER_TICK;
        NaiveDate::from_num_days_from_ce(days)
            .and_time(NaiveTime::from_hms(0, 0, 0) + chrono::Duration::nanoseconds(nanos as i64))
    }

    fn gen_marshal(_ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        format!("({}).Ticks", arg).into()
    }
}

unsafe impl ToNet for NaiveDateTime {
    fn into_raw(self) -> Self::Raw {
        let days = (self.num_days_from_ce() - 1) as i64;
        let nanos = (self.time() - NaiveTime::from_hms(0, 0, 0))
            .num_nanoseconds()
            .unwrap();
        (days * (TICKS_PER_DAY as i64)) + (nanos / (NANOS_PER_TICK as i64))
    }

    fn gen_marshal(_ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        format!("new DateTime({}, DateTimeKind.Unspecified)", arg).into()
    }
}

unsafe impl Net for DateTime<Utc> {
    type Raw = i64;

    fn gen_type(_ctx: &mut GeneratorContext) -> Box<str> {
        "DateTime".into()
    }

    fn gen_raw_type(_ctx: &mut GeneratorContext) -> Box<str> {
        "long".into()
    }

    fn is_nullable(_ctx: &mut GeneratorContext) -> bool {
        false
    }
}

unsafe impl FromNet for DateTime<Utc> {
    unsafe fn from_raw(arg: Self::Raw) -> Self {
        DateTime::from_utc(NaiveDateTime::from_raw(arg), Utc)
    }

    fn gen_marshal(_ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        format!("({}).ToUniversalTime().Ticks", arg).into()
    }
}

unsafe impl ToNet for DateTime<Utc> {
    fn into_raw(self) -> Self::Raw {
        self.naive_utc().into_raw()
    }

    fn gen_marshal(_ctx: &mut GeneratorContext, arg: &str) -> Box<str> {
        format!("new DateTime({}, DateTimeKind.Utc)", arg).into()
    }
}
