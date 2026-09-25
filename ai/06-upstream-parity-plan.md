# Upstream Parity Plan (shadcn/ui, 2026-09-24)

Source: https://ui.shadcn.com/docs/components (base variant, 64 components).
Research: `ai/research/batch-{a..e}.md` (per-component gap tables), `ai/research/new-components.md` (specs for new components).

## Findings

### 1. Missing components (6)

| Component | Purpose | Est. |
|---|---|---|
| Marker | Inline status, system note, or labeled separator in a conversation | 3 h |
| Bubble | Chat message surface: 7 variants, start/end alignment, grouping, reactions | 5 h |
| Message | Row layout around a message: avatar, alignment, header, footer | 3 h |
| Attachment | File/image card: upload states, sizes, actions (display only) | 7 h |
| Questionnaire | Multi-step fieldset form: radio/checkbox/freeform answers, skip, validation | 22 h |
| Message Scroller | Chat scroll container: stick-to-bottom, streaming follow, prepend anchoring | 32 h |

Naming only: upstream `radio-group` is our `radio.rs`. Ours only: `form.rs` (upstream moved forms to Field) and `sonner.rs` (upstream folds it into Toast).

### 2. Changed or incomplete components

352 gaps in the 58 shared components: 66 HIGH, 148 MED, 138 LOW. Grouped by kind:

- **Broken today (props accepted but ignored, or visibly wrong):** Tabs (no selection; all panels shown), Tooltip (always visible), Popover (open state ignored), Radio Group (group props ignored), Menubar and Navigation Menu (stateless; menus always open), Hover Card (delays ignored; card closes before the pointer reaches it), Collapsible (controlled toggle flips the wrong state), Checkbox (`default_checked` no-op), Switch (can't be forced off; the callback doesn't report the new value), Toggle Group (emits the clicked item, not the selection), Input OTP (`value` read once), Native Select (`value` does nothing in Yew 0.21), Combobox/Command (Empty always shown; selecting doesn't close), Dropdown/Menubar radio items (ignore the group value), Select (preset value shows no label), Sidebar (doesn't collapse from the provider), Avatar (size/shape classes don't match the CSS), AlertDialog/Dialog/Drawer (`open` only honored when `on_open_change` is also set), Resizable (2 panels only; `default_size` ignored), and missing CSS for Empty, Item, Kbd, Input OTP and Menubar.
- **New events and controlled state:** onValueChange emitting the full value (Accordion arrays, Toggle Group arrays), onCheckedChange(bool), onOpenChange everywhere, Slider onValueCommitted, Resizable onLayout, Calendar visible-month control.
- **Keyboard and a11y:** roving focus and arrow keys in Tabs, menus, Command, Combobox, Select, Calendar, Radio Group, Resizable handles; typeahead in menus and Select; dialogs labelled by their Title/Description; scroll lock; `aria_label` on Toggle and Toggle Group; triggers must not nest a `<button>` inside a `<button>`.
- **New sub-components and props:** AlertAction, AlertDialogMedia, AvatarBadge/Group, CardAction, BreadcrumbEllipsis, ButtonGroupSeparator/Text, Button icon sizes, the Field family (FieldSet, FieldLegend, orientation...), Item parts, InputGroupAddon (4 positions), KbdGroup, EmptyMedia, Sidebar collapsible modes and variants, submenus (Dropdown, Context, Menubar), CommandDialog, Chart tooltip and multi-series, DatePicker range, Carousel multi-item/vertical/swipe, Drawer swipe, Slider vertical.
- **API consolidation:** Toast becomes an imperative `Toaster` plus a `use_toast()` handle (add/close/update/promise); Sonner becomes a deprecated alias; the Form sub-components are deprecated in favor of Field.

## Phases

Each phase ends with fmt, clippy (`-D warnings`), `cargo test --workspace`, the browser suite, showcase screenshots at 700px and 1400px, and one commit per component or unit.

### Phase 1: Make existing components work (HIGH bugs), about 2 days
Fix every "broken today" item above. No new API surface except where a fix requires it (e.g. `open: Option<bool>`). Add a unit or browser test per fix.
- 1a Overlays and state: AlertDialog, Dialog, Drawer (controlled open), Collapsible, Popover (state and trigger), Tooltip (hover/focus/Escape, delays), Hover Card (delays, pointer bridge).
- 1b Form controls: Checkbox, Switch (`checked: Option<bool>`, `on_checked_change: Callback<bool>`), Radio Group (context-driven items), Toggle Group (array value), Toggle/Toggle Group `aria_label`, Input OTP (controlled value), Native Select (value via property), Select (item labels).
- 1c Composite widgets: Tabs (state, panels, aria-selected), Menubar and Navigation Menu (open state, click-outside, Escape), Combobox and Command (Empty visibility, select closes), Dropdown/Menubar radio items, Sidebar (collapse from the provider), Avatar CSS, and missing CSS for Empty/Item/Kbd/Input OTP/Menubar.

### Phase 2: Shared infrastructure, about 2 days
- `utils::floating`: anchor a portaled popup to its trigger (side, align, offsets, flip on collision). Used by Popover, Hover Card, Tooltip, Select, Dropdown, Context Menu, Menubar, Combobox.
- `hooks::use_roving_focus` / list navigation: arrow keys, Home/End, loop, typeahead, `aria-activedescendant` mode for input-driven lists.
- `hooks::use_scroll_lock` for modal overlays.
- Dialog labelling: generated ids that link Title and Description to `aria-labelledby` and `aria-describedby` (Dialog, AlertDialog, Sheet, Drawer).
- Trigger rendering that doesn't nest buttons (render the child's handlers onto the child).

### Phase 3: Keyboard and a11y rollout, about 2 days
Apply Phase 2 to Tabs, Radio Group, Dropdown, Context Menu, Menubar, Navigation Menu, Command, Combobox, Select, Calendar grid, Resizable handles, Accordion (fix the doc claim), and Toggle Group.

### Phase 4: Interface parity (MED sub-components, props, events), about 3 days
The sub-component and prop list above, including submenus, CommandDialog, the Field family, Item parts, Sidebar modes and variants, Slider vertical and onValueCommitted, Accordion array values, Button icon sizes, and Badge ghost/link CSS.

### Phase 5: Toast and Form consolidation, about 1 day
`Toaster` and `use_toast()` (add, close, update, promise, limit and stacking, types including loading). `sonner.rs` becomes deprecated re-exports and the Form sub-components are deprecated. CHANGELOG and migration notes.

### Phase 6: New components, about 9 days
In dependency order: Marker, Message, Bubble, Attachment (plus shared CSS utilities: sr-only, shimmer, edge fade, hidden scrollbar), Questionnaire, Message Scroller. Each gets a showcase page and tests.

### Phase 7: Advanced behavior, about 3 days
Chart tooltip and multi-series with a ChartConfig, Carousel multi-item/vertical/swipe, Resizable N panels with min/max, Drawer swipe-to-dismiss, DatePicker range mode, Calendar month/year dropdowns.

### Later (LOW gaps)
Tracked in the research tables; picked up opportunistically.

## Versioning
Phases 1 to 5 contain breaking changes (Switch callback type, Toggle Group value type, Toast API). Ship them as 0.2.0 before the first crates.io publish.
