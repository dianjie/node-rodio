use neon::prelude::*;
use std::io::Cursor;
use std::thread;

pub fn player_play(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let full_url = cx.argument::<JsString>(0)?.value(&mut cx);
    let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
    let channel = cx.channel();

    thread::spawn(move || {
        let resp = reqwest::blocking::get(full_url).unwrap();
        let cursor = Cursor::new(resp.bytes().unwrap());
        let source = rodio::Decoder::new(cursor).unwrap();
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        sink.play();
        sink.sleep_until_end();
        channel.send(move |mut cx| {
            let callback = callback.into_inner(&mut cx);
            let this = cx.undefined();
            let args = vec![
                cx.number(1).upcast(),
            ];
            callback.call(&mut cx, this, args)?;
            Ok(())
        });
    });
    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("playerPlay", player_play)?;

    Ok(())
}