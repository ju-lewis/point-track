
# PointTrack Module components:

## MCU
ESP32-S3 WROOM-1
- Built-in PCB antenna
- Built-in 
- up to 16MB PSRAM (R16 variant)
- up to 16MB program flash

Currently, I think the best choice is `ESP32-S3-WROOM-1-N16R8`, with 8MB PSRAM and 16MB flash


## GNSS Receiver
Ublox ZED-F9P
- UART config and output support

## Li-Ion battery
- 2S bat, 2x18650??? (Would provide plenty of capacity)

## Li-Ion charge controller IC


## Display
Any HD44780 compatible 16x2 display is fine



# Currently implemented in the design:

- [x] ESP32-S3 WROOM-1-N16R8
- [x] UBlox ZED-F9P (including LNA antenna SMA connector and bias-T)
- [x] I2C OLED Display
- [x] LDO low-noise linear regulator
- [x] 1x 18650 Holder
- [x] Li-ion battery charger IC
- [x] Pin headers for flashing esp code
- [x] Pin headers for flashing f9p code
- [x] Master switch
- [x] USB-C port
- [ ] Connector for button panel

- [ ] Button panel schematic + PCB design


# Power draw calculations

ESP32: (WiFi TX active: 300mA) (WiFi RX active: 100mA)
ZED-F9P: 90mA
OLED: 20mA
LNA Antenna: 15mA

Total current draw: 225mA (up to 500mA worst case scenario)

Desired battery life: ~8 hours

Needed capacity: 1800mAh



# Charging circuit:

Need to set R_{ILIM}

Max input current = K_{ILIM} / R_{ILIM} = 1.25A (required by 18650)
typ. current factor = 1610 A\Omega 
So I need 1610/1.25 ~= 1.3k resistor. (Actual output = 1.238 A)


# Final Pinouts:

## Button panel:
- IO 14,15,16,17,18

## ZED-F9P:
- Reset: IO 6
- MOSI: IO 5
- MISO: IO 4

## 128x64 OLED
- SDA: IO 9
- SCL: IO 10


# Trace widths:
Standard: 0.254mm
Battery rail: 0.5mm
Charge IC output: 0.4mm


# NOTE:

When programming the ZED-F9P and ESP32, connect TX to TX and RX to RX. The UART pin headers are intended to be read from the perspective of the master device.

## Final board dimensions:
Board: 50.038mm * 99.949mm
Height: 14.86mm + 10mm = 24.9mm

19.812mm to the edge of the USB-C port

2.159mm to the edge of the SMA connector
41.148mm to the other edge of the SMA connector

0.635mm to the side of the switch
1.651mm to the other side of the switch




