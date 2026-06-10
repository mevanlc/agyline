# Subcomponents

Let a component be "focused into" so its internal parts (subcomponents) can be
reordered, toggled, recolored, and separated using the same interaction style
as top-level components.

## Motivating example

The `Model` component currently renders as a single segment, e.g.
`🤖 Claude Opus 4.7 (1 million)`. Subcomponents split that into ordered parts:

- `Name` — e.g. `Opus`
- `Version` — e.g. `4.7`
- `Extras` — e.g. `(1 million)` or empty

Each part gets its own enabled flag, icon, colors, bold, and participates in a
per-component intra-subcomponent separator.

## Scope

v1: `Model` only. Land the framework and the Model conversion. Other components
(`Directory`, `Git`, `Cost`, `Session`, …) in follow-ups once we've learned
what's awkward in practice.

## Design Decisions

### 1. Data model

Each component type declares a fixed, ordered list of subcomponent *kinds*.
For `Model`: `[Name, Version, Extras]`. Kinds are defined by the component
impl, not user-addable.

`Component::collect()` returns a `Vec<SubcomponentData>` (ordered by kind)
instead of a single `ComponentData`. A component with no meaningful split
returns a single-element vec (the existing components pre-conversion).

`ComponentConfig` gains:

```rust
pub struct ComponentConfig {
    // ...existing fields: id, enabled, icon, colors, styles, options
    pub subcomponents: Vec<SubcomponentConfig>,     // user ordering + per-sub settings
    pub sub_separator: Option<SeparatorConfig>,     // intra-component separator
}

pub struct SubcomponentConfig {
    pub id: SubcomponentId,        // enum variant scoped per component type
    pub enabled: bool,
    pub icon: IconConfig,          // same shape as top-level IconConfig
    pub colors: ColorConfig,       // all Options — None means "inherit from parent"
    pub styles: TextStyleConfig,
}
```

Persistence: TOML gains a nested `[[components.subcomponents]]` array and an
optional `[components.sub_separator]` table.

**No backcompat / no migration.** Existing user theme files are expected to
break and be regenerated from scratch. Do not write load-time shims to
populate defaults from missing fields — deserialization can hard-fail on old
configs. Built-in preset themes get rewritten by hand as part of this work.

### 2. Color inheritance

Subcomponent colors are `Option<AnsiColor>` everywhere and default to `None`.
`None` means "inherit from the parent component's color". This is applied at
render time — no resolved values are persisted.

The editor's color picker for a subcomponent field needs an explicit
"Unset / Inherit" action to return a color to `None` after it's been set. The
editor should also surface the effective (inherited) color somehow so users
aren't confused about what's rendering. Low priority for v1 — acceptable to
just show "Inherit" text and let the preview speak.

Ergonomics nice-to-have (can defer): a "copy from parent" shortcut that
materializes the inherited color as an explicit override.

### 3. Interaction model

**v1 UX — dive-in via extra panel level.**

New `Panel` variant: `Subcomponents` (a focused-component-scoped list). From
`ComponentList`, pressing Enter or Right on a component with subcomponents
replaces/overlays the component list with that component's subcomponent list.
Esc / Left pops back.

Inside `Subcomponents`:
- `↑` / `↓` — navigate
- `Shift+↑` / `Shift+↓` — reorder
- `Space` / `Enter` — toggle enabled
- `Tab` — jump to editor panel for the selected subcomponent

Subcomponent list includes a `Separator` entry at the bottom (same convention
as the top-level list), which exposes `sub_separator` for editing. It is not
reorderable and its enabled flag controls whether the intra-component
separator renders at all. When disabled, subcomponents render adjacent with no
separator between them.

The editor panel grows a parallel `FieldSelection` variant for
subcomponent-scoped fields: Enabled, StyleMode (inherit or override?), Icons,
IconColor, TextColor, BackgroundColor, Bold.

**Future UX to experiment with:** push/pop within the leftmost panel so the
dive-in doesn't require an additional column of screen real estate — better
for size-constrained terminals. Design the state model so this is a rendering
choice, not a data-model change.

### 4. Rendering

In `build_render_line`, one enabled `ComponentConfig` can now expand into
multiple `Segment`s — one per enabled subcomponent, in the user-configured
order.

Between consecutive enabled subcomponents of the same parent, insert the
parent's `sub_separator` (if enabled) — using the same Powerline/plain logic
as the top-level separator. The top-level separator continues to appear
between different components.

Edge case: if a parent component has only one enabled subcomponent, no
sub-separator is rendered. If all are disabled, the whole component is
skipped (same as `enabled = false` today).

### 5. StyleMode

Open question: does each subcomponent get its own StyleMode (Plain / NerdFont
/ Powerline / PlainPowerline), or does it inherit the parent's?

v1 recommendation: **inherit from parent, no per-subcomponent override.**
Powerline transitions between subcomponents use their backgrounds, same as
between top-level components. Simpler; revisit if a concrete need appears.

## Non-goals (v1)

- Nested subcomponents (subcomponents-of-subcomponents).
- User-defined subcomponent kinds.
- Per-subcomponent StyleMode.
- Converting non-Model components.
- "Copy from parent" color shortcut.

## Model specifics

The parent `Model` component keeps its icon options (base plain / nerd-font
icons, colors, etc.) — same as other top-level components.

Per-model icons (Opus / Sonnet / Haiku) move **down** to the `Name`
subcomponent as an opt-in override: Name has its own `per_model` toggle and
three per-model icon fields. When enabled, the Name subcomponent's dynamic
icon overrides the parent `Model` icon for that segment. When disabled, Name
inherits as usual. `dynamic_icon` metadata flows from `collect()` on the Name
subcomponent.

## Open Questions

- How to visually indicate inherited colors in the editor without clutter.
- Whether sub-separator defaults should differ from top-level (e.g. a thinner
  glyph) or match.
