use littlefs;
use stm32l4xx_hal as hal;
use hal::prelude::*;

const START_PAGE: u8 = 96;
const PAGE_SIZE: usize = 2048;
const PAGE_SIZE_POW: usize = 11;
const START_ADDR: usize = 2048 * START_PAGE as usize;

pub struct Gordon (hal::flash::Flash);

impl Gordon {
    pub fn new(flash: hal::flash::Flash) -> Gordon {
        Self(flash)
    }
}

impl littlefs::Storage for Gordon {
    fn read(&self, off: usize, buf: &mut [u8]) -> Result<usize, littlefs::FsError> {
        self.0.read(START_ADDR + off, buf);
        Ok(buf.len())
    }
    fn write(&mut self, off: usize, data: &[u8]) -> Result<usize, littlefs::FsError> {
        let mut unlocked_flash = self.0.unlocked();
        unlocked_flash.write(START_ADDR + off, data).unwrap();
        Ok(data.len())
    }
    fn erase(&mut self, off: usize, len: usize) -> Result<usize, littlefs::FsError> {
        let mut unlocked_flash = self.0.unlocked();
        let pages = len >> PAGE_SIZE_POW;
        let first_page = START_PAGE + (off >> PAGE_SIZE_POW) as u8;
        for i in 0..pages {
            unlocked_flash.erase_page(first_page + i as u8).unwrap();
        }
        Ok(len)
    }
}
