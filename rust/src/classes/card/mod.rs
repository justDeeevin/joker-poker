use godot::{
    classes::{ISprite3D, InputEvent, InputEventMouseButton, Sprite3D},
    global::MouseButton,
    prelude::*,
};

use super::camera::Camera;

#[derive(GodotClass)]
#[class(init, base=Sprite3D)]
pub struct Card {
    mouse_input_received: bool,
    mouse_over: bool,
    held: bool,
    previous_position: Vector3,

    base: Base<Sprite3D>,
}

const PUSH_SCALE: f32 = 0.15;
const HOVER_DRAW: f32 = 0.1;
const DRAG_SCALE: f32 = 1.2;

#[godot_api]
impl ISprite3D for Card {
    fn ready(&mut self) {
        let mut camera = self
            .base()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .get_node_as::<Camera>("Root/Camera");
        camera.connect(
            "mouse_ray_processed",
            &self.base().callable("on_3d_mouse_ray_processed"),
        );
        let on_input_event = self.base().callable("on_input_event");
        self.base_mut().connect("input_event", &on_input_event);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(event) = event.clone().try_cast::<InputEventMouseButton>() {
            if event.get_button_index() == MouseButton::LEFT {
                if event.is_pressed() && self.mouse_over {
                    self.held = true;
                    self.base_mut().set_rotation(Vector3::ZERO);
                } else if event.is_released() {
                    self.held = false;
                }
            }
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

    #[func]
    fn on_input_event(
        &mut self,
        _camera: Gd<Node>,
        _event: Gd<InputEvent>,
        input_position: Vector3,
        _normal: Vector3,
    ) {
        let relative_position = input_position - self.base().get_position();
        if self.held {
            let delta = input_position - self.previous_position;
            self.base_mut()
                .translate(Vector3::new(delta.x, delta.y, 0.0));
            self.base_mut()
                .set_rotation(Vector3::new(0.0, 0.0, -delta.x * DRAG_SCALE));
        } else {
            self.base_mut().set_rotation(
                Vector3::new(-relative_position.y, relative_position.x, 0.0) * PUSH_SCALE,
            );
        }
        self.previous_position = input_position;
    }

    #[func]
    fn on_3d_mouse_ray_processed(&mut self) {
        if self.mouse_input_received {
            if !self.mouse_over {
                self.mouse_over = true;
                self.on_mouse_entered();
            }
        } else if self.mouse_over {
            self.mouse_over = false;
            self.on_mouse_exited();
        }
        self.mouse_input_received = false;
    }

    #[func]
    fn on_mouse_exited(&mut self) {
        self.base_mut()
            .translate(Vector3::new(0.0, 0.0, -HOVER_DRAW));
        self.base_mut().set_rotation(Vector3::ZERO);
    }

    #[func]
    fn on_mouse_entered(&mut self) {
        self.base_mut()
            .translate(Vector3::new(0.0, 0.0, HOVER_DRAW));
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
