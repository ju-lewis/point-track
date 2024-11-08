#include <LiquidCrystal.h>
#include "race_setup.h"
#include "peripherals.h"




void setup() {

    Serial.begin(9600);
    configurePins();

    LiquidCrystal lcd = LiquidCrystal(LCD_RS, LCD_ENABLE, LCD_D4, LCD_D5, LCD_D6, LCD_D7);
    lcd.begin(8, 2);
    
    RaceConfig config = enterConfig(lcd);
}

void loop() {
}



void configurePins() {
    pinMode(ENTER_BTN,   INPUT_PULLUP);
    pinMode(ACTION_BTN,  INPUT_PULLUP);
    pinMode(DIGIT_1_BTN, INPUT_PULLUP);
    pinMode(DIGIT_2_BTN, INPUT_PULLUP);
    pinMode(DIGIT_3_BTN, INPUT_PULLUP);
}


