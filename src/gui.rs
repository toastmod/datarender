extern crate web_view;

use lazy_static;
use ringbuf::RingBuffer;
use std::{thread, process};
use std::time::Duration;
use tfd::MessageBoxIcon;
use web_view::*;
use crate::*;
use std::sync::atomic::Ordering;

fn kill_proc(webview: &mut WebView<()>){
    println!("ending process...");
    webview.exit();
    process::exit(0x001);
}

fn trigger_loadfile(){
    let tmp = match tfd::open_file_dialog("Please choose a file...", "", None) {
        Some(path) => tfd::message_box_ok("File chosen", &path, MessageBoxIcon::Info),
        None => tfd::message_box_ok(
            "Warning",
            "You didn't choose a file.",
            MessageBoxIcon::Warning,
        ),
    };
}

fn gettuple(input: Vec<&str>) -> (usize, i64) {
    // outputs backwards for some reason
    // this will flip them back around
    // 0 = index => 1
    // 1 = value => 0
    (
        input[1].parse::<usize>().unwrap(),
        input[0].parse::<i64>().unwrap(),
    )
}

pub fn startgui(mut prod: ringbuf::Producer<[i64; 9]>) -> WVResult {
    let mut values: [i64;9] = [0i64;9];
    // 012 = rot
    // 345 = pos

    let mut webview = web_view::builder()
        .title("datarender | controls")
        .content(Content::Html(HTML))
        .size(400, 600)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {

            //check if it was a button
            match arg {
                "open" => trigger_loadfile(),
                "exit" => kill_proc(webview),
                _ => (),
            };

            //do the thing
            let split: Vec<&str> = arg.rsplitn(2, '|').collect();
            let tmp = gettuple(split);
            //println!("{}",tmp.1);
            match tmp.0 {
                0=>rotx.store(tmp.1,Ordering::SeqCst),
                1=>roty.store(tmp.1,Ordering::SeqCst),
                2=>rotz.store(tmp.1,Ordering::SeqCst),

                3=>posx.store(tmp.1,Ordering::SeqCst),
                4=>posy.store(tmp.1,Ordering::SeqCst),
                5=>posz.store(tmp.1,Ordering::SeqCst),

                6=>buf.store(tmp.1,Ordering::SeqCst),
                7=>ind.store(tmp.1,Ordering::SeqCst),
                8=>fov.store(tmp.1,Ordering::SeqCst),
                _ => {}
            }

            //update(values);
            upd8.store(true, Ordering::SeqCst);
            Ok(())
        })
        .build()?;

    let res = webview.run();
    exit_code.store(true, Ordering::Relaxed);
    res
}

//night mode
//color: white; background-color: rgb(0,0,0);

const HTML: &str = r#"
<!doctype html>
<html>
<head>
<link href="https://fonts.googleapis.com/css2?family=Inconsolata:wght@300&display=swap" rel="stylesheet">
<style>
body {
  -webkit-touch-callout: none; /* iOS Safari */
    -webkit-user-select: none; /* Safari */
     -khtml-user-select: none; /* Konqueror HTML */
       -moz-user-select: none; /* Old versions of Firefox */
        -ms-user-select: none; /* Internet Explorer/Edge */
            user-select: none; /* Non-prefixed version, currently
                                  supported by Chrome, Opera and Firefox */
}
</style>
</head>
    <body style='margin-top: 10px; margin-bottom: 10px; overflow: visible; font-family: Inconsolata; '>
        <button onclick="external.invoke('open')">Open</button>
        <button onclick="external.invoke('exit')">Exit</button>
        <hr></hr>
        FOV
        <div></div>
        F <input oninput="spill(8,'fov')" type="range" min="0" max="360" value="0" class="slider" id="fov">
        <hr></hr>
        verticies to render (*100)
        <div></div>
        V <input oninput="spill(6,'bufsize')" type="range" min="1" max="5" value="3" class="slider" id="bufsize">
        <hr></hr>
        index
        <div></div>
        I <input oninput="spill(7,'index')" type="range" min="1" max="5000" value="3" class="slider" id="index">
        <hr></hr>
        rotate
        <div></div>
        X <input oninput="spill(0,'Xrot')" type="range" min="-3600" max="3600" value="0" class="slider" id="Xrot">
        <br></br>
        Y <input oninput="spill(1,'Yrot')" type="range" min="-3600" max="3600" value="0" class="slider" id="Yrot">
        <br></br>
        Z <input oninput="spill(2,'Zrot')" type="range" min="-3600" max="3600" value="0" class="slider" id="Zrot">
        <hr></hr>
        translate
        <div></div>
        X <input oninput="spill(3,'Xpos')" type="range" min="-10000" max="10000" value="0" class="slider" id="Xpos">
        <br></br>
        Y <input oninput="spill(4,'Ypos')" type="range" min="-10000" max="10000" value="0" class="slider" id="Ypos">
        <br></br>
        Z <input oninput="spill(5,'Zpos')" type="range" min="-10000" max="10000" value="0" class="slider" id="Zpos">

        <script type='text/javascript'>

        function spill(ind,id){
            external.invoke(ind+'|'+document.getElementById(id).value);
        }

        </script>
    </body>
</html>
"#;
