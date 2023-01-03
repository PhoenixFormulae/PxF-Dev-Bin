// Standard Uses

// Crate Uses

// External Uses
use wry::{
    application::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};


pub fn create() -> wry::Result<()> {

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_transparent(true)
        .build(&event_loop)
        .unwrap();

    let _webview = WebViewBuilder::new(window)
        .unwrap()
        .with_transparent(true)
        .with_url(create_root().as_str())?
        .with_file_drop_handler(|_, data| {
            println!("Window 1: {:?}", data);
            false // Returning true will block the OS default behaviour.
        })
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });

}


pub fn create_root() -> String {
    let body = r#"
        <div>

        </div>
    "#;

    let scripts = r#"
    <script>
        window.onload = function() {
            document.body.innerText = `hello, ${navigator.userAgent}`;
        };
    </script>
    "#;

    let root = format!(r#"
    data:text/html,
    <!doctype html>
    <html>
        <body style="background-color:rgba(87,87,87,0.5);">
            {}
        </body>
        {}
    </html>"#, body, scripts);

    root
}

