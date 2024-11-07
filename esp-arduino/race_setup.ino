
#include "race_setup.h"
#include "peripherals.h"


void writeCode(LiquidCrystal lcd);
char *resolveCurrentField(int state, RaceConfig *config);


RaceConfig enterConfig(LiquidCrystal lcd) {
    bool finished = false;
    int state = 0;
    char *names[3] = {"Boat Number", "Course Number", "Nominated Speed"};
    RaceConfig config;

    // Current value buffer
    char buf[3] = {'0','0','0'};

    // Init screen
    lcd.clear();
    lcd.setCursor(0,0);


    while(!finished) {
        
        if(wasPressed(DIGIT_1_BTN)) {
            buf[0]++;
            if(buf[0] > '9') {
                buf[0] = '0';
            }
        }
        if(wasPressed(DIGIT_2_BTN)) {
            buf[1]++;
            if(buf[1] > '9') {
                buf[1] = '0';
            }
        }
        if(wasPressed(DIGIT_3_BTN)) {
            buf[2]++;
            if(buf[2] > '9') {
                buf[2] = '0';
            }
        }

        
        
        if(wasPressed(ENTER_BTN)) {
            // Increment state (clamped to 0-2)
            state++;
            state %= 3;
        }
    }

    return config;
}


void writeCode(char* code, LiquidCrystal lcd) {
    lcd.setCursor(0,1);
    lcd.print(code);
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




