const int numSamples = 32;
const int numPots = 5;

int values[numPots] = {0};
int last_values[numPots] = {0};

// GPIO pins: {36, 39, 32, 33, 34, 35, 4, 0, 2, 15, 13, 12, 14, 27, 25};
const int potPins[numPots] = {36, 39, 32, 33, 34};

void setup()
{
    Serial.begin(115200);
}

void loop()
{
    for (int pot = 0; pot < numPots; pot++)
    {
        long sum = 0;
        for (int i = 0; i < numSamples; i++)
        {
            sum += analogRead(potPins[pot]);
            // delayMicroseconds(100);
        }
        values[pot] = sum / numSamples;
        values[pot] = map(values[pot], 0, 4095, 0, 255);

        if (values[pot] != last_values[pot])
        {
            last_values[pot] = values[pot];
            Serial.print("0:0,255:255,Pot"); // Plotter format
            Serial.print(pot + 1);           // Plotter format
            Serial.print(":");               // Plotter format
            Serial.println(values[pot]);
        }
    }
}
