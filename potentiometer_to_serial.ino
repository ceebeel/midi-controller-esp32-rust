const int NUM_SAMPLES = 32;
const int NUM_POTS = 5;
int values[NUM_POTS] = {0};
int last_values[NUM_POTS] = {0};
const int pot_pins[NUM_POTS] = {36, 39, 32, 33, 34};

void setup()
{
    Serial.begin(115200);
}

void send_midi_cc(byte channel, byte cc, byte value)
{
    Serial.write(0xB0 | (channel & 0x0F)); // 0xB0 to 0xBF for CC on channels 1-16
    Serial.write(cc & 0x7F);               // CC number (0-127)
    Serial.write(value & 0x7F);            // Value (0-127)
}

void loop()
{
    for (int pot = 0; pot < NUM_POTS; pot++)
    {
        long sum = 0;
        for (int i = 0; i < NUM_SAMPLES; i++)
        {
            sum += analogRead(pot_pins[pot]);
        }
        values[pot] = sum / NUM_SAMPLES;
        values[pot] = map(values[pot], 0, 4095, 0, 127); // Map to MIDI range 0-127

        if (values[pot] != last_values[pot])
        {
            last_values[pot] = values[pot];
            // Send MIDI CC message
            send_midi_cc(0, pot, values[pot]); // MIDI channel 1 (0), CC0 to CC4, value 0-127
        }
    }
}
