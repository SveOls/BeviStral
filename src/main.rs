use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "BeviStral".to_string(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_sub_state::<GamePhase>()
        .add_systems(Startup, setup_camera_and_background)
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_screen::<OnMainMenuScreen>)
        .add_systems(OnEnter(AppState::InGame), setup_game_screen)
        .add_systems(OnExit(AppState::InGame), despawn_screen::<OnGameScreen>)
        .add_systems(OnEnter(AppState::Paused), setup_pause_menu)
        .add_systems(OnExit(AppState::Paused), despawn_screen::<OnPauseScreen>)
        .add_systems(
            Update,
            main_menu_button_handler.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(
            Update,
            pause_button_handler.run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            pause_menu_button_handler.run_if(in_state(AppState::Paused)),
        )
        .add_systems(
            Update,
            pause_key_handler.run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            OnTransition {
                exited: AppState::MainMenu,
                entered: AppState::InGame,
            },
            transition_to_game,
        )
        .add_systems(
            OnTransition {
                exited: AppState::InGame,
                entered: AppState::MainMenu,
            },
            transition_to_menu,
        )
        .run();
}

#[derive(States, Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}

#[derive(SubStates, Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
#[source(AppState = AppState::InGame)]
pub enum GamePhase {
    #[default]
    Day,
    Night,
}

#[derive(States, Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum MenuTab {
    #[default]
    Play,
    Settings,
    Credits,
}

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct OnPauseScreen;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum MenuButton {
    Play,
    Settings,
    Credits,
}

#[derive(Component)]
struct PauseButton;

#[derive(Component)]
struct ResumeButton;

#[derive(Component)]
struct QuitToMenuButton;

fn setup_camera_and_background(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn despawn_screen<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_button(parent: &mut ChildSpawnerCommands, text: &str, marker: impl Component) {
    parent
        .spawn((
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(16.0)),
                margin: UiRect::all(Val::Px(8.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.17, 0.19, 0.24)),
            Button,
        ))
        .insert(marker)
        .with_children(|button| {
            button.spawn((
                Text::new(text),
                TextFont {
                    font_size: FontSize::Px(24.0),
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.95)),
            ));
        });
}

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.06, 0.08)),
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("BeviStral"),
                TextFont {
                    font_size: FontSize::Px(64.0),
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.95)),
            ));

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                })
                .with_children(|row| {
                    spawn_button(row, "Play", MenuButton::Play);
                    spawn_button(row, "Settings", MenuButton::Settings);
                    spawn_button(row, "Credits", MenuButton::Credits);
                });
        });
}

fn setup_game_screen(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            OnGameScreen,
        ))
        .with_children(|parent| {
            spawn_button(parent, "Pause", PauseButton);
        });
}

fn setup_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
            OnPauseScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Paused"),
                TextFont {
                    font_size: FontSize::Px(48.0),
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.95)),
            ));

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                })
                .with_children(|column| {
                    spawn_button(column, "Resume", ResumeButton);
                    spawn_button(column, "Quit to Menu", QuitToMenuButton);
                });
        });
}

fn main_menu_button_handler(
    interaction_query: Query<
        (&Interaction, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button {
                MenuButton::Play => next_state.set(AppState::InGame),
                MenuButton::Settings => {}
                MenuButton::Credits => {}
            }
        }
    }
}

fn pause_button_handler(
    interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<PauseButton>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::Paused);
        }
    }
}

fn pause_menu_button_handler(
    resume_query: Query<
        &Interaction,
        (Changed<Interaction>, With<ResumeButton>, With<Button>),
    >,
    quit_query: Query<
        &Interaction,
        (Changed<Interaction>, With<QuitToMenuButton>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for interaction in &resume_query {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::InGame);
        }
    }
    for interaction in &quit_query {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::MainMenu);
        }
    }
}

fn pause_key_handler(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Paused);
    }
}

fn transition_to_game() {
    info!("Entering InGame; GamePhase substate activates");
}

fn transition_to_menu() {
    info!("Returning to MainMenu");
}
