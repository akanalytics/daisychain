use log::Level::Trace;
use log::{log_enabled, trace};

use crate::prelude::Matchable;
use crate::{util, LABEL, LOG_TARGET};
use std::fmt::Debug;

pub(crate) trait Loggable {
    const LABEL_WIDTH: usize = 15;
    const INPUT_WIDTH: usize = 35;
    fn log_inputs<Args: Debug>(&self, msg: &str, args: Args);
    fn log_success<Args: Debug>(&self, msg: &str, args: Args);
    fn log_success_with_result<Args1: Debug, Args2: Debug>(&self, m: &str, args: Args1, res: Args2);
    fn log_failure<Args: Debug, Error: Debug>(&self, msg: &str, args: Args, error: &Error);
}

impl<'a, Cur> Loggable for Cur
where
    Cur: Matchable<'a>,
{
    fn log_inputs<Args: Debug>(&self, msg: &str, args: Args) {
        if log_enabled!(target: LOG_TARGET, Trace) && self.is_skip() {
            trace!(
                target: LOG_TARGET,
                "{inp:<iw$} {label:<lw$} : {operation:<lw$}",
                iw = Self::INPUT_WIDTH,
                lw = Self::LABEL_WIDTH,
                label = LABEL.with(|f| f.get()),
                inp = util::formatter_str(self.str().unwrap_or_default()),
                operation = format!("{msg}({args:?})"),
            );
        }
    }
    fn log_success<Args: Debug>(&self, msg: &str, args: Args) {
        trace!(
            target: LOG_TARGET,
            "{inp:<iw$} {label:<lw$} : {operation:<lw$}",
            iw = Self::INPUT_WIDTH,
            lw = Self::LABEL_WIDTH,
            label = LABEL.with(|f| f.get()),
            inp = util::formatter_str(self.str().unwrap_or_default()),
            operation = format!("{msg}({args:?})"),
        );
    }
    fn log_success_with_result<A1: Debug, A2: Debug>(&self, msg: &str, args: A1, res: A2) {
        trace!(
            target: LOG_TARGET,
            "{inp:<iw$} {label:<lw$} : {operation:<lw$} -> {res:?}",
            iw = Self::INPUT_WIDTH,
            lw = Self::LABEL_WIDTH,
            label = LABEL.with(|f| f.get()),
            inp = util::formatter_str(self.str().unwrap_or_default()),
            operation = format!("{msg}:{args:?}"),
        );
    }
    fn log_failure<Args: Debug, Error: Debug>(&self, msg: &str, args: Args, error: &Error) {
        trace!(
            target: LOG_TARGET,
            "{inp:<iw$} {label:<lw$} : {operation:<lw$} -> {e:?}",
            iw = Self::INPUT_WIDTH,
            lw = Self::LABEL_WIDTH,
            label = LABEL.with(|f| f.get()),
            inp = util::formatter_str(self.str().unwrap_or_default()),
            operation = format!("{msg}({args:?})"),
            e = error,
        );
    }
}
