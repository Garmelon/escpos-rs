use escpos::printer::Printer;
use escpos::utils::*;
use escpos::{driver::*, errors::Result};

const EURO: &[u8] = &[0xD5]; // '€' in code page PC858

fn main() -> Result<()> {
    env_logger::init();

    // let driver = NetworkDriver::open("192.168.1.248", 9100)?;
    let driver = ConsoleDriver::open(true);
    Printer::new(driver, Protocol::default(), Some(PageCode::PC858))
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .writeln("Test with page code PC858:")?
        .writeln("€, é, à, À, Ô")?
        .feeds(2)?
        .page_code(PageCode::PC437)?
        .writeln("Test with page code PC437:")?
        .writeln("€, é, à, À, Ô")?
        .feeds(2)?
        .page_code(PageCode::PC858)?
        .writeln("Test with custom command:")?
        .custom(EURO)?
        .feeds(2)?
        .print_cut()?;

    Ok(())
}
