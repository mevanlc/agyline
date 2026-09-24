# Named theme catalog

`config.toml` is the only configuration file read by agyline. The current schema is version 2. It stores one `active_theme` reference and four maps: `themes`, `component_configs`, `icon_configs`, and `color_configs`. Built-ins live in the binary and are never installed as files. A missing catalog uses built-in Default in memory until an explicit save or activation.

See the complete, validated [example catalog](../examples/config.toml). Try it without changing your current setup:

```sh
mkdir -p /tmp/agyline-example
cp examples/config.toml /tmp/agyline-example/config.toml
agyline --config-dir /tmp/agyline-example --validate-config
agyline --config-dir /tmp/agyline-example
```

The directory precedence is `--config-dir`, `AGYLINE_CONFIG_DIR`, `XLINE_CONFIG_DIR`, then `~/.config/agyline`. `XLINE_CONFIG_DIR` remains a directory override, not an old-format reader. Old `themes/*.toml` files are ignored and left untouched. There is no automatic migration or `--install-themes` command.

## Resources and ownership

Every reference has both `source = "builtin" | "user"` and `name`. User names can match a built-in name because their sources differ. Within each custom resource kind, names must be nonempty, trimmed, contain no control characters, fit in 200 UTF-8 bytes, and be unique without regard to case. References use exact names.

| Resource | Fields it owns |
| --- | --- |
| Theme | Exactly three references: `components`, `icons`, `colors`; optional description |
| Component config | Ordered `components` array: machine `id`, `enabled`, and behavior `options` |
| Icon config | `glyph_mode`, `powerline`, `defaults`, per-component glyph overrides, and `separator` |
| Color config | `defaults`, per-component color/bold overrides, and separator foreground |

Missing supported components appear disabled after the configured entries in registry order. Saving that component config materializes the complete list. A separator is not a persisted data component. Disabled unknown component entries and unknown option keys survive round trips; enabling an unknown component prevents a referencing theme from resolving. Known option values and types are validated.

Icon `glyph_mode` is `plain` or `nerd_font`; `powerline` is an independent Boolean. `separator.enabled` controls ordinary separators only. `plain` and `nerd_font` are the ordinary separator strings; `powerline_plain` and `powerline_nerd_font` are the independent connector strings used while Powerline is on. Connector colors come from adjacent segment backgrounds. Glyph strings may be empty or contain multiple Unicode characters; terminal control characters are rejected.

Glyph entries contain `plain`, `nerd_font`, optional `thinking_icon`, and optional `per_model`. A `per_model` table has `enabled` and tier tables (`flash`, `pro`, `ultra`, `flash_lite`, `opus`, `sonnet`, `haiku`, `fable`, `mythos`), each with `plain` and `nerd_font`. Omitted tiers use the shipped tier defaults. In the TUI, tier edits use the config's current glyph mode.

Color entries contain optional `icon`, `text`, and `background` plus `text_bold`. A color is exactly one of `{ c16 = 0..15 }`, `{ c256 = 0..255 }`, or `{ r = 0..255, g = 0..255, b = 0..255 }`. An omitted color means terminal default. The separator color table contains only optional `icon`.

For icons and colors, a missing component override uses the whole `defaults` entry. Once an override exists, its omitted colors mean terminal default; individual fields do not inherit again. The Defaults row edits the fallback entry, and `v` on a component removes its override.

## Editing and sharing

The four sections are Themes, Components, Icons, and Colors. Themes associates named configs. The other sections are independent libraries: choosing a resource previews it in the current theme context but does not retarget that theme. Use an association picker in Themes to retarget it.

Built-ins are immutable. Editing one prompts for a named copy; if its theme is also built-in, agyline creates a custom theme draft. Editing a config used by multiple themes defaults to copying it for the current theme. **Edit shared original** is explicit, shows dependent themes and active usage, and remains visibly marked for that editing session. `u` inspects dependent themes at any time.

**Duplicate** on a theme reuses its three references. **Full fork** creates a new theme and independent copies of all three configs. Duplicate on a config creates an independent library resource. Unused configs are retained until explicitly deleted.

Renaming patches every saved reference in the same atomic save, including `active_theme` when appropriate. This does not commit unrelated changes from dependent theme drafts. Delete is blocked while a config has dependents or a theme is active; reassign/save dependencies or activate a replacement first.

## Saving, activation, and recovery

Navigation never saves or discards a draft. `s` reviews and saves the current context and its required related drafts. Theme saves include dirty referenced configs; customizing and full-fork operations save their new resources and theme retargets together. Unrelated drafts remain unsaved. `S` reviews and saves all drafts.

`a` activates the current theme. If that theme or its referenced configs have drafts, it offers Save and activate. `x` saves, activates, and then exits; remaining unrelated drafts still use the normal save/discard/cancel exit flow. Saving a config already referenced by the active theme changes the next rendered statusline without reactivation. There is no activation snapshot.

Saves validate the candidate catalog, acquire `config.toml.lock`, check the original file bytes, write and sync a temporary file in the same directory, then replace `config.toml` atomically and sync the directory. Application writers coordinate through the lock; external editors do not honor it. Comments and formatting are rewritten on save.

A failed save retains drafts and does not activate or exit. If the catalog changed externally, use `E` to export a uniquely named draft catalog and `R` to reload. Export is a recovery copy of the entire draft catalog and may include unfinished/invalid drafts. A durability failure after replacement explicitly requires reload because the write may already have succeeded. A malformed catalog opens a diagnostic/reload screen; rendering and validation report an error and never replace it.

The whole catalog is the sharing and backup artifact. Per-theme portable export/import is outside this refactor.

## Keyboard and terminal sizes

| Keys | Action |
| --- | --- |
| `1`–`4` | Switch section |
| `[` / `]`, `/` | Previous/next resource; searchable chooser |
| Tab, Left/Right | Switch list/inspector pane |
| Up/Down, PgUp/PgDn, Home/End | Navigate lists and fields |
| Enter/Space | Assign or edit |
| Shift+Up/Down | Reorder components |
| `m`, `?` | Actions menu; full help |
| `s`, `S`, `a`, `x` | Save context; Save all; Activate; Save/activate/exit |
| `c`, `e`, `d`, `f` | Customize copy; edit shared; Duplicate; Full fork |
| `r`, `D`, `u`, `v` | Rename; Delete; dependents; use defaults |
| `R`, `E` | Reload; export draft catalog |
| `q`, Escape | Normal exit, or close current dialog |
| Ctrl+C | Immediate exit, discarding unsaved drafts |

`B`/`C` identify built-in/custom resources, `A` identifies the active theme, and `*` marks unsaved changes. Input dialogs consume their own keys; global shortcuts do not run while typing.

All workflows fit 60×18. Below 80 columns, Tab switches between full-width list and inspector. At 80 columns and above, the list uses 31 columns alongside the inspector. Preview uses one row through height 21, three rows from height 22, and the full 11-row CLI context only at 80×30 or larger. Resizing preserves drafts, selections, focus, and open dialogs; lists page by their actual visible height. Smaller terminals provide a best-effort layout or a resize notice.
