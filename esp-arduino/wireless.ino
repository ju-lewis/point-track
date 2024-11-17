#include "wireless.h"
#include "peripherals.h"
#include <WiFi.h>

int getNumNetworks() {
    return (int)WiFi.scanNetworks();
}



void scanAndConnect(LiquidCrystal lcd) {
    
    int n = getNumNetworks();
    int curr = 0;
    bool connected = false;
   
    // Reset screen
    lcd.clear();
    lcd.setCursor(0,0);
    lcd.print("Network:");
    lcd.setCursor(1,0);
    lcd.print(WiFi.SSID(curr));

    while(!connected) {

        if(wasPressed(DIGIT_1_BTN)) {
            curr -= 1;
            if(curr < 0) curr = n;
            lcd.setCursor(1,0);
            lcd.print(WiFi.SSID(curr));
        }
        if(wasPressed(DIGIT_3_BTN)) {
            curr += 1;
            if(curr > n) curr = 0;
            lcd.setCursor(1,0);
            lcd.print(WiFi.SSID(curr));
        }
        if(wasPressed(ENTER_BTN)) {
            // Network chosen, go to password entry prompt
            bool passwordEntered = false;
        }

    }

}




