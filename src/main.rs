use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use rand::Rng;


#[derive(Resource)]
struct Settings {
    temperature: i32,
    oxygen: i32,
    radius: i32,
    speed_left: i32,
    speed_right: i32,
    mutations_left: i32,
    mutations_right: i32,
    max_agents: i32,
    running: bool
}

#[derive(Component)]
struct LineH;

#[derive(Component)]
struct LineV;

#[derive(Component)]
struct CircleLeft;

#[derive(Component)]
struct CircleRight;

#[derive(Component)]
struct AgentLeft {
    temperature: i32,
    oxygen: i32
}

#[derive(Component)]
struct AgentRight {
    temperature: i32,
    oxygen: i32
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(ShapePlugin)
        .add_systems(Startup, (setup, setup_grid, setup_circles))
        .add_systems(Update, (move_grid, update_circles, ui, simulate))
        .run()    
}

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(Settings {
        temperature: 0,
        oxygen: 0,
        radius: 100,
        speed_left: 1,
        speed_right: 1,
        mutations_left: 1,
        mutations_right: 1,
        max_agents: 128,
        running: false
    });
}

fn setup_circles(
    mut commands: Commands
) {
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Circle {
                radius: 100.,
                center: Vec2::splat(0.0)
            }),
            spatial: SpatialBundle {
                transform: Transform::from_xyz(-250.0, 100.0, 2.),
                ..default()
            },
            ..default()
        },
        Stroke::new(Color::WHITE, 2.0),
        Fill::color(Color::NONE),
        CircleLeft
    )).with_children(|parent| {
        parent.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: 1.,
                    center: Vec2::splat(0.0)
                }),
                ..default()
            },
            Stroke::new(Color::NONE, 2.0),
            Fill::color(Color::WHITE),
            AgentLeft {temperature:0, oxygen:0}
        ));
    });

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Circle {
                radius: 100.,
                center: Vec2::splat(0.0)
            }),
            spatial: SpatialBundle {
                transform: Transform::from_xyz(250.0, 100.0, 2.),
                ..default()
            },
            ..default()
        },
        Stroke::new(Color::WHITE, 2.0),
        Fill::color(Color::NONE),
        CircleRight
    )).with_children(|parent| {
        parent.spawn((

            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: 1.,
                    center: Vec2::splat(0.0)
                }),
                ..default()
            },
            Stroke::new(Color::NONE, 2.0),
            Fill::color(Color::WHITE),
            AgentRight {temperature:0, oxygen:0}
        ));
    });
}

fn setup_grid(
   mut commands: Commands,
) {
    for x in -10..=10 {
        let mut path_builder = PathBuilder::new();

        path_builder.move_to(Vec2::new(0.0, -2000.0));
        path_builder.line_to(Vec2::new(0.0, 2000.0));

        path_builder.close();
        let path = path_builder.build();
    
        commands.spawn((
            ShapeBundle {
                path,
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(x as f32 * 200.0, 0.0, 0.),
                    ..default()
                },
                ..default()
            },
            Stroke::new(Color::GRAY, 2.0),
            Fill::color(Color::NONE),
            LineH
        ));
    }

    for y in -10..=10 {
        let mut path_builder = PathBuilder::new();

        path_builder.move_to(Vec2::new(-2000.0, 0.0));
        path_builder.line_to(Vec2::new(2000.0, 0.0));

        path_builder.close();
        let path = path_builder.build();
    
        commands.spawn((
            ShapeBundle {
                path,
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0., y as f32 * 200.0, 0.),
                    ..default()
                },
                ..default()
            },
            Stroke::new(Color::GRAY, 2.0),
            Fill::color(Color::NONE),
            LineV
        ));
    }
}

fn move_grid(
    mut h: Query<&mut Transform, (With<LineH>, Without<LineV>)>,
    mut v: Query<&mut Transform, (With<LineV>, Without<LineH>)>,
    settings: Res<Settings>,
    time: Res<Time>
) {
    if !settings.running {
        return;
    }

    for mut t in h.iter_mut() {
        t.translation.x -= settings.temperature as f32 * time.delta_seconds();
        if t.translation.x > 2000.0 {t.translation.x -= 4000.0;}
        if t.translation.x < -2000.0 {t.translation.x += 4000.0;}
    }

    for mut t in v.iter_mut() {
        t.translation.y += settings.oxygen as f32 * time.delta_seconds();
        if t.translation.y > 2000.0 {t.translation.y -= 4000.0;}
        if t.translation.y < -2000.0 {t.translation.y += 4000.0;}
    }
}

fn update_circles(
    mut q: Query<&mut Path, Or<(With<CircleLeft>, With<CircleRight>)>>,
    settings: Res<Settings>
) {
    for mut c in q.iter_mut() {
        *c = GeometryBuilder::build_as(&shapes::Circle {
            radius: settings.radius as f32,
            center: Vec2::splat(0.0)
        });
    }
}

fn ui(
    mut contexts: EguiContexts, 
    mut settings: ResMut<Settings>,
    q_left: Query<Entity, (With<AgentLeft>, Without<AgentRight>)>,
    q_right: Query<Entity, (With<AgentRight>, Without<AgentLeft>)>,
) {
    egui::TopBottomPanel::bottom("Параметры").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label(format!("Численность слева: {}", q_left.iter().count()));
            ui.label(format!("Численность справа: {}", q_right.iter().count()))
        });
        
        ui.horizontal(|ui| {
            ui.label("Максимальная численность:");
            ui.add(egui::Slider::new(&mut settings.max_agents, 1..=512));
        });

        ui.horizontal(|ui| {
            ui.label("Температура:");
            ui.add(egui::Slider::new(&mut settings.temperature, -50..=50));
        });

        ui.horizontal(|ui| {
            ui.label("Концентрация кислорода:");
            ui.add(egui::Slider::new(&mut settings.oxygen, 0..=100));
        });

        ui.horizontal(|ui| {
            ui.label("Толерантность:");
            ui.add(egui::Slider::new(&mut settings.radius, 10..=200));
        });

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Левая популяция");
                });

                ui.horizontal(|ui| {
                    ui.label("Рождаемость: ");
                    ui.add(egui::Slider::new(&mut settings.speed_left, 1..=4));
                });

                ui.horizontal(|ui| {
                    ui.label("Склонность к мутации: ");
                    ui.add(egui::Slider::new(&mut settings.mutations_left, 0..=10));
                });
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Правая популяция");
                });

                ui.horizontal(|ui| {
                    ui.label("Рождаемость: ");
                    ui.add(egui::Slider::new(&mut settings.speed_right, 1..=4));
                });

                ui.horizontal(|ui| {
                    ui.label("Склонность к мутации: ");
                    ui.add(egui::Slider::new(&mut settings.mutations_right, 0..=10));
                });
            });
        });

        ui.horizontal(|ui| {
            if ui.button("Остановить").clicked() {
                settings.running = false;
            }
            if ui.button("Продолжить").clicked() {
                settings.running = true;
            }
        });
    });
}

fn simulate(
    mut q_left: Query<(Entity, &mut Transform, &Fill, &AgentLeft), Without<AgentRight>>,
    mut q_right: Query<(Entity, &mut Transform, &Fill, &AgentRight), Without<AgentLeft>>,
    circle_left: Query<Entity, (With<CircleLeft>, Without<CircleRight>)>,
    circle_right: Query<Entity, (With<CircleRight>, Without<CircleLeft>)>,
    settings: Res<Settings>,
    mut commands: Commands,
    time: Res<Time>,
    mut timer: Local<Option<Timer>>
) {
    use bevy::utils::Duration;

    if !settings.running {
        return;
    }

    let timer = &mut *timer;

    if *timer == None {
        *timer = Some(Timer::new(Duration::from_secs(1), TimerMode::Repeating));
    }

    let timer = (*timer).as_mut().unwrap();

    timer.tick(time.delta());

    let circle_left = circle_left.single();
    let circle_right = circle_right.single();
    
    let mut cnt_agents_left = 0;
    let mut cnt_agents_right = 0;

    for (entity, mut transform, fill, agent) in q_left.iter_mut() {
        transform.translation.x += (agent.temperature - settings.temperature) as f32 * time.delta_seconds();
        transform.translation.y += (agent.oxygen + settings.oxygen) as f32 * time.delta_seconds();

        if transform.translation.length() > settings.radius as f32 {
            commands.entity(entity).despawn();
        }

        cnt_agents_left += 1;

        if !timer.finished() {
            continue;
        }

        for _ in 0..settings.speed_left {
            commands.spawn(
                (AgentLeft {
                    temperature: agent.temperature + rand::thread_rng().gen_range(-settings.mutations_left..=settings.mutations_left),
                    oxygen: agent.oxygen + rand::thread_rng().gen_range(-settings.mutations_left..=settings.mutations_left),
                },
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Circle {
                        radius: 1.,
                        center: Vec2::splat(0.0)
                    }),
                    spatial: SpatialBundle {
                        transform: transform.clone(),
                        ..default()
                        }, 
                    ..default()
                },
                Stroke::new(Color::NONE, 2.0),
                Fill::color(
                    Color::rgb(
                        (fill.color.r() + rand::thread_rng().gen_range(-0.05..=0.05)).clamp(0.0, 1.0),
                        (fill.color.g() + rand::thread_rng().gen_range(-0.05..=0.05)).clamp(0.0, 1.0),
                        (fill.color.b() + rand::thread_rng().gen_range(-0.05..=0.05)).clamp(0.0, 1.0),
                    )
                ),)
            ).set_parent(circle_left);
        }

        cnt_agents_left += settings.speed_left;
    }

    for (entity, mut transform, fill, agent) in q_right.iter_mut() {
        transform.translation.x += (agent.temperature - settings.temperature) as f32 * time.delta_seconds();
        transform.translation.y += (agent.oxygen + settings.oxygen) as f32 * time.delta_seconds();

        if transform.translation.length() > settings.radius as f32 {
            commands.entity(entity).despawn();
        }

        cnt_agents_right += 1;

        if !timer.finished() {
            continue;
        }

        for _ in 0..settings.speed_right {
            commands.spawn(
                (AgentRight {
                    temperature: agent.temperature + rand::thread_rng().gen_range(-settings.mutations_right..=settings.mutations_right),
                    oxygen: agent.oxygen + rand::thread_rng().gen_range(-settings.mutations_right..=settings.mutations_right),
                },
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Circle {
                        radius: 1.,
                        center: Vec2::splat(0.0)
                    }),
                    spatial: SpatialBundle {
                        transform: transform.clone(),
                        ..default()
                        },
                    ..default()
                },
                Stroke::new(Color::NONE, 2.0),
                Fill::color(
                    Color::rgb(
                        (fill.color.r() + rand::thread_rng().gen_range(-0.05..=0.05)).clamp(0.0, 1.0),
                        (fill.color.g() + rand::thread_rng().gen_range(-0.05..=0.05)).clamp(0.0, 1.0),
                        (fill.color.b() + rand::thread_rng().gen_range(-0.05..=0.05)).clamp(0.0, 1.0),
                    )
                ),)
            ).set_parent(circle_right);
        }

        cnt_agents_right += settings.speed_right;
    }

    for (e, ..) in q_left.iter() {
        if cnt_agents_left <= settings.max_agents {break;}
        commands.entity(e).despawn();
        cnt_agents_left -= 1;
    }

    for (e, ..) in q_right.iter() {
        if cnt_agents_right <= settings.max_agents {break;}
        commands.entity(e).despawn();
        cnt_agents_right -= 1;
    }
}