#include "wireless.h"
#include "peripherals.h"
#include <WiFi.h>

void printSSID(LiquidCrystal lcd, String SSID);
bool connectToNetwork(LiquidCrystal lcd, String SSID);

int getNumNetworks() {
    return (int)WiFi.scanNetworks();
}



void scanAndConnect(LiquidCrystal lcd) {

    // Notify user of network scanning
    lcd.clear();
    lcd.print("Finding networks");
    lcd.setCursor(0,1);
    lcd.print("Please wait.");
    
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
        if(wasPressed(ACTION_BTN)) {
            // Network chosen, go to password entry prompt
            bool connection_status = connectToNetwork(lcd, WiFi.SSID(curr));
            if(connection_status) {
                lcd.clear();
                lcd.print("Connected.");
                connected = true;
            } else {
                lcd.clear();
                lcd.print("Connection fail");
                lcd.setCursor(0,1);
                lcd.print("Try again.");
                delay(2000);
                printSSID(lcd, WiFi.SSID(curr));
            }
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



bool connectToNetwork(LiquidCrystal lcd, String SSID) {

    char *chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*().,_-~:;'\"`|+=?<> ";
    int numChars = strlen(chars);
    int currChar = numChars-1;

    String password = "";

    
    lcd.clear();
    lcd.setCursor(0,0);
    lcd.print(SSID);
    lcd.setCursor(0,1);
    lcd.cursor(); // Clearly display cursor

    // idk bruh
    pinMode(DIGIT_2_BTN, INPUT_PULLUP);

    bool passwordEntered = false;
    while(!passwordEntered) {
        if(wasPressed(DIGIT_1_BTN)) {
            // Cycle to next character

            currChar += 1;
            if(currChar >= numChars) {
                currChar = 0;
            }
            lcd.write(chars[currChar]);
            lcd.setCursor(password.length(), 1);
        }
        if(wasPressed(DIGIT_2_BTN)) {
            // Backspace

            if(password.length() > 0) {
                lcd.write(" "); // Clear current unconfirmed character
                password.remove(password.length() - 1); // Remove from password
                lcd.setCursor(password.length(), 1); 
                lcd.write(" "); // Clear last confirmed character
                lcd.setCursor(password.length(), 1); // Finally reset position
            }
        }
        if(wasPressed(DIGIT_3_BTN)) {
            // Cycle to previous character

            currChar -= 1;
            if(currChar < 0) {
                currChar = numChars - 1;
            }
            lcd.write(chars[currChar]);
            lcd.setCursor(password.length(), 1);
        }
        if(wasPressed(ENTER_BTN)) {
            // Enter character

            password += chars[currChar];
            lcd.write(chars[currChar]);
            lcd.setCursor(password.length(), 1);
        }
        if(wasPressed(ACTION_BTN)) {
            passwordEntered = true;
        }
    }

    Serial.println(password);

    lcd.noCursor();
    lcd.clear();
    lcd.print("Connecting...");

    WiFi.begin(SSID, password);
    delay(5000);

    if(WiFi.status() == WL_CONNECTED) return true;

    return false;
}



