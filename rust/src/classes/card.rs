use godot::{
    classes::{ISprite3D, InputEvent, Sprite3D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Sprite3D)]
pub struct Card {
    mouse_input_received: bool,
    mouse_over: bool,
    reset_timer: Option<f64>,

    base: Base<Sprite3D>,
}

const PUSH_SCALE: f32 = 0.15;
const HOVER_DRAW: f32 = 0.1;
const RESET_LERP_SCALE: f64 = 4.0;

#[godot_api]
impl ISprite3D for Card {
    fn init(base: Base<Sprite3D>) -> Self {
        Self {
            base,
            mouse_over: false,
            mouse_input_received: false,
            reset_timer: None,
        }
    }

    fn process(&mut self, delta: f64) {
        let Some(reset_timer) = self.reset_timer.as_mut() else {
            return;
        };
        *reset_timer += delta * RESET_LERP_SCALE;
        let reset_timer = *reset_timer as f32;
        let rotation = self.base().get_rotation();
        self.base_mut()
            .set_rotation(rotation.lerp(Vector3::ZERO, reset_timer));
        if self.base().get_rotation() == Vector3::ZERO {
            self.reset_timer = None;
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
    fn on_input_event(
        &mut self,
        _camera: Gd<Node>,
        _event: Gd<InputEvent>,
        input_position: Vector3,
        _normal: Vector3,
    ) {
        self.base_mut()
            .set_rotation(Vector3::new(-input_position.y, input_position.x, 0.0) * PUSH_SCALE);
    }

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

    #[func]
    fn on_mouse_exited(&mut self) {
        self.base_mut()
            .translate(Vector3::new(0.0, 0.0, -HOVER_DRAW));
        self.reset_timer = Some(0.0);
    }

    #[func]
    fn on_mouse_entered(&mut self) {
        self.base_mut()
            .translate(Vector3::new(0.0, 0.0, HOVER_DRAW));
        self.reset_timer = None;
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
