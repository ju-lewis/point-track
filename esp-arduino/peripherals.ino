


int wasPressed(int btn) {
    // Halt execution until button is released
    if(!digitalRead(btn)) {
        while(!digitalRead(btn)) {}
        // Short time buffer to prevent accidental double-pressing
        delay(150);
        return 1;
    }
    return 0;
}
