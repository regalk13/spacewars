use bevy::prelude::*;
use crate::rocket::{self, Rocket};

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
    pub radius_collision: f32,
}

pub fn spawn_bullet(mut commands: Commands, keys: Res<ButtonInput<KeyCode>>, asset_server: Res<AssetServer>, mut query: Query<&mut Transform, With<Rocket>>) {
       let bullet_sprite: Handle<Image> = asset_server.load("Bullet.png");

       for transform in query.iter_mut(){

        let pos_x = transform.translation.x;
        let pos_y = transform.translation.y;

        if keys.pressed(KeyCode::Space){
            commands.spawn((
                SpriteBundle {
                    texture: bullet_sprite.clone(),
                    transform: Transform {
                        translation: Vec3::new(pos_x, pos_y + 25., 0.0),
                        scale: Vec3::new(1., 1., 1.),
                        ..default()
                    },
                    ..default()
                },
                Bullet {
                    speed: 0.0,
                    radius_collision: 50.
                },
            ));
        }
    }
}


pub fn handle_bullet_movement(

) {

}

pub fn clip_bullets(

) {
    
}


pub fn check_bullet_coll(
    mut commands: Commands,
    rocket_query: Query<(Entity, &Transform), With<Rocket>>,
    bullet_query: Query<&Transform, With<Bullet>>
) {
    for (entity, enemy_transform) in rocket_query.iter() {
        for bullet_transform in bullet_query.iter() {            
            let distance = enemy_transform
            .translation
            .truncate()
            .distance(bullet_transform.translation.truncate());
            if distance < 150.
            {
                commands.entity(entity).despawn();
            }
        }
    }
}