#include "wireless.h"
#include "peripherals.h"
#include <WiFi.h>

void printSSID(LiquidCrystal lcd, String SSID);
void connectToNetwork(LiquidCrystal lcd, String SSID);

int getNumNetworks() {
    return (int)WiFi.scanNetworks();
}



void scanAndConnect(LiquidCrystal lcd) {
    
    int n = getNumNetworks();
    int curr = 0;
    bool connected = false;
   
    // Reset screen
    printSSID(lcd, WiFi.SSID(curr));

    while(!connected) {

        if(wasPressed(DIGIT_1_BTN)) {
            curr -= 1;
            if(curr < 0) curr = n;
            printSSID(lcd, WiFi.SSID(curr));
        }
        if(wasPressed(DIGIT_3_BTN)) {
            curr += 1;
            if(curr > n) curr = 0;
            printSSID(lcd, WiFi.SSID(curr));
        }
        if(wasPressed(ENTER_BTN)) {
            // Network chosen, go to password entry prompt
            connectToNetwork(lcd, WiFi.SSID(curr));
        }

    }

}

void printSSID(LiquidCrystal lcd, String SSID) {
    lcd.clear();
    lcd.setCursor(0,0);
    lcd.print("Network:");
    lcd.setCursor(0,1);
    lcd.print(SSID);
}



void connectToNetwork(LiquidCrystal lcd, String SSID) {

    char *chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*() .,_-~:;'\"`|+=?<>";

    
    lcd.clear();
    lcd.setCursor(0,0);
    lcd.print("Password:");
    lcd.setCursor(0,1);
    //lcd.print("****************");

    bool connected = false;
    while(!connected) {
        
        connected = true;
    }
}
