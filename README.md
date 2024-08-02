# bevy_ecs for miniquad/macroquad without wasm-bindgen

### Why ?
IDK, less bloat & faster compiles ?

---
### How it works ?
By patching bevy crates to remove functionality or chage implementation.
as of 0.14.0 , the changes include:
* removing all multi-threaded related functionality of `bevy_ecs`
* removing schedule_runner and panic_handler plugins of `bevy_app`
* removing `bevy_time` usage in `bevy_app`
* modifying `bevy_utils` to use `miniquad::date::now()` instead of `web-time`

---
### How can I use it ?
By adding the following lines to `.cargo/config.toml`
```toml
[patch.crates-io]
bevy_ecs = { git = "https://github.com/yui-915/quad_bevy", branch = "bevy_ecs@0.14.0" }
bevy_app = { git = "https://github.com/yui-915/quad_bevy", branch = "bevy_app@0.14.0" }
bevy_utils = { git = "https://github.com/yui-915/quad_bevy", branch = "bevy_utils@0.14.0" }
```

---
### Example
<details><summary>Click to show</summary>

You can build it for web just like any other macroquad project and it'll work without any extra setup
```rust
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use macroquad::prelude::*;

fn main() {
    App::new()
        .add_plugins(MacroquadRunner("Hello, world!"))
        .add_systems(Startup, spawn_squares)
        .add_systems(Update, (move_squares, bg, draw_squares).chain())
        .run();
}

#[derive(Component)]
struct Position(f32, f32);

fn spawn_squares(mut cmds: Commands) {
    cmds.spawn(Position(0.0, 100.0));
    cmds.spawn(Position(450.0, 500.0));
}

fn bg() {
    clear_background(ORANGE);
}

fn move_squares(mut query: Query<&mut Position>) {
    for mut position in &mut query {
        position.0 += 10.0;
        if position.0 > screen_width() {
            position.0 = 0.0;
        }
    }
}

fn draw_squares(query: Query<&Position>) {
    for position in &query {
        draw_rectangle(position.0, position.1, 100.0, 100.0, BLACK);
    }
}

// Example bevy runner plugin for macroquad
pub struct MacroquadRunner(pub &'static str);
impl Plugin for MacroquadRunner {
    fn build(&self, app: &mut App) {
        let window_title = self.0;
        app.set_runner(|mut app| {
            macroquad::Window::new(window_title, async move {
                loop {
                    app.update();
                    next_frame().await;
                }
            });
            bevy_app::AppExit::Success
        });
    }
}
```
</details>

---
### Should I use it ?
Probably not, this project is highly experimental, and might break your code due to incompatible changes or different miniquad version

---
### Why is this a crate ?
Just so it shows up on crates.io, otherwise it does nothing.
you can used the patched bevy crates without download this crate.

---
### Licence
I'll keep the "MIT or Apache 2.0" licence from the original bevy crates
