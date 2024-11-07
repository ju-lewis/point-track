


int wasPressed(int btn) {
    // Halt execution until button is released
    if(!digitalRead(btn)) {
        while(!digitalRead(btn)) {}
        // 20 millisecond buffer
        delay(100);
        return 1;
    }
    return 0;
}
