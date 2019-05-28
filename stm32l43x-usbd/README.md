stm32l43x-usbd
===============

[usb-device](https://github.com/mvirkkunen/usb-device) implementation for:
- STM32L432 (Nucleo board)
- STM32L433 (additional LCD peripheral)
- STM32L442 (additional crypto peripheral)
- STM32L443 (additional both)

microcontrollers.



This is a fork of [stm32f103xx-usb](https://github.com/mvirkkunen/stm32f103xx-usb), adjusting for differences in the USB peripheral.



We will use [libusb_stm32](https://github.com/dmitrystu/libusb_stm32) as guideline for which adjustments need to be made, thanks to:
- [usbd_stm32f103_devfs.c](https://github.com/dmitrystu/libusb_stm32/blob/master/src/usbd_stm32f103_devfs.c) (using [startup code](https://github.com/dmitrystu/libusb_stm32/blob/master/demo/cdc_startup.c#L69-L80))

- [usbd_stm32l433_devfs.c](https://github.com/dmitrystu/libusb_stm32/blob/master/src/usbd_stm32l433_devfs.c) (using [startup code](https://github.com/dmitrystu/libusb_stm32/blob/master/demo/cdc_startup.c#L170-L185))

  

Initial collaborators are
- @nickray
- @szszszsz

All and any help is welcome!

