import struct

import pytest
import usb.core

vid = 0x1209
pid = 0xCC1D

CCID_CLASS = 0x0B
CCID_SUBCLASS = 0x00
CCID_PROTOCOL_0 = 0x00

from usb.util import (
    find_descriptor,
    claim_interface,
    ENDPOINT_TYPE_BULK,
    ENDPOINT_OUT,
    ENDPOINT_IN,
    endpoint_type,
    endpoint_direction,
)


def ccid_compose(msg_type, seq, slot=0, rsv=0, param=0, data=b""):
    return struct.pack('<BiBBBH', msg_type, len(data), slot, seq, rsv, param) + data


class CCID:
    def __init__(self, dev):
        self.dev = dev
        self.cfg = cfg = dev.get_active_configuration()

        self.intf = intf = find_descriptor(
            cfg,
            bInterfaceClass=CCID_CLASS,
            bInterfaceSubClass=CCID_SUBCLASS,
            # bInterfaceProtocol=CCID_PROTOCOL_0,
        )
        if intf is None:
            raise ValueError("Not a CCID device")

        claim_interface(dev, intf)

        for ep in intf:
            if (
                endpoint_type(ep.bmAttributes) == ENDPOINT_TYPE_BULK
                and endpoint_direction(ep.bEndpointAddress) == ENDPOINT_OUT
            ):
                self.bulkout = ep.bEndpointAddress
            if (
                endpoint_type(ep.bmAttributes) == ENDPOINT_TYPE_BULK
                and endpoint_direction(ep.bEndpointAddress) == ENDPOINT_IN
            ):
                self.bulkin = ep.bEndpointAddress

        self.seq = 0
        self.timeout = 10000

    def increment_seq(self):
        self.seq = (self.seq + 1) & 0xFF

    def ccid_get_result(self):
        msg = self.dev.read(self.bulkin, 1024, self.timeout)

        if len(msg) < 10:
            print(msg)
            raise ValueError("ccid_get_result")

        # assert len(msg) == 14

        msg_type = msg[0]
        data_len = msg[1] + (msg[2] << 8) + (msg[3] << 16) + (msg[4] << 24)
        slot = msg[5]
        seq = msg[6]
        status = msg[7]
        error = msg[8]
        chain = msg[9]
        data = msg[10:]
        # XXX: check msg_type, data_len, slot, seq, error
        return None, None, msg
        return (status, chain, data.tobytes())

    def power_on(self):
        msg = ccid_compose(0x62, self.seq, rsv=1)  # Vcc=5V
        self.dev.write(self.bulkout, msg, self.timeout)
        self.increment_seq()
        status, chain, data = self.ccid_get_result()
        # XXX: check status, chain
        self.atr = atr = data

        return atr

    def power_off(self):
        msg = ccid_compose(0x63, self.seq, rsv=1)  # Vcc=5V
        self.dev.write(self.bulkout, msg, self.timeout)
        self.increment_seq()
        stuff = status, chain, data = self.ccid_get_result()
        print(f"data = {data}")
        # XXX: check status, chain

        return stuff


@pytest.fixture
def ccid():
    """The CCID interface to Zissou."""

    dev = usb.core.find(idVendor=vid, idProduct=pid)
    assert dev is not None

    yield CCID(dev)

    # do teardown here if necessary
    pass


def test_connect(ccid):
    """Check access (udev."""

    assert None == ccid.power_off()
    atr = ccid.power_on()

    assert atr == b"123"
