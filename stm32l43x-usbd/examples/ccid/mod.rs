/// Minimal and incomplete CDC-ACM implementation for the examples - this will eventually be a real
/// crate!

use core::cmp::min;
use usb_device::class_prelude::*;
use usb_device::Result;

#[allow(dead_code)]
const USB_CLASS_NONE: u8 = 0x00;
#[allow(dead_code)]
const USB_CLASS_CCID: u8 = 0x0B;
#[allow(dead_code)]
const USB_SUBCLASS_NONE: u8 = 0x0;

const CCID_PROTOCOL_CCID: u8 = 0x00;
#[allow(dead_code)]
const CCID_PROTOCOL_ICCD_A: u8 = 0x01;
#[allow(dead_code)]
const CCID_PROTOCOL_ICCD_B: u8 = 0x02;

const CCID_FUNCTIONAL_INTERFACE: u8 = 0x21;


pub struct SmartCard<'a, B: UsbBus> {
    intf: InterfaceNumber,  // need this?
    read_ep: EndpointOut<'a, B>,
    write_ep: EndpointIn<'a, B>,
    intr_ep: EndpointIn<'a, B>,

    buf: [u8; 64],
    len: usize,
    need_zlp: bool,
}

impl<B: UsbBus> SmartCard<'_, B> {
    pub fn new(alloc: &UsbBusAllocator<B>) -> SmartCard<'_, B> {
        SmartCard {
            intf: alloc.interface(),
            read_ep: alloc.bulk(64),
            write_ep: alloc.bulk(64),
            intr_ep: alloc.interrupt(8, 255),

            buf: [0; 64],
            len: 0,
            need_zlp: false,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize> {
        if self.need_zlp {
            return Ok(0);
        }

        if data.len() == 64 {
            self.need_zlp = true;
        }

        match self.write_ep.write(data) {
            Ok(count) => Ok(count),
            Err(UsbError::WouldBlock) => Ok(0),
            e => e,
        }
    }

    pub fn read(&mut self, data: &mut [u8]) -> Result<usize> {
        // Terrible buffering implementation for brevity's sake

        if self.len == 0 {
            self.len = match self.read_ep.read(&mut self.buf) {
                Ok(0) | Err(UsbError::WouldBlock) => return Ok(0),
                Ok(count) => count,
                e => return e,
            };
        }

        let count = min(data.len(), self.len);

        &data[..count].copy_from_slice(&self.buf[0..count]);

        self.buf.rotate_left(count);
        self.len -= count;

        Ok(count)
    }
}

impl<B: UsbBus> UsbClass<B> for SmartCard<'_, B> {
    fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<()> {
        writer.interface(
            self.intf,
            USB_CLASS_CCID,
            USB_SUBCLASS_NONE,
            CCID_PROTOCOL_CCID)?;

        // TODO: wrap this in something nicer, just stealing Yubikey values here
        writer.write(
            CCID_FUNCTIONAL_INTERFACE,
            &[
                // 0x10, 0x01,  // bcdCCID rev1.10 <-- Linux doesn't know about this
                0x00, 0x01,  // bcdCCID rev1.00
                0x00,  // bMaxSlotIndex
                0x07,  // bVoltageSupport (5.0V+3.0V+1.8V)
                0x02, 0x00, 0x00, 0x00,  // dwProtocols T=1
                0xA0, 0x0F, 0x00, 0x00,  // dwDefaultClock (integer 4000 = 4 MHz)
                0xA0, 0x0F, 0x00, 0x00,  // dwMaximumClock (same)
                0x00,  // bNumClockSupported
                0x00, 0xb0, 0x04, 0x00,  // dwDataRate (integer,  307200 bps)
                0x00, 0xb0, 0x04, 0x00,  // dwMaxDataRate (same)
                0x00,  // bNumDAtaRatesSupported
                0xf6, 0x07, 0x00, 0x00,  // dwMaxIFSD (2038)
                0x00, 0x00, 0x00, 0x00,  // dwSyncProtocols
                0x00, 0x00, 0x00, 0x00,  // dwMechanical
                0xFE, 0x00, 0x04, 0x00,  // dwFeatures, see following comments
                // Auto configuration based on ATR
                // Auto activation on insert
                // Auto voltage selection
                // Auto clock change
                // Auto baud rate change
                // Auto parameter negotiation made by CCID
                // Short and extended APDU level exchange
                0x00, 0x0C, 0x00, 0x00,  // dwMaxCCIDMsgLen (3072)
                0xFF,  // bClassGetResponse ("echo")
                0xFF,  // bClassEnvelope ("echo")
                0x00, 0x00,  // wlcdLayout (none)
                0x00,  // bPinSupport (none -- 0x01 = verification, 0x02 = modification)
                0x01,  // bMaxCCIDBusySlots
            ])?;

        writer.endpoint(&self.write_ep)?;
        writer.endpoint(&self.read_ep)?;
        writer.endpoint(&self.intr_ep)?;


        Ok(())
    }

    fn endpoint_in_complete(&mut self, addr: EndpointAddress) {
        if self.need_zlp && addr == self.write_ep.address() {
            self.need_zlp = false;
            self.write_ep.write(&[]).ok();
        }
    }

}
