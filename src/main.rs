use bevy::{prelude::{App, ResMut, Local, EventWriter, EventReader, Res}, app::AppExit};


pub struct Indicator(pub char);

pub fn render_random(
    mut cnt: Local<usize>,
    mut exit: EventWriter<AppExit>,
    indicator: Res<Indicator>,
    mut display_buffer: ResMut<monadic::render::DisplayBuffer>,
) {
    // println!("{:?}", cnt);
    // if *cnt == 2 {
    //     exit.send_default();
    //     return;
    // }
    // *cnt += 1;
    
    display_buffer.clear();
    let monadic::render::Rect{h, w} = display_buffer.shape;
    let mut pos = monadic::render::Point(rand::random::<u32>() % h,
                                            rand::random::<u32>() % w);
    pos.0 = h.saturating_sub(5);
    display_buffer.set_single(pos, indicator.0);
}

pub fn exit_signal(
    mut indicator: ResMut<Indicator>,
    mut keyboard_input: EventReader<monadic::input::KeyEvent>,
    mut exit: EventWriter<AppExit>,
) {
    for kev in keyboard_input.iter() {
        println!("event read: {:?}", kev.code);
        if let crossterm::event::KeyCode::Char(ch) = kev.code {
            indicator.0 = ch;
            if ch == 'q' {
                exit.send_default();
            }
        }
    }
}

fn main() {
    let mut app = App::new();

    app
        .insert_resource(monadic::render::DisplayConfig {
            h: 20,
            w: 40,
        })
        .insert_resource(Indicator('@'))
        .add_plugins(bevy::MinimalPlugins)
        .add_plugin(monadic::render::RenderPlugin)
        .add_plugin(monadic::input::InputPlugin)
        .add_system(render_random)
        .add_system(exit_signal)
        .run()
        ;

    // const WAIT_FOR_INPUT_MS: u64 = 5;
    // let mut stdout = std::io::stdout();
    // loop {
    //     if crossterm::event::poll(std::time::Duration::from_millis(WAIT_FOR_INPUT_MS)).unwrap_or(false) {
    //         println!("polled");
            
    //         let result = crossterm::queue!(stdout, 
    //             crossterm::terminal::Clear(crossterm::terminal::ClearType::All),        
    //             crossterm::terminal::Clear(crossterm::terminal::ClearType::Purge),
    //         );
    //         let flush_result = std::io::Write::flush(&mut stdout);
            
    //         match crossterm::event::read() {
    //             Ok(event) => {
    //                 println!("{:?}", event);
    //                 match event {
    //                     crossterm::event::Event::Key(k) => {
    //                         match k.code {
    //                             crossterm::event::KeyCode::Char('q') => break,
    //                             _ => {},
    //                         };
    //                     },
    //                     // Event::Mouse(m) => mouse_ew.send(MouseEvent {
    //                     //     kind: m.kind,
    //                     //     column: m.column,
    //                     //     row: m.row,
    //                     //     modifiers: m.modifiers,
    //                     // }),
    //                     // Event::Resize(h, w) => resize_ew.send(ResizeEvent { h, w }),
    //                     _ => {}
    //                 }
    //             },
    //             Err(_) => {
    //                 // TODO: Error handling
    //                 println!("input read error")
    //             },
    //         }
    //     }
    // }
}