import pytest
import serial as pyserial


@pytest.fixture
def serial():
    """The serial interface to Zissou."""

    # N.B. Everything after the yield is teardown
    with pyserial.Serial("/dev/zissouserial") as s:
        yield s


def test_loopback(serial):
    """Current behaviour - will change."""

    test_str = "Hello!"

    sent = serial.write(test_str.encode())
    assert sent == len(test_str)

    read = serial.read_all()
    assert read.decode() == test_str.upper()
