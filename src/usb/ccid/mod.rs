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

const CCID_REQ_ABORT: u8 = 0x01;
const CCID_REQ_GET_CLOCK_FREQUENCIES: u8 = 0x02;
const CCID_REQ_GET_DATA_RATES: u8 = 0x03;

pub struct SmartCard<'a, B: UsbBus> {
    intf: InterfaceNumber, // need this?
    read_ep: EndpointOut<'a, B>,
    write_ep: EndpointIn<'a, B>,
    // intr_ep: EndpointIn<'a, B>,

    buf: [u8; 64],
    cmd_buf: [u8; 64],
    resp_buf: [u8; 64],
    len: usize,
    need_zlp: bool,
}

impl<B: UsbBus> SmartCard<'_, B> {
    pub fn new(alloc: &UsbBusAllocator<B>) -> SmartCard<'_, B> {
        SmartCard {
            intf: alloc.interface(),
            read_ep: alloc.bulk(64),
            write_ep: alloc.bulk(64),
            // intr_ep: alloc.interrupt(8, 255),

            buf: [0; 64],
            cmd_buf: [0; 64],
            resp_buf: [0; 64],
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

// /*
//  * ATR (Answer To Reset) string
//  *
//  * TS = 0x3b: Direct conversion
//  * T0 = 0xda: TA1, TC1 and TD1 follow, 10 historical bytes
//  * TA1 = 0x11: FI=1, DI=1
//  * TC1 = 0xff
//  * TD1 = 0x81: TD2 follows, T=1
//  * TD2 = 0xb1: TA3, TB3 and TD3 follow, T=1
//  * TA3 = 0xFE: IFSC = 254 bytes
//  * TB3 = 0x55: BWI = 5, CWI = 5   (BWT timeout 3.2 sec)
//  * TD3 = 0x1f: TA4 follows, T=15
//  * TA4 = 0x03: 5V or 3.3V
//  *
//  * Minimum: 0x3b, 0x8a, 0x80, 0x01
//  *
//  */
// static const uint8_t ATR_head[] = {
//   0x3b, 0xda, 0x11, 0xff, 0x81, 0xb1, 0xfe, 0x55, 0x1f, 0x03,
// };

impl<B: UsbBus> UsbClass<B> for SmartCard<'_, B> {
    fn poll(&mut self) {
        match self.read_ep.read(&mut self.cmd_buf) {
            Ok(count) => {
                if count > 0 {
                    match self.cmd_buf[0] {
                        // PC_to_RDR_IccPower_On message
                        0x62 => {
                            self.resp_buf[0] = 0x80; // RDR_to_PC_DataBlock
                            // self.resp_buf[1] = ATR.len(); // 4 bytes
                            self.resp_buf[1] = 0x04; // message specific data length
                            self.resp_buf[2] = 0x00; // is all zeros for some reason?
                            self.resp_buf[3] = 0x00;
                            self.resp_buf[4] = 0x00;

                            self.resp_buf[5] = 0x00; // bSlot <- "USB-ICC requires single slot"
                            self.resp_buf[6] = self.cmd_buf[6]; // sequence number of command
                            self.resp_buf[7] = 0x00; // status
                            self.resp_buf[8] = 0x00; // error
                            self.resp_buf[9] = 0x00; // chain parameter (0 = this is complete)

                            // Yubikey NEO: 3B 8C 80 01 59 75 62 69 6B 65 79 4E 45 4F 72 33 58
                            // (17 bytes)
                            // the actual ATR
                            // self.resp_buf[10] = 0x3b;
                            // self.resp_buf[11] = 0x8a;
                            // self.resp_buf[12] = 0x80;
                            // self.resp_buf[13] = 0x01;
                            //
                            //
                            // self.resp_buf[10..17].copy_from_slice(&[0x3bu8, 0x8c,0x80,0x01,0x59,0x75,0x62,0x69,0x6b,0x65,0x79,0x4e,0x45,0x4f,0x72,0x33,0x58]);
                            self.resp_buf[10..14].copy_from_slice(&[0x3bu8, 0x8a,0x80,0x01]);

                            /*
                             * not sure what's up here
                             *
                            // 00000011 ifdhandler.c:1154:IFDHPowerICC() action: PowerUp, usb:1209/cc1d:libudev:0:/dev/bus/usb/001/034 (lun: 0)
                            // 00000007 -> 000000 62 00 00 00 00 00 05 00 00 00
                            // 00000507 <- 000000 80 00 00 00 00 00 05 00 00 00 3B 8A 80 01
                            // 00000018 eventhandler.c:289:EHStatusHandlerThread() powerState: POWER_STATE_POWERED
                            // 00000004 eventhandler.c:298:EHStatusHandlerThread() Card ATR: (NULL)

                            *
                            * also if we send the NEO ATR, it crashes somehow
                            *
                            */

                            // match self.write_ep.write(&self.resp_buf[..17]) {
                            match self.write_ep.write(&self.resp_buf[..14]) {
                                // Ok(count) => Ok(count),
                                // Err(UsbError::WouldBlock) => Ok(0),
                                // e => e,
                                _ => (),
                            }
                        },
                        // PC_to_RDR_IccPower_Off message
                        0x63 | 0x65 => {
                            self.resp_buf[0] = 0x81; // RDR_to_PC_SlotStatus
                            // self.resp_buf[1] = ATR.len(); // 4 bytes
                            self.resp_buf[1] = 0x00; // message specific data length
                            self.resp_buf[2] = 0x00; // is all zeros for some reason?
                            self.resp_buf[3] = 0x00;
                            self.resp_buf[4] = 0x00;

                            self.resp_buf[5] = 0x00; // bSlot <- "USB-ICC requires single slot"
                            self.resp_buf[6] = self.cmd_buf[6]; // sequence number of command
                            // self.resp_buf[7] = 0x02; // status
                            self.resp_buf[7] = 0x00; // status
                            self.resp_buf[8] = 0x00; // error
                            self.resp_buf[9] = 0x00; // chain parameter (0 = this is complete)

                            match self.write_ep.write(&self.resp_buf[..10]) {
                                // Ok(count) => Ok(count),
                                // Err(UsbError::WouldBlock) => Ok(0),
                                // e => e,
                                _ => (),
                            }
                        },
                        _ => (),

                    }
                }
            },
            Err(UsbError::WouldBlock) => { },
            Err(err) => panic!("bulk read {:?}", err),
        };
    }

    fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<()> {
        writer.interface(
            self.intf,
            USB_CLASS_CCID,
            USB_SUBCLASS_NONE,
            CCID_PROTOCOL_CCID,
            // CCID_PROTOCOL_ICCD_B,
        )?;

        // TODO: wrap this in something nicer, just stealing Yubikey values here
        #[rustfmt::skip]
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
                                         // gnuk: 9600
                0x00, 0xb0, 0x04, 0x00,  // dwMaxDataRate (same)
                                         // gnuk: 9600
                0x00,  // bNumDataRatesSupported
                0xf6, 0x07, 0x00, 0x00,  // dwMaxIFSD (2038)
                                         // gnuk: 254
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
                                         // gnuk: 271
                0xFF,  // bClassGetResponse ("echo")
                0xFF,  // bClassEnvelope ("echo")
                       // gnuk: 0
                0x00, 0x00,  // wlcdLayout (none)
                0x00,  // bPinSupport (none -- 0x01 = verification, 0x02 = modification)
                0x01,  // bMaxCCIDBusySlots
            ])?;

        writer.endpoint(&self.write_ep)?;
        writer.endpoint(&self.read_ep)?;
        // writer.endpoint(&self.intr_ep)?;

        Ok(())
    }

    fn endpoint_in_complete(&mut self, addr: EndpointAddress) {
        if self.need_zlp && addr == self.write_ep.address() {
            self.need_zlp = false;
            self.write_ep.write(&[]).ok();
        }
    }

    fn control_out(&mut self, xfer: ControlOut<B>) {
        let req = xfer.request();

        if req.request_type == control::RequestType::Class
            && req.recipient == control::Recipient::Interface
                // also need to check we are the intended interface index?
        {
            return match req.request {
                CCID_REQ_ABORT => {
                    // either accept or reject?
                    // in any case, not implemented yet
                    xfer.reject().ok();
                },
                _ => (),
            };
        }
    }

    fn control_in(&mut self, xfer: ControlIn<B>) {
        let req = xfer.request();

        if !(req.request_type == control::RequestType::Class
            && req.recipient == control::Recipient::Interface
            // && req.index == u8::from(self.comm_if) as u16)
            )
        {
            return;
        }

        // TODO: share constants with descriptor
        match req.request {
            CCID_REQ_GET_CLOCK_FREQUENCIES => {
                xfer.accept(|data| {
                    data[0] = 0xA0;
                    data[1] = 0x0F;
                    data[2] = 0x00;
                    data[3] = 0x00;
                    Ok(4)
                }).ok();
            },
            CCID_REQ_GET_DATA_RATES => {
                xfer.accept(|data| {
                    data[0] = 0x00;
                    data[1] = 0xb0;
                    data[2] = 0x04;
                    data[3] = 0x00;
                    Ok(4)
                }).ok();
            },
            _ => (),
        }
    }
}
