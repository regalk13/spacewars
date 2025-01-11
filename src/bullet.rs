use bevy::prelude::*;
use crate::rocket::Rocket;
use bevy_hanabi::prelude::*;

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
}

pub fn spawn_bullet(
    time: Res<Time>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Transform, &mut Rocket)>,
) {
    let bullet_sprite: Handle<Image> = asset_server.load("Bullet.png");

    for (transform, mut rocket) in query.iter_mut() {
        let current_time = time.elapsed();

        if keys.just_pressed(rocket.spawn_key)
           && current_time - rocket.last_shot_time >= rocket.cooldown
        {
            let direction = transform.rotation.mul_vec3(Vec3::Y); // Rocket's forward direction
            let bullet_spawn_pos = transform.translation + direction * 50.0; // Padding of 50 units

            commands.spawn((
                SpriteBundle {
                    texture: bullet_sprite.clone(),
                    transform: Transform {
                        translation: bullet_spawn_pos,
                        scale: Vec3::new(2., 2., 1.0),
                        rotation: transform.rotation,
                        ..default()
                    },
                    ..default()
                },
                Bullet {
                    speed: 300.0, // Example bullet speed
                },
            ));

            // Update the last shot time
            rocket.last_shot_time = current_time;
        }
    }
}


pub fn handle_bullet_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Bullet)>,
) {
    for (entity, mut transform, bullet) in query.iter_mut() {
        let forward = transform.rotation.mul_vec3(Vec3::Y).normalize();
        transform.translation += forward * bullet.speed * time.delta_seconds();
        if transform.translation.y > 510.0 || transform.translation.y < -380.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn check_bullet_coll(
    mut commands: Commands,
    rocket_query: Query<(Entity, &Transform), (With<Rocket>, Without<EffectProperties>)>,
    bullet_query: Query<&Transform, (With<Bullet>, Without<EffectProperties>)>,
    mut effect: Query<(
        &mut EffectProperties,
        &mut EffectInitializers,
        &mut Transform,
    )>,
) {
    let Ok((mut properties, mut initializers, mut effect_transform)) = effect.get_single_mut()
    else {
        return;
    };

    for (entity, enemy_transform) in rocket_query.iter() {
        for bullet_transform in bullet_query.iter() {            
            let distance = enemy_transform
            .translation
            .truncate()
            .distance(bullet_transform.translation.truncate());
            if distance < 30.
            {
                effect_transform.translation = enemy_transform.translation;

                let r = 255.;
                let g = 165.;
                let b = 0.;
                let color = 0xFF000000u32 | (b as u32) << 16 | (g as u32) << 8 | (r as u32);
                properties.set("spawn_color", color.into());
                initializers.reset();
    
                commands.entity(entity).despawn();
            }
        }
    }
}