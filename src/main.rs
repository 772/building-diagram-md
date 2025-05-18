use bevy::prelude::*;

#[derive(Component)]
struct Object(char);

pub fn get_letters_in_ascii_grid(
    image: Vec<&str>,
    x: usize,
    y: usize,
    wdt: usize,
    hgt: usize,
) -> Vec<(usize, usize, char)> {
    const IGNORED_CHARS: [char; 4] = ['+', '-', ' ', '|'];

    image[y..y + hgt]
        .iter()
        .map(|row| &row[x..x + wdt])
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, ch)| !IGNORED_CHARS.contains(ch))
                .map(move |(x, ch)| (x, y, ch))
        })
        .collect()
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, setup)
        .add_plugins(bevy_panorbit_camera::PanOrbitCameraPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn objects.
    let scaling = 1.0 / 18.0;
    let filename_markdown = "input.md";
    let mut room_index = 0.0;
    for section in std::fs::read_to_string(filename_markdown)
        .expect("Konnte input.md nicht einlesen")
        .split('#')
        .filter(|s| !s.trim().is_empty())
    {
        let mut lines = section.trim().lines();
        let first_line = lines.next().unwrap_or("").trim();
        let name = first_line.to_string();
        let content = lines.collect::<Vec<&str>>().join("\n").trim().to_string();
        if name.len() == 1 {
            let _name = name.chars().next().unwrap();
            let _desc = content;
        } else {
            let lines: Vec<&str> = content.trim().lines().collect();
            let first_line = lines[0].trim();
            let width: usize = first_line.len();
            let depth: usize = 1 + lines
                .clone()
                .into_iter()
                .enumerate()
                .skip(1)
                .find_map(|(i, line)| line.starts_with('+').then_some(i))
                .unwrap();
            let line_count = lines.len();
            let height = line_count - 2 * depth;
            assert_eq!(
                2 * (width * depth + width * height + depth * height),
                content.replace(['\n', '\r'], "").len()
            );
            let (top, back, right, front, left, floor) = (
                get_letters_in_ascii_grid(lines.clone(), 0, 0, width, depth),
                get_letters_in_ascii_grid(lines.clone(), 0, depth, width, height),
                get_letters_in_ascii_grid(lines.clone(), width, depth, depth, height),
                get_letters_in_ascii_grid(lines.clone(), width + depth, depth, width, height),
                get_letters_in_ascii_grid(lines.clone(), 2 * width + depth, depth, depth, height),
                get_letters_in_ascii_grid(lines.clone(), 0, depth + height, width, depth),
            );

            commands
                .spawn((
                    Mesh3d(meshes.add(Cuboid::new(
                        width as f32 * scaling,
                        height as f32 * scaling,
                        depth as f32 * scaling,
                    ))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgba(0.2, 0.3, 0.4, 0.7),
                        metallic: 0.7,
                        perceptual_roughness: 0.3,
                        reflectance: 0.8,
                        emissive: Color::srgba(0.05, 0.05, 0.07, 1.0).into(),
                        cull_mode: Some(bevy::render::render_resource::Face::Front),
                        double_sided: true,
                        unlit: false,
                        fog_enabled: true,
                        ..default()
                    })),
                    bevy::pbr::NotShadowCaster,
                    Pickable::IGNORE,
                    Transform::from_translation(Vec3::new(0.4 * room_index, 0.0, 0.0)),
                ))
                .with_children(|parent| {
                    for obj in &top {
                        let white_matl = materials.add(Color::Hsla(char_to_color(obj.2)));
                        parent
                            .spawn((
                                Mesh3d(meshes.add(Cuboid::new(scaling, scaling * 0.2, scaling))),
                                MeshMaterial3d(white_matl.clone()),
                                Transform::from_translation(Vec3::new(
                                    scaling * (obj.0 as f32 - width as f32 / 2.0 + 0.5),
                                    scaling * (height as f32 / 2.0 - 0.2),
                                    scaling * (0.0 - obj.1 as f32 + depth as f32 / 2.0 - 0.5),
                                )),
                                Object(obj.2),
                            ))
                            .observe(update_material_on::<Pointer<Over>>(
                                materials.add(Color::Hsla(Hsla {
                                    lightness: 0.8,
                                    ..char_to_color(obj.2)
                                })),
                                obj.2,
                            ))
                            .observe(update_material_on::<Pointer<Out>>(
                                white_matl.clone(),
                                obj.2,
                            ));
                    }
                    for obj in &back {
                        let white_matl = materials.add(Color::Hsla(char_to_color(obj.2)));
                        parent
                            .spawn((
                                Mesh3d(meshes.add(Cuboid::new(scaling, scaling, scaling * 0.2))),
                                MeshMaterial3d(white_matl.clone()),
                                Transform::from_translation(Vec3::new(
                                    scaling * (obj.0 as f32 - width as f32 / 2.0 + 0.5),
                                    scaling * (0.0 - obj.1 as f32 + height as f32 / 2.0 - 0.5),
                                    scaling * (0.0 - depth as f32 / 2.0 + 0.2),
                                )),
                                Object(obj.2),
                            ))
                            .observe(update_material_on::<Pointer<Over>>(
                                materials.add(Color::Hsla(Hsla {
                                    lightness: 0.8,
                                    ..char_to_color(obj.2)
                                })),
                                obj.2,
                            ))
                            .observe(update_material_on::<Pointer<Out>>(
                                white_matl.clone(),
                                obj.2,
                            ));
                    }
                    for obj in &right {
                        let white_matl = materials.add(Color::Hsla(char_to_color(obj.2)));
                        parent
                            .spawn((
                                Mesh3d(meshes.add(Cuboid::new(scaling * 0.2, scaling, scaling))),
                                MeshMaterial3d(white_matl.clone()),
                                Transform::from_translation(Vec3::new(
                                    scaling * (width as f32 / 2.0 - 0.2),
                                    scaling * (0.0 - obj.1 as f32 + depth as f32 / 2.0 - 0.5),
                                    scaling * (obj.0 as f32 - height as f32 / 2.0 + 0.5),
                                )),
                                Object(obj.2),
                            ))
                            .observe(update_material_on::<Pointer<Over>>(
                                materials.add(Color::Hsla(Hsla {
                                    lightness: 0.8,
                                    ..char_to_color(obj.2)
                                })),
                                obj.2,
                            ))
                            .observe(update_material_on::<Pointer<Out>>(
                                white_matl.clone(),
                                obj.2,
                            ));
                    }
                    for obj in &front {
                        let white_matl = materials.add(Color::Hsla(char_to_color(obj.2)));
                        parent
                            .spawn((
                                Mesh3d(meshes.add(Cuboid::new(scaling, scaling, scaling * 0.2))),
                                MeshMaterial3d(white_matl.clone()),
                                Transform::from_translation(Vec3::new(
                                    scaling * (0.0 - obj.0 as f32 + width as f32 / 2.0 - 0.5),
                                    scaling * (0.0 - obj.1 as f32 + height as f32 / 2.0 - 0.5),
                                    scaling * (depth as f32 / 2.0 - 0.2),
                                )),
                                Object(obj.2),
                            ))
                            .observe(update_material_on::<Pointer<Over>>(
                                materials.add(Color::Hsla(Hsla {
                                    lightness: 0.8,
                                    ..char_to_color(obj.2)
                                })),
                                obj.2,
                            ))
                            .observe(update_material_on::<Pointer<Out>>(
                                white_matl.clone(),
                                obj.2,
                            ));
                    }
                    for obj in &left {
                        let white_matl = materials.add(Color::Hsla(char_to_color(obj.2)));
                        parent
                            .spawn((
                                Mesh3d(meshes.add(Cuboid::new(scaling * 0.2, scaling, scaling))),
                                MeshMaterial3d(white_matl.clone()),
                                Transform::from_translation(Vec3::new(
                                    scaling * (0.0 - width as f32 / 2.0 + 0.2),
                                    scaling * (0.0 - obj.1 as f32 + depth as f32 / 2.0 - 0.5),
                                    scaling * (0.0 - obj.0 as f32 + height as f32 / 2.0 - 0.5),
                                )),
                                Object(obj.2),
                            ))
                            .observe(update_material_on::<Pointer<Over>>(
                                materials.add(Color::Hsla(Hsla {
                                    lightness: 0.8,
                                    ..char_to_color(obj.2)
                                })),
                                obj.2,
                            ))
                            .observe(update_material_on::<Pointer<Out>>(
                                white_matl.clone(),
                                obj.2,
                            ));
                    }
                    for obj in &floor {
                        let white_matl = materials.add(Color::Hsla(char_to_color(obj.2)));
                        parent
                            .spawn((
                                Mesh3d(meshes.add(Cuboid::new(scaling, scaling * 0.2, scaling))),
                                MeshMaterial3d(white_matl.clone()),
                                Transform::from_translation(Vec3::new(
                                    scaling * (obj.0 as f32 - width as f32 / 2.0 + 0.5),
                                    scaling * (0.0 - height as f32 / 2.0 + 0.2),
                                    scaling * (obj.1 as f32 - depth as f32 / 2.0 + 0.5),
                                )),
                                Object(obj.2),
                            ))
                            .observe(update_material_on::<Pointer<Over>>(
                                materials.add(Color::Hsla(Hsla {
                                    lightness: 0.8,
                                    ..char_to_color(obj.2)
                                })),
                                obj.2,
                            ))
                            .observe(update_material_on::<Pointer<Out>>(
                                white_matl.clone(),
                                obj.2,
                            ));
                    }
                });
            room_index += 1.0;
        }
    }

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 1.0, 3.0).looking_at(Vec3::new(0.0, -0.5, 0.0), Vec3::Y),
        bevy_panorbit_camera::PanOrbitCamera::default(),
    ));
}

#[allow(clippy::type_complexity)]
fn update_material_on<E>(
    new_material: Handle<StandardMaterial>,
    ch: char,
) -> impl Fn(Trigger<E>, Query<(&mut MeshMaterial3d<StandardMaterial>, &Object)>) {
    move |_trigger, mut objects| {
        for (mut material, character) in objects.iter_mut() {
            if character.0 == ch {
                material.0 = new_material.clone();
            }
        }
    }
}

pub fn char_to_color(c: char) -> Hsla {
    let hash = c as u32 * 10007; // Big prime as multiplicator.
    Hsla {
        hue: (hash % 360) as f32,
        saturation: 1.0,
        lightness: 0.5,
        alpha: 1.0,
    }
}
