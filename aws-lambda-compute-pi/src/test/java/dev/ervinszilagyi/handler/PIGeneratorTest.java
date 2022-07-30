package dev.ervinszilagyi.handler;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class PIGeneratorTest {

    @Test
    void getDigit() {
        PIGenerator piGenerator = new PIGenerator();
        for (int i = 0; i < 10_000; i++) {
            System.out.println(piGenerator.getDigit());
        }
    }
}
