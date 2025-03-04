use godot::{
    classes::{ISprite3D, InputEvent, Sprite3D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Sprite3D)]
pub struct Card {
    mouse_input_received: bool,
    mouse_over: bool,

    base: Base<Sprite3D>,
}

#[godot_api]
impl ISprite3D for Card {
    fn init(base: Base<Sprite3D>) -> Self {
        Self {
            base,
            mouse_over: false,
            mouse_input_received: false,
        }
    }
}

#[godot_api]
impl Card {
    #[signal]
    fn input_event(
        camera: Gd<Node>,
        event: Gd<InputEvent>,
        input_position: Vector3,
        normal: Vector3,
    );

    #[signal]
    fn mouse_entered();

    #[signal]
    fn mouse_exited();

    #[func]
    fn on_3d_mouse_ray_processed(&mut self) {
        if self.mouse_input_received {
            if !self.mouse_over {
                self.mouse_over = true;
                self.base_mut().emit_signal("mouse_entered", &[]);
            }
        } else if self.mouse_over {
            self.mouse_over = false;
            self.base_mut().emit_signal("mouse_exited", &[]);
        }
        self.mouse_input_received = false;
    }

    /// This is a function
    #[func]
    fn on_mouse_entered(&mut self) {
        self.base_mut().set_modulate(Color::RED);
    }

    #[func]
    fn on_mouse_exited(&mut self) {
        self.base_mut().set_modulate(Color::WHITE);
    }

    pub fn try_mouse_input(
        &mut self,
        camera: Gd<Node>,
        event: Gd<InputEvent>,
        input_position: Vector3,
        normal: Vector3,
    ) -> bool {
        self.mouse_input_received = true;
        self.base_mut().emit_signal(
            "input_event",
            &[
                Variant::from(camera),
                Variant::from(event),
                Variant::from(input_position),
                Variant::from(normal),
            ],
        );
        true
    }
}
