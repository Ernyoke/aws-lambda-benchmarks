use neon::prelude::*;
use num_bigint::{BigInt, ToBigInt};
use num_traits::cast::ToPrimitive;
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generate", generate)?;
    Ok(())
}

fn runtime<'a, C: Context<'a>>(cx: &mut C) -> NeonResult<&'static Runtime> {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();
    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| cx.throw_error(err.to_string())))
}

fn generate(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let js_limit = cx.argument::<JsNumber>(0)?;
    let limit = js_limit.value(&mut cx);

    let rt = runtime(&mut cx)?;
    let (deferred, promise) = cx.promise();
    let channel = cx.channel();

    rt.spawn(async move {
        let digits = generate_pi(limit.to_i64().unwrap());
        deferred.settle_with(&channel, move |mut cx| {
            let res: Handle<JsArray> = JsArray::new(&mut cx, digits.len() as u32);
            for (i, &digit) in digits.iter().enumerate() {
                let val = cx.number(f64::from(digit));
                res.set(&mut cx, i as u32, val);
            }
            Ok(res)
        });
    });
    Ok(promise)
}

fn generate_pi(limit: i64) -> Vec<i32> {
    let mut q = 1.to_bigint().unwrap();
    let mut r = 180.to_bigint().unwrap();
    let mut t = 60.to_bigint().unwrap();
    let mut i = 2.to_bigint().unwrap();
    let mut res: Vec<i32> = Vec::new();
    for _ in 0..limit {
        let digit: BigInt = ((&i * 27 - 12) * &q + &r * 5) / (&t * 5);
        res.push(digit.to_i32().unwrap());
        let mut u: BigInt = &i * 3;
        u = (&u + 1) * 3 * (&u + 2);
        r = &u * 10 * (&q * (&i * 5 - 2) + r - &t * digit);
        q *= 10 * &i * (&i * 2 - 1);
        i = i + 1;
        t *= u;
    }
    res
}
