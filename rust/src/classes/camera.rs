use super::Card;
use godot::{
    classes::{Area3D, InputEvent, InputEventMouse, PhysicsRayQueryParameters3D},
    prelude::*,
};

const RAY_LENGTH: f32 = 1000.0;

#[derive(GodotClass)]
#[class(init, base=Camera3D)]
pub struct Camera {
    query_mouse: bool,
    mouse_event: Option<Gd<InputEventMouse>>,
    #[export(flags_3d_physics)]
    /// The physics layers for raycasting
    sprite_layers: u32,

    base: Base<Camera3D>,
}

#[godot_api]
impl ICamera3D for Camera {
    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if let Ok(event) = event.try_cast::<InputEventMouse>() {
            self.query_mouse = true;
            self.mouse_event = Some(event);
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        if self.query_mouse {
            self.check_sprite_input();
            self.query_mouse = false;
            self.base_mut().emit_signal("mouse_ray_processed", &[]);
        }
    }
}

#[godot_api]
impl Camera {
    #[signal]
    fn mouse_ray_processed();

    fn check_sprite_input(&self) -> bool {
        let mut not_hits = Array::new();
        let mouse_event = self.mouse_event.clone().unwrap();
        let mouse_position = mouse_event.get_position();

        let mut space_state = self
            .base()
            .get_world_3d()
            .unwrap()
            .get_direct_space_state()
            .unwrap();
        let from = self.base().project_ray_origin(mouse_position);
        let to = from + (self.base().project_ray_normal(mouse_position) * RAY_LENGTH);

        loop {
            let mut query = PhysicsRayQueryParameters3D::create_ex(from, to)
                .collision_mask(self.sprite_layers)
                .exclude(&not_hits)
                .done()
                .unwrap();
            query.set_collide_with_areas(true);
            let result = space_state.intersect_ray(&query);

            if result.is_empty() {
                return false;
            } else if result
                .get("collider")
                .unwrap()
                .to::<Gd<Node>>()
                .cast::<Area3D>()
                .get_owner()
                .unwrap()
                .cast::<Card>()
                .bind_mut()
                .try_mouse_input(
                    self.to_gd().upcast(),
                    mouse_event.clone().upcast(),
                    result.get("position").unwrap().to(),
                    result.get("normal").unwrap().to(),
                )
            {
                return true;
            } else {
                not_hits.push(result.get("collider").unwrap().to::<Rid>());
            }
        }
    }
}
