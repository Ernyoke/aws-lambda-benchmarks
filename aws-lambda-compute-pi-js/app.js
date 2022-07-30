function* generateDigitsOfPi() {
    let q = 1n;
    let r = 180n;
    let t = 60n;
    let i = 2n;
    while (true) {
        let digit = ((i * 27n - 12n) * q + r * 5n) / (t * 5n);
        yield Number(digit);
        let u = i * 3n;
        u = (u + 1n) * 3n * (u + 2n);
        r = u * 10n * (q * (i * 5n - 2n) + r - t * digit);
        q *= 10n * i * (i++ * 2n - 1n);
        t *= u;
    }
}


exports.handler = async (event) => {
    const iter = generateDigitsOfPi();
    const digits = [];
    for (let i = 0; i < event.digits; i++) {
        digits.push(iter.next().value);
    }
    return {
        digits: event.digits,
        pi: digits.join('')
    };
};
