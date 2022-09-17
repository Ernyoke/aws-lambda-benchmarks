package dev.ervinszilagyi.handler;

import java.math.BigInteger;

public class PIGenerator {
    private BigInteger q = BigInteger.ONE;
    private BigInteger r = BigInteger.valueOf(180);
    private BigInteger t = BigInteger.valueOf(60);
    private BigInteger i = BigInteger.TWO;

    public int getDigit() {
        BigInteger digit = i.multiply(BigInteger.valueOf(27))
                .subtract(BigInteger.valueOf(12))
                .multiply(q)
                .add(r.multiply(BigInteger.valueOf(5)))
                .divide(t.multiply(BigInteger.valueOf(5)));
        BigInteger u = i.multiply(BigInteger.valueOf(3));
        u = u.add(BigInteger.ONE)
                .multiply(BigInteger.valueOf(3))
                .multiply(u.add(BigInteger.valueOf(2)));
        r = u.multiply(BigInteger.TEN)
                .multiply(q.multiply(i.multiply(BigInteger.valueOf(5)).subtract(BigInteger.TWO)).add(r).subtract(t.multiply(digit)));
        q = q.multiply(BigInteger.TEN.multiply(i).multiply(i.multiply(BigInteger.TWO).subtract(BigInteger.ONE)));
        i = i.add(BigInteger.ONE);
        t = t.multiply(u);
        return digit.intValue();
    }
}


