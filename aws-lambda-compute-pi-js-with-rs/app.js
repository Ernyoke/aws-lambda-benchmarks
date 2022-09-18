const compute_pi = require("compute-pi-rs");

exports.handler = async (event) => {
    const digits = await compute_pi.generate(event.digits);
    return {
        digits: event.digits,
        pi: digits.join('')
    };
};

async function run(x, it) {
    console.log(`start ${x}`);
    const res = await compute_pi.generate(it);
    console.log(`end ${x}`);
    return res;
}

