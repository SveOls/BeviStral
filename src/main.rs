use bevy::prelude::*;

mod io;

use io::simulated;
use io::{BindTag, IoPlugin, TagId};

fn main() {
    App::new()
        .add_plugins(IoPlugin::new(simulated::SimulatedSource::default()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "BeviStral".to_string(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_sub_state::<MainMenu>()
        .add_sub_state::<SettingsTab>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_screen::<OnMainMenuScreen>)
        .add_systems(OnEnter(MainMenu::Root), spawn_main_menu_root)
        .add_systems(OnExit(MainMenu::Root), despawn_screen::<OnMainMenuRootScreen>)
        .add_systems(OnEnter(MainMenu::Settings), spawn_settings_menu)
        .add_systems(OnExit(MainMenu::Settings), despawn_screen::<OnSettingsScreen>)
        .add_systems(OnEnter(MainMenu::LiveData), spawn_live_data)
        .add_systems(OnExit(MainMenu::LiveData), despawn_screen::<OnLiveDataScreen>)
        .add_systems(OnEnter(SettingsTab::Audio), spawn_audio_panel)
        .add_systems(OnExit(SettingsTab::Audio), despawn_screen::<OnAudioPanelScreen>)
        .add_systems(OnEnter(SettingsTab::Video), spawn_video_panel)
        .add_systems(OnExit(SettingsTab::Video), despawn_screen::<OnVideoPanelScreen>)
        .add_systems(OnEnter(SettingsTab::Controls), spawn_controls_panel)
        .add_systems(OnExit(SettingsTab::Controls), despawn_screen::<OnControlsPanelScreen>)
        .add_systems(
            Update,
            main_menu_root_button_handler.run_if(in_state(MainMenu::Root)),
        )
        .add_systems(
            Update,
            settings_button_handler.run_if(in_state(MainMenu::Settings)),
        )
        .add_systems(
            Update,
            live_data_button_handler.run_if(in_state(MainMenu::LiveData)),
        )
        .add_systems(
            Update,
            back_key_handler.run_if(in_state(MainMenu::Settings)),
        )
        .add_systems(
            Update,
            live_data_back_key_handler.run_if(in_state(MainMenu::LiveData)),
        )
        .run();
}

#[derive(States, Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
}

#[derive(SubStates, Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
#[source(AppState = AppState::MainMenu)]
pub enum MainMenu {
    #[default]
    Root,
    Settings,
    LiveData,
}

#[derive(SubStates, Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
#[source(MainMenu = MainMenu::Settings)]
pub enum SettingsTab {
    #[default]
    Audio,
    Video,
    Controls,
}

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct OnMainMenuRootScreen;

#[derive(Component)]
struct OnSettingsScreen;

#[derive(Component)]
struct OnLiveDataScreen;

#[derive(Component)]
struct OnAudioPanelScreen;

#[derive(Component)]
struct OnVideoPanelScreen;

#[derive(Component)]
struct OnControlsPanelScreen;

#[derive(Component, Clone, Copy)]
enum MainMenuButton {
    Settings,
    LiveData,
    Quit,
}

#[derive(Component, Clone, Copy)]
struct SettingsTabButton(SettingsTab);

#[derive(Component)]
struct BackButton;

fn setup_camera(mut commands: Commands) {
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
            Interaction::default(),
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

fn spawn_label(parent: &mut ChildSpawnerCommands, text: &str) {
    parent.spawn((
        Text::new(text),
        TextFont {
            font_size: FontSize::Px(22.0),
            ..default()
        },
        TextColor(Color::srgb(0.75, 0.78, 0.85)),
        Node {
            margin: UiRect::top(Val::Px(10.0)),
            ..default()
        },
    ));
}

fn spawn_title(parent: &mut ChildSpawnerCommands, text: &str) {
    parent.spawn((
        Text::new(text),
        TextFont {
            font_size: FontSize::Px(48.0),
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.95)),
    ));
}

fn panel_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }
}

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            panel_node(),
            BackgroundColor(Color::srgb(0.05, 0.06, 0.08)),
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            spawn_title(parent, "BeviStral");
        });
}

fn spawn_main_menu_root(mut commands: Commands) {
    commands
        .spawn((
            panel_node(),
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
            OnMainMenuRootScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                })
                .with_children(|column| {
                    spawn_button(column, "Settings", MainMenuButton::Settings);
                    spawn_button(column, "Live Data", MainMenuButton::LiveData);
                    spawn_button(column, "Quit", MainMenuButton::Quit);
                });
        });
}

fn spawn_settings_menu(mut commands: Commands) {
    commands
        .spawn((
            panel_node(),
            BackgroundColor(Color::srgb(0.07, 0.08, 0.11)),
            OnSettingsScreen,
        ))
        .with_children(|parent| {
            spawn_title(parent, "Settings");
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                })
                .with_children(|row| {
                    spawn_button(row, "Audio", SettingsTabButton(SettingsTab::Audio));
                    spawn_button(row, "Video", SettingsTabButton(SettingsTab::Video));
                    spawn_button(row, "Controls", SettingsTabButton(SettingsTab::Controls));
                });
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                })
                .with_children(|content| {
                    content
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(16.0)),
                            border_radius: BorderRadius::all(Val::Px(6.0)),
                            ..default()
                        })
                        .insert(BackgroundColor(Color::srgb(0.1, 0.11, 0.15)))
                        .with_children(|panel| {
                            spawn_button(panel, "Back", BackButton);
                        });
                });
        });
}

fn spawn_audio_panel(mut commands: Commands) {
    commands
        .spawn((
            panel_node(),
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            OnAudioPanelScreen,
        ))
        .with_children(|parent| {
            spawn_title(parent, "Audio");
            spawn_label(parent, "Master volume: 80%");
            spawn_label(parent, "Music volume: 60%");
            spawn_label(parent, "SFX volume: 75%");
        });
}

fn spawn_video_panel(mut commands: Commands) {
    commands
        .spawn((
            panel_node(),
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            OnVideoPanelScreen,
        ))
        .with_children(|parent| {
            spawn_title(parent, "Video");
            spawn_label(parent, "Resolution: 1920x1080");
            spawn_label(parent, "VSync: On");
            spawn_label(parent, "UI scale: 100%");
        });
}

fn spawn_controls_panel(mut commands: Commands) {
    commands
        .spawn((
            panel_node(),
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            OnControlsPanelScreen,
        ))
        .with_children(|parent| {
            spawn_title(parent, "Controls");
            spawn_label(parent, "Escape: back");
            spawn_label(parent, "Enter / Space: activate button");
        });
}

fn spawn_live_data(mut commands: Commands) {
    commands
        .spawn((
            panel_node(),
            BackgroundColor(Color::srgb(0.07, 0.08, 0.11)),
            OnLiveDataScreen,
        ))
        .with_children(|parent| {
            spawn_title(parent, "Live Data");
            spawn_tag_label(
                parent,
                "Line 1 speed",
                simulated::TAG_LINE1_SPEED,
            );
            spawn_tag_label(
                parent,
                "Line 1 temperature",
                simulated::TAG_LINE1_TEMP,
            );
            spawn_tag_label(parent, "Line 2 parts", simulated::TAG_LINE2_COUNT);
            spawn_tag_label(parent, "Tank level", simulated::TAG_TANK_LEVEL);
            spawn_tag_label(parent, "Pump", simulated::TAG_PUMP_RUNNING);
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                })
                .with_children(|column| {
                    spawn_button(column, "Back", BackButton);
                });
        });
}

fn spawn_tag_label(parent: &mut ChildSpawnerCommands, label: &str, tag: &str) {
    parent.spawn((
        Text::new(format!("{label}: …")),
        TextFont {
            font_size: FontSize::Px(22.0),
            ..default()
        },
        TextColor(Color::srgb(0.75, 0.78, 0.85)),
        Node {
            margin: UiRect::top(Val::Px(10.0)),
            ..default()
        },
        BindTag {
            tag: TagId::new(tag),
            label: label.to_string(),
            last_display: None,
        },
    ));
}

fn main_menu_root_button_handler(
    interaction_query: Query<
        (&Interaction, &MainMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<MainMenu>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button {
                MainMenuButton::Settings => {
                    next_state.set(MainMenu::Settings);
                }
                MainMenuButton::LiveData => {
                    next_state.set(MainMenu::LiveData);
                }
                MainMenuButton::Quit => {
                    exit.write(AppExit::Success);
                }
            }
        }
    }
}

fn settings_button_handler(
    tab_query: Query<
        (&Interaction, &SettingsTabButton),
        (Changed<Interaction>, With<Button>),
    >,
    back_query: Query<
        &Interaction,
        (Changed<Interaction>, With<BackButton>, With<Button>),
    >,
    mut next_tab: ResMut<NextState<SettingsTab>>,
    mut next_menu: ResMut<NextState<MainMenu>>,
) {
    for (interaction, button) in &tab_query {
        if *interaction == Interaction::Pressed {
            next_tab.set(button.0);
        }
    }
    for interaction in &back_query {
        if *interaction == Interaction::Pressed {
            next_menu.set(MainMenu::Root);
        }
    }
}

fn back_key_handler(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MainMenu>>,
    state: Res<State<MainMenu>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) && *state.get() == MainMenu::Settings {
        next_state.set(MainMenu::Root);
    }
}

fn live_data_button_handler(
    back_query: Query<
        &Interaction,
        (Changed<Interaction>, With<BackButton>, With<Button>),
    >,
    mut next_menu: ResMut<NextState<MainMenu>>,
) {
    for interaction in &back_query {
        if *interaction == Interaction::Pressed {
            next_menu.set(MainMenu::Root);
        }
    }
}

fn live_data_back_key_handler(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MainMenu>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(MainMenu::Root);
    }
}
