use crate::std::sync::Once;
use env_logger::Builder;
use log::{trace, LevelFilter};

/// Intialize log with default settings
pub fn init() {
    static START: Once = Once::new();

    START.call_once(|| {
        let mut builder = Builder::new();
        builder.filter(None, LevelFilter::Info);
        builder.parse_default_env();
        builder.init();
        trace!("logger initialized");
    })
}
