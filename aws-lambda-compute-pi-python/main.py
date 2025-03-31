def generate_digits_of_pi():
    q = 1
    r = 180
    t = 60
    i = 2
    while True:
        digit = ((i * 27 - 12) * q + r * 5) // (t * 5)
        yield digit
        u = i * 3
        u = (u + 1) * 3 * (u + 2)
        r = u * 10 * (q * (i * 5 - 2) + r - t * digit)
        q *= 10 * i * (i * 2 - 1)
        t *= u
        i += 1

def handler(event, context):
    iter_pi = generate_digits_of_pi()
    digits = [next(iter_pi) for _ in range(event["digits"])]
    pi = "".join(map(str, digits))
    print(f"PI: {pi}")
    return None