# Turtle Lib Bevy Upgrade Guide

## Goal and Scope
- upgrade path: `bevy` 0.10.x → 0.16.1
- crates covered: `turtle-lib`, example app(s), supporting tooling
- audience: maintainers comfortable with Rust & Bevy internals

## Dependency Targets
- `bevy` 0.10.x → 0.16.1
- `bevy_inspector_egui` 0.18 → 0.33
- `bevy_tweening` 0.6→0.13
- `bevy_prototype_lyon` 0.8 → 0.13

## Bevy relevant changes for the turtle-lib crate
src: https://bevy.org/news/

### 0.10 → 0.11
- Schedule-first API (mandatory): replace all add_system/add_startup_system calls with schedule-scoped add_systems
	- In `turtle-lib/src/lib.rs::TurtlePlugin::build`:
		- `.add_startup_system(setup)` → `.add_systems(Startup, setup)`
		- `.add_system(keypresses)` → `.add_systems(Update, keypresses)`
		- `.add_system(component_animator_system::<Path>)` → `.add_systems(Update, component_animator_system::<Path>)`
		- `.add_system(close_on_esc)` → `.add_systems(Update, close_on_esc)`
		- `.add_system(draw_lines)` → `.add_systems(Update, draw_lines)`
	- Source: 0.10→0.11 “Schedule-First: the new and improved add_systems”

- Plugin API: `add_plugin` deprecated → use `add_plugins` (single items or tuples)
	- In `TurtlePlugin::build`:
		- `.add_plugin(debug::DebugPlugin)` → `.add_plugins(debug::DebugPlugin)`
		- `.add_plugin(ShapePlugin)` → `.add_plugins(ShapePlugin)`
		- `.add_plugin(TweeningPlugin)` → `.add_plugins(TweeningPlugin)`
	- Source: 0.10→0.11 “Allow tuples and single plugins in add_plugins, deprecate add_plugin”

- Window configuration moved from `WindowDescriptor` to `Window` on `WindowPlugin`
	- In `TurtlePlugin::build` replace:
		- `DefaultPlugins.set(WindowPlugin { window: WindowDescriptor { title, width, height, present_mode, ..Default::default() }, ..default() })`
		- with `DefaultPlugins.set(WindowPlugin { primary_window: Some(Window { title, resolution: (width, height).into(), present_mode, ..default() }), ..default() })`
	- Notes: PresentMode variants are unchanged here; title now takes `String` via `.into()`; width/height become `resolution`.
	- Source: Bevy 0.11 window API (examples/docs)

- Events must derive Event
	- In `turtle-lib/src/events.rs`: add `#[derive(Event)]` to `DrawingStartedEvent`.
	- Ensure `app.add_event::<DrawingStartedEvent>()` remains in `TurtlePlugin::build`.
	- Source: 0.10→0.11 “Require #[derive(Event)] on all Events”

- Reflect derives: `FromReflect` is auto-derived by `#[derive(Reflect)]`
	- In `turtle-lib/src/commands.rs`, many enums/structs derive both `Reflect` and `FromReflect`.
	- You can remove explicit `FromReflect` derives unless you provide a manual impl; if you keep a manual impl, disable auto via `#[reflect(from_reflect = false)]`.
	- Source: 0.10→0.11 “FromReflect Ergonomics Implementation”

### 0.11 → 0.12
- EventReader API rename: `iter()` → `read()`
	- In `turtle-lib/src/lib.rs::draw_lines` change:
		- `for _ev in query_event.iter() { ... }` → `for _ev in query_event.read() { ... }`
	- Also, `&mut EventReader` no longer implements `IntoIterator`; prefer `.read()`.
	- Source: 0.11→0.12 “Refactor EventReader::iter to read” and “Remove IntoIterator impl for &mut EventReader”

- Unified `configure_sets` API (FYI only)
	- If you later configure system sets, prefer `app.configure_sets(ScheduleLabel, …)`. Turtle-lib doesn’t currently call this, so no change.
	- Source: 0.11→0.12 “Replace IntoSystemSetConfig with IntoSystemSetConfigs” and “Unified configure_sets API”

- Present mode/Window tweaks (FYI only)
	- `PresentMode::Fifo` still valid; an additional `FifoRelaxed` variant exists—no change required.
	- Hot reloading workflow changed to feature flag `bevy/file_watcher`; turtle-lib doesn’t configure `AssetPlugin`, so no change.
	- Source: 0.11→0.12 windowing and assets notes

### 0.12 → 0.13
- Scheduling and plugin API are unchanged from 0.12 for turtle-lib
	- Existing schedule-first usage and `add_plugins` continue to work. No changes needed in `TurtlePlugin::build` for scheduling semantics.

- WindowPlugin: primary window field naming consistency
	- If you already use the 0.11+ style (recommended earlier):
		- `WindowPlugin { primary_window: Some(Window { title, resolution, present_mode, ..default() }), ..default() }` continues to be the correct pattern.
	- If any code still uses `WindowPlugin { window: WindowDescriptor { .. } }`, migrate per 0.10→0.11 notes above (crate currently uses `WindowDescriptor` and needs that migration anyway).

- Camera clear color configuration remains valid
	- `Camera2dBundle { camera_2d: Camera2d { clear_color: ClearColorConfig::Custom(Color::BEIGE) }, ..default() }` remains correct. No change needed.

- Input API is stable
	- `Res<Input<KeyCode>>` and `keys.just_pressed(KeyCode::W)` continue to work without changes.

- Events ergonomics (carry-over from 0.11)
	- Ensure custom events derive `#[derive(Event)]` and are registered with `app.add_event::<T>()`. This remains required and stable in 0.13.
	- `EventReader::read()` introduced in 0.12 continues to be the correct method in 0.13; prefer `for ev in reader.read()` over the removed `iter()`.

- Bevy Inspector Egui (debug) still uses plugin as-is
	- `WorldInspectorPlugin` continues to be added via `add_plugins` in debug builds; no API change affecting `turtle-lib` at this hop.

### 0.13 → 0.14
- Color API refinements (no code change needed for current usage)
	- Named colors like `Color::BEIGE`, `Color::MIDNIGHT_BLUE`, `Color::BLACK` remain valid. No renames impacting `turtle-lib` in this hop.
	- Type-safe color constructors improved, but we don’t currently construct custom colors beyond constants.

- Windowing migrated to winit 0.30 (FYI)
	- No direct use of backend-specific window APIs in `turtle-lib`; existing `Window { title, resolution, present_mode }` remains valid.
	- If examples or downstream apps interact with window events, check their migrations separately; not applicable in this crate.

- Events are always updated in minimal apps (FYI)
	- Bevy 0.14 guarantees events update even in minimal setups; `turtle-lib` already uses `DefaultPlugins` and explicit `add_event::<DrawingStartedEvent>()` so no action required.

- Scheduling/UI/Camera
	- `Camera2dBundle` stays stable in this hop; our use of `ClearColorConfig::Custom(Color::BEIGE)` continues to be correct.
	- `close_on_esc` remains available via `bevy::window::close_on_esc` with the same call pattern.

### 0.14 → 0.15
- Query::single family migrated to `Single` (N/A)
	- `turtle-lib` does not call `Query::single()`/`single_mut()`; no migration needed here.

- State-scoped events (FYI)
	- 0.15 introduces state-scoped events; `turtle-lib` uses a global `DrawingStartedEvent` and doesn’t scope events to states. No change required.

- Color, camera, and window remain compatible
	- Existing use of `Color::BEIGE`, `Camera2dBundle`, and `Window { resolution, present_mode }` remains valid; no API renames affecting our code in this hop.

- Scheduling order clarifications (FYI)
	- 0.15 tightened ambiguities in system ordering; `turtle-lib` doesn’t rely on ambiguous ordering between systems, and uses default sets. No action needed.

### 0.15 → 0.16
- ECS Relationships replace `Parent`/`Children` (N/A)
	- `turtle-lib` doesn’t use hierarchy components directly; no migration required.

- Improved spawn ergonomics (N/A)
	- The new `children!`/`related!` helpers don’t affect current code; we spawn via bundles without parent-child APIs.

- Unified ECS error handling (optional)
	- Systems and commands can return `Result` in 0.16+. `turtle-lib` systems currently return `()`. You may adopt `Result` for clearer error paths later, but no change is required to compile.

- Window/Input/Camera remain compatible
	- `close_on_esc`, `Input<KeyCode>`, `Camera2dBundle`, and `ClearColorConfig::Custom(Color::BEIGE)` continue to work. No API renames in these areas affecting current usage.

- Tweening and Lyon plugins
	- API incompatibilities, if any, are tracked in their dedicated sections below. No direct Bevy-0.16-specific change required in our calls (`Animator`, `Tween`, `Path`).

## Bevy_inspector_egui relevant changes for the turtle-lib crate
src: https://github.com/jakobhellermann/bevy-inspector-egui/blob/main/docs/CHANGELOG.md
src: https://github.com/jakobhellermann/bevy-inspector-egui/blob/main/docs/MIGRATION_GUIDE_0.15_0.16.md

- Version alignment across Bevy upgrades
	- 0.12 → 0.13: use bevy_inspector_egui ≥ 0.21 (changelog shows 0.21 updated to Bevy 0.12; 0.23 to Bevy 0.13). Our target path ends at 0.33 for Bevy 0.16.
	- 0.14: use ≥ 0.25 (changelog: 0.25.0 updated to Bevy 0.14).
	- 0.15: use ≥ 0.28 (changelog: 0.28.0 updated to Bevy 0.15).
	- 0.16: use ≥ 0.31 (changelog: 0.31.0 updated to Bevy 0.16). Latest at time of writing is 0.33.x.

- WorldInspectorPlugin path and construction (0.15 → 0.16)
	- The old `WorldInspectorPlugin` moved to `bevy_inspector_egui::quick::WorldInspectorPlugin`.
	- In `turtle-lib/src/debug.rs`, ensure we import from `quick`:
		- `use bevy_inspector_egui::quick::WorldInspectorPlugin;`
	- Continue to add it conditionally in debug:
		- `app.add_plugins(WorldInspectorPlugin);` (use `add_plugins`, not `add_plugin`).
	- The removed `WorldInspectorParams` is not used in this crate, so no action is required.

- Reflect-driven inspector (0.15 → 0.16 rewrite)
	- The inspector is centered on `Reflect` and type registration. We already register types we care about (`TurtleColors`, `TurtleCommands`). No additional registration is necessary for using the world inspector itself.
	- If we later add per-resource inspectors (e.g., `ResourceInspectorPlugin<T>`), derive `Reflect` for `T` and call `app.register_type::<T>()`.

- Schedules/ambiguity fixes in quick plugins
	- The crate’s quick plugins run in their own schedule to avoid system ambiguity since ~0.25. No overrides needed by `turtle-lib`.

- Known guardrails
	- 0.18.4 added a guard for missing `PrimaryWindow`; our app defines a primary window via `DefaultPlugins.set(WindowPlugin { … })`, so the inspector will work in debug.

## Bevy_tweening relevant changes for the turtle-lib crate
src: https://github.com/djeedai/bevy_tweening/blob/main/CHANGELOG.md

- Version alignment with Bevy
	- Bevy 0.10 → bevy_tweening 0.7
	- Bevy 0.11 → bevy_tweening 0.8
	- Bevy 0.12 → bevy_tweening 0.9
	- Bevy 0.13 → bevy_tweening 0.10
	- Bevy 0.14 → bevy_tweening 0.11
	- Bevy 0.15 → bevy_tweening 0.12
	- Bevy 0.16 → bevy_tweening 0.13

- Custom Lens signature change (bevy_tweening 0.11 for Bevy 0.14)
	- Impacted files: `turtle-lib/src/drawing/animation/line_lens.rs`, `turtle-lib/src/drawing/animation/circle_lens.rs`.
	- Update `Lens<T>::lerp(&mut self, target: &mut T, ratio: f32)` → `Lens<T>::lerp(&mut self, target: &mut dyn Targetable<T>, ratio: f32)`.
	- Code using `target` can remain unchanged because `dyn Targetable<T>` implements `Defer`/`DeferMut` and derefs like `&mut T`.
	- This change is required when moving to Bevy 0.14 (bevy_tweening 0.11+) and remains compatible up to 0.13.

- EaseFunction source update (bevy_tweening 0.12 for Bevy 0.15)
	- `interpolation::EaseFunction` was replaced by `bevy_math::EaseFunction` and re-exported by bevy_tweening.
	- Our code imports `bevy_tweening::EaseFunction`, which continues to work. Alternatively, import `bevy_math::EaseFunction` directly.

- Animator systems and Path animations
	- Continue adding `TweeningPlugin` to the app.
	- Continue registering `component_animator_system::<Path>` in `Update` to animate `Animator<Path>` (still required up to bevy_tweening 0.13).
	- Note: The removal of `component_animator_system` is planned in a future bevy_tweening 0.14 (unreleased at the time of writing) and does not apply to our 0.13 target.

- Completion events
	- `TweenCompleted` event remains available through 0.13. Our use of `.with_completed_event(state.segment_index() as u64)` is valid up to 0.13.
	- If migrating beyond 0.13 later, the API changes (no `user_data`, new event types) will require adjustments; out of scope for Bevy 0.16 + bevy_tweening 0.13.

- Bevy 0.12 EventReader change applies here too
	- When listening to `TweenCompleted`, use `EventReader<TweenCompleted>::read()` (not `iter()`), as documented in the Bevy 0.11→0.12 section above.

## Bevy_prototype_lyon relevant changes for the turtle-lib crate
src: https://github.com/Nilirad/bevy_prototype_lyon/blob/master/CHANGELOG.md

- Version alignment with Bevy
	- Bevy 0.10 → lyon 0.8
	- Bevy 0.11 → lyon 0.9
	- Bevy 0.12 → lyon 0.10
	- Bevy 0.13 → lyon 0.11
	- Bevy 0.14 → lyon 0.12
	- Bevy 0.15 → lyon 0.13
	- Bevy 0.16 → lyon 0.14

- 0.8: Fill/Stroke rename
	- `FillMode`/`StrokeMode` renamed to `Fill`/`Stroke` and became Components.
	- In `turtle-lib/src/turtle_bundle.rs`, if still using the old names, update:
		- `DrawMode::Outlined { fill_mode: FillMode::color(c), outline_mode: StrokeMode::new(c2, w) }`
		- to the current API for your target lyon version (≤0.13 still supports DrawMode; in 0.14 see below).

- 0.13: `ShapeBundle` composition change (FYI)
	- Deprecated `SpatialBundle` was removed from `ShapeBundle`; `Transform` and `Visibility` are added separately internally.
	- We construct shapes via `GeometryBuilder::build_as(...)` returning `ShapeBundle`; we don’t access its fields—no code change required.

- 0.14 (Bevy 0.16): Major API rework affecting turtle-lib
	- Path component renamed to `Shape`; `Shape` now includes fill and stroke data.
	- `Fill` and `Stroke` are no longer `Component`s; they’re part of `Shape` data.
	- `GeometryBuilder` and `PathBuilder` removed. Use `ShapeBuilder` and `ShapePath`:
		- Build shapes: `ShapeBuilder::with(&geometry).fill(color).stroke((color, width)).build()`.
		- `ShapePath` now implements `Geometry` and replaces direct `PathBuilder` usage.
	- `ShapeBundle` is deprecated and no longer exported from `prelude`.

	Impacted code and migration sketch for 0.14:
	- Imports in `turtle-lib/src/lib.rs` and lenses:
		- Replace `use bevy_prototype_lyon::prelude::{Path, ShapePlugin};` with `use bevy_prototype_lyon::prelude::{Shape, ShapePath, ShapeBuilder, ShapePlugin};`
	- Animator and systems:
		- `Animator<Path>` → `Animator<Shape>`
		- `component_animator_system::<Path>` → `component_animator_system::<Shape>`
	- Lenses (`drawing/animation/line_lens.rs`, `circle_lens.rs`):
		- `impl Lens<Path> for ...` → `impl Lens<Shape> for ...`
		- Replace `PathBuilder` usage with creating/updating a `ShapePath` geometry and attaching it to the `Shape` target per the new API.
	- Spawn/build (`turtle_bundle.rs`):
		- Replace `GeometryBuilder::build_as(&shapes::turtle(), DrawMode::Outlined { ... }, Transform::IDENTITY)` and `ShapeBundle`
		- with `ShapeBuilder::with(&shapes::turtle()).fill(Color::MIDNIGHT_BLUE).stroke((Color::BLACK, 1.0)).build()`.

Notes:
- If you stop at lyon 0.13 (Bevy 0.15), you don’t need the 0.14 rework yet. If you upgrade to Bevy 0.16, adopt lyon 0.14 and apply the above migrations.