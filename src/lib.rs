use neon::prelude::*;
use std::io::Cursor;
use std::thread;

pub fn player_play(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let full_url = cx.argument::<JsString>(0)?.value(&mut cx);

    thread::spawn(move || {
        let resp = reqwest::blocking::get(full_url).unwrap();
        let cursor = Cursor::new(resp.bytes().unwrap());
        let source = rodio::Decoder::new(cursor).unwrap();
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        sink.play();
        sink.sleep_until_end();
    });
    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("playerPlay", player_play)?;

    Ok(())
}