
#include "race_setup.h"
#include "peripherals.h"
#include <string.h>

void writeCode(char *code, LiquidCrystal lcd);
void writeName(char *name, LiquidCrystal lcd);

char *resolveCurrentField(int state, RaceConfig *config);


RaceConfig enterConfig(LiquidCrystal lcd) {
    bool finished = false;
    int state = 0;
    char *names[3] = {"Boat Number", "Course Number", "Nominated Speed"};
    RaceConfig config = {
        {'0','0','0'},
        {'0','0','0'},
        {'0','0','0'}
    };


    // Init screen
    lcd.clear();
    lcd.setCursor(0,0);
    
    // Write initial code to the screen
    writeName(names[0], lcd);
    writeCode("000",    lcd);

    while(!finished) {

        // Current value buffer
        char *buf = resolveCurrentField(state, &config);
        if(!buf) {
            lcd.print("Config Error");
            return config;
        }
        
        if(wasPressed(DIGIT_1_BTN)) {
            buf[0]++;
            if(buf[0] > '9') {
                buf[0] = '0';
            }
            writeCode(buf, lcd);
        }
        if(wasPressed(DIGIT_2_BTN)) {
            buf[1]++;
            if(buf[1] > '9') {
                buf[1] = '0';
            }
            writeCode(buf, lcd);
        }
        if(wasPressed(DIGIT_3_BTN)) {
            buf[2]++;
            if(buf[2] > '9') {
                buf[2] = '0';
            }
            writeCode(buf, lcd);
        }

        // Handle switching fields
        if(wasPressed(ENTER_BTN)) {
            // Increment state (clamped to 0-2)
            state++;
            state %= 3;
            buf = resolveCurrentField(state, &config);

            writeName(names[state], lcd);
            writeCode(buf, lcd);
        }

        if(wasPressed(ACTION_BTN)) {
            finished = true;
        }
    }

    return config;
}


void writeCode(char *code, LiquidCrystal lcd) {
    lcd.setCursor(0,1);

    // Copy 3 significant digit bytes of the code into a null-terminated string
    char writeBuffer[4] = "000";
    memcpy(writeBuffer, code, 3);

    lcd.write(writeBuffer);
    lcd.setCursor(0,0);
}


char *resolveCurrentField(int state, RaceConfig *config) {
    char *field = NULL;
    switch(state) {
        case 0:
            field = config->boatNum;
            break;
        case 1:
            field = config->courseNum;
            break;
        case 2:
            field = config->nomSpeed;
            break;
        default:
            break;
    }

    return field;
}

void writeName(char *name, LiquidCrystal lcd) {
    lcd.clear();
    lcd.setCursor(0,0);
    lcd.print(name);
}


