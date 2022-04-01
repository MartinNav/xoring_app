use std::borrow::Borrow;

use fltk::{app, prelude::*, window::Window, frame::Frame, input::Input, button::Button};
use hex;
fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 1400, 300, "xoring app");
    let mut intpt_box = Input::new(180, 20, 800, 30, "Encrypted Text(hex):");
    let mut key = Input::new(180,60,350,30,"Key(hex):");
    let mut btn_enter =Button::new(500,90,90,30,"Decrypt");
    let mut result = Frame::new(120,180,1400,30,"decrypted data");
    wind.end();
    wind.show();
    btn_enter.set_callback(move |_|{
        let mut txt=hex::decode(intpt_box.value()).expect("unable to decode cipher");
        let mut k = hex::decode(key.value()).expect("unable to decode key");
        let mut o_vec =Vec::new();
        for i in 0..txt.len() {
            o_vec.push(txt[i]^k[i%k.len()]);
        }

        let output_string =String::from_utf8_lossy(&o_vec).to_string() ;
        result.set_label(&output_string);
    });
    app.run().unwrap();

    

}
