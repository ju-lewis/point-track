
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



