# Upstream gap analysis — batch B

Compared on 2026-09-24 against `https://ui.shadcn.com/docs/components/base/<name>` (Base UI flavour).
All 12 pages loaded (HTTP 200) via Firecrawl. Firecrawl's markdown drops code blocks, so code samples were read from the `.md` versions of the same pages (`.../base/<name>.md`).
Rust sources: `shadcn-rs/src/components/<name>.rs`.

**Note that applies to all components:** upstream uses Base UI's `render={<Button/>}` prop (the old `asChild`) on Trigger, Close and similar parts. Our Trigger and Close components wrap their children in their own `<div>` or `<button>`, so a `<Button>` inside `CollapsibleTrigger` becomes a button nested in a button. This is React polymorphism and is not rated below.

Priority key: **HIGH** = common usage breaks or a key feature is missing · **MED** = a documented feature or a11y gap · **LOW** = nice-to-have or naming.

---

## Chart

Upstream is a composable layer over Recharts: `ChartContainer` + `ChartConfig`, `ChartTooltip`/`ChartTooltipContent`, `ChartLegend`/`ChartLegendContent`. Ours is one `Chart` component that renders its own SVG from `Vec<ChartData{label,value}>`.

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Multi-series data + ChartConfig | Props / sub-component | Data rows of any shape (`{month, desktop, mobile}`) mapped by `dataKey`. `ChartConfig` maps each series key to `label`, `icon` and `color`/`theme` | Only single-series `ChartData { label, value }`. No per-series config | HIGH |
| Tooltip | Sub-component | `ChartTooltip` + `ChartTooltipContent` with `labelKey`, `nameKey`, `indicator` (`dot`/`line`/`dashed`), `hideLabel`, `hideIndicator` | No tooltip or hover feedback at all | HIGH |
| Legend content | Sub-component / props | `ChartLegend` + `ChartLegendContent` with `nameKey`, colors and icons from config | `show_legend: bool` only, labels come from data points | MED |
| Theme color tokens | Props | Colors come from `var(--chart-N)` / `var(--color-KEY)`, which follow light/dark theme | `colors: Option<Vec<AttrValue>>` plus a hard-coded `DEFAULT_COLORS` (no `var(--chart-*)`) | MED |
| Responsive container | Behavior | `ChartContainer` is responsive and needs `min-h-*`/`aspect-*` | Fixed `width`/`height` attributes (500x300) with a viewBox | MED |
| `accessibilityLayer` | A11y | Keyboard access and screen-reader support for data points | `role="img"` plus a summary `aria-label` only | MED |
| Chart variants | Variants | Library includes stacked, horizontal bar, radar, radial, interactive (range toggle), label and grid options | Bar, Line, Area, Pie, Donut. No stacked, horizontal, radar or radial | MED |

## Checkbox

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Uncontrolled `defaultChecked` | Controlled/uncontrolled | "Use `defaultChecked` for uncontrolled checkboxes" | `default_checked` is declared but destructured as `_` and never used (no-op). `checked` is always sent to the DOM, so the checkbox is effectively controlled-only | HIGH |
| `onCheckedChange(bool)` | Event | `checked` + `onCheckedChange` is the documented controlled pattern | Only a raw `onchange: Callback<Event>`. Callers have to read the input's `checked` themselves | MED |
| Group / parent checkbox | Sub-component | Base UI `CheckboxGroup` and the `parent` prop (select-all) | None. Only `indeterminate`, set by hand | LOW |
| `readOnly` | Prop | Base UI checkbox supports `readOnly` | Not supported | LOW |

## Collapsible

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Controlled toggle | Controlled/uncontrolled | `open` + `onOpenChange` | `toggle` flips `internal_open`, not the effective `is_open`. When `open=Some(x)` is passed, the emitted value can be wrong (e.g. `open=Some(true)` with internal `false` emits `true` again), so the controlled trigger can't close the panel | HIGH |
| Trigger/content ARIA link | A11y | Base UI links the trigger to the panel (`aria-controls`/id) | Only `aria-expanded`. No ids and no `aria-controls` | MED |
| `disabled` | Prop | Base UI `Collapsible` has `disabled` | Not supported | LOW |
| Keep-mounted / animation | Behavior | Panel supports `keepMounted`/`hiddenUntilFound` and height CSS vars for animation | Content unmounts when closed, so there is no height animation | LOW |

## Combobox

Upstream (Base UI) is item-driven: `<Combobox items value onValueChange multiple>` with `ComboboxInput`, `ComboboxContent`, `ComboboxList`, `ComboboxItem`, `ComboboxEmpty`, `ComboboxGroup`, `ComboboxLabel`, `ComboboxCollection`, `ComboboxSeparator`, `ComboboxChips`/`ComboboxChip`/`ComboboxChipsInput`, `ComboboxValue`, `ComboboxTrigger`.

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Root selection state | Event / controlled | `value`/`defaultValue`/`onValueChange` on `Combobox` | No value state on the root. Each item takes `selected: bool` + `onclick`, and the caller does all bookkeeping | HIGH |
| Selecting commits and closes | Behavior | Choosing an item sets the value, fills the input and closes the popup | Item click only fires the caller's `onclick`. It doesn't close the popup or update the input or trigger | HIGH |
| Keyboard navigation | Behavior / a11y | Arrow Up/Down moves the highlight while focus stays in the input (`aria-activedescendant`), Enter selects, Home/End work | No arrow handling. Items are tab stops that react only to Enter/Space. No `aria-activedescendant`. Input has no `aria-controls`/`aria-expanded` | HIGH |
| `ComboboxEmpty` only when there are no matches | Behavior | Empty state shows only when the filter returns nothing | `ComboboxEmpty` always renders | HIGH |
| Controlled `open` | Controlled/uncontrolled | `open`/`onOpenChange` | `open` only seeds `use_state` at mount. Later prop changes are ignored | MED |
| Multiple selection + chips | Props / sub-components | `multiple`, `ComboboxChips`, `ComboboxChip`, `ComboboxChipsInput` | Not supported | MED |
| `ComboboxValue` | Sub-component | Renders the selected value(s) in a trigger or popup | Missing | MED |
| `showClear` | Prop | Built-in clear button | Missing | MED |
| `disabled` / `aria-invalid` on root and input | Props | Documented Disabled and Invalid examples | Input has neither `disabled` nor `aria-invalid`. Only items have `disabled` | MED |
| `autoHighlight` | Prop | Highlights the first match while filtering | Missing (no highlight concept) | LOW |
| `ComboboxList`, `ComboboxLabel`, `ComboboxCollection` | Sub-components | List wrapper, group label, per-group collections | Missing. `ComboboxGroup` has a `heading` prop instead of a Label child | LOW |
| `showTrigger`, InputGroup addon in input | Props | Input can show or hide its trigger chevron and host an `InputGroupAddon` | Missing | LOW |

(We have extras upstream doesn't document: `allow_create`/`on_create`, `max_visible_items`, `keywords`.)

## Command

Upstream wraps `cmdk`: `Command`, `CommandDialog`, `CommandInput`, `CommandList`, `CommandEmpty`, `CommandGroup`, `CommandItem`, `CommandSeparator`, `CommandShortcut`.

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `CommandDialog` | Sub-component | Every doc example ("Basic", "Shortcuts", "Groups", "Scrollable") uses `<CommandDialog open onOpenChange>` for the ⌘K palette | Missing. Users have to hand-compose it with `Dialog` | HIGH |
| Highlight + arrow navigation | Behavior / a11y | cmdk keeps a selected item (`data-selected`/`aria-selected`), Arrow Up/Down/Home/End move it (optional `loop`), Enter runs it, and focus stays in the input | No highlight state or arrow keys. Items are tab stops that react only to Enter/Space. The input hard-codes `aria-expanded="true"` with no `aria-activedescendant` | HIGH |
| `CommandEmpty` only when there are no results | Behavior | Renders only when zero items match | Always renders | HIGH |
| `onSelect(value)` on items | Event | `CommandItem onSelect` receives the item value, from both keyboard and mouse | Only `onclick: Callback<MouseEvent>` | MED |
| Filtering without `value` / `keywords` | Behavior | cmdk infers value from text content and supports a `keywords` prop | Items without `value` are always shown during a search. No `keywords` on `CommandItem` | MED |
| Groups hide when empty | Behavior | Group and heading disappear when none of its items match | `CommandGroup` always renders its heading | MED |
| Root `value`/`onValueChange`, `shouldFilter`, `filter`, `loop` | Props | cmdk root API | Missing. Filter is a fixed case-insensitive substring match | LOW |
| Input search icon / root role | A11y / visual | Input shows a search icon. cmdk uses listbox/combobox semantics | No icon. Root is `role="application"` (discouraged) | LOW |

## Context Menu

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Submenus | Sub-components | `ContextMenuSub` + `ContextMenuSubTrigger` + `ContextMenuSubContent` (hover/ArrowRight opens, ArrowLeft closes) | `ContextMenuSub` is a plain `<div>`. SubTrigger and SubContent are missing | HIGH |
| Keyboard navigation / typeahead | Behavior / a11y | Arrow Up/Down roving focus, Home/End, typeahead by label, focus moves into the menu on open | None. Items are tab stops with Enter/Space only. Escape closes | HIGH |
| `ContextMenuShortcut` | Sub-component | Right-aligned key hint | Missing | MED |
| `variant="destructive"` on items | Variant | Documented "Destructive" example | No `variant` prop | MED |
| Root `open`/`onOpenChange` | Event / controlled | Base UI ContextMenu root supports `onOpenChange` (and controlled `open`) | Root has only `class`/`children`. State is internal | MED |
| Viewport collision | Behavior | Popup is positioned with collision avoidance | `position: fixed` at raw `clientX/Y`, so it can overflow the viewport edges | MED |
| `ContextMenuGroup` | Sub-component | `role=group` wrapper used in almost every example | Missing | LOW |
| Keyboard open | A11y | Opens with the Menu key or Shift+F10 on the focused trigger | Opens on the `contextmenu` event only (the Menu key may fire it natively). Trigger is not focusable | LOW |
| `side`/`align` on content and subcontent | Props | "Sides" example sets submenu placement | Missing (depends on submenus) | LOW |
| CheckboxItem uncontrolled / naming | Controlled / event | `defaultChecked`, `onCheckedChange`. RadioGroup uses `onValueChange` | `checked` controlled only. Callbacks are named `onchange` (not `on_checked_change`/`on_value_change`, unlike our DropdownMenu) | LOW |

## Data Table

Upstream is a **guide** (TanStack Table v9 + `<Table/>`), not a component. Ours is a concrete generic `DataTable<T>` with sort, global filter, selection and pagination. The gaps below are features the guide builds that our component can't do.

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Column visibility toggle | Feature | "Visibility" section: a DropdownMenu of checkbox items to hide or show columns | Not supported | MED |
| Custom header rendering | Props | Headers can be components (sortable button with icon, select-all checkbox, `DataTableColumnHeader` with sort and hide menu) | `header: AttrValue` text only | MED |
| Controlled sort, filter and page state | Events / controlled | Table state (sorting, filters, pagination) is caller-owned, so server-side data is possible | Sort, filter and page are internal. No `on_sort_change`/`on_page_change`/`on_filter_change`. Only selection is controllable | MED |
| Selection identity | Behavior | Rows selected by row id (`getRowId`) | Selection is `Vec<usize>` indices into `data`, so it goes stale when data is re-ordered or replaced | MED |
| Page-size selector and "n of m row(s) selected" | Feature | `DataTablePagination` has a rows-per-page select and a selection count | Fixed `rows_per_page`, "Page x of y" plus Prev/Next only | LOW |
| Per-column filter | Feature | Filter input bound to one column (`email`) | One global search across `searchable` columns (arguably fine) | LOW |
| Header select-all indeterminate | A11y / visual | Header checkbox shows the indeterminate state for partial selection | Checked/unchecked only | LOW |

## Date Picker

Upstream: "there is no `DatePicker` root component". It is a composition of `Popover` + `Calendar` (react-day-picker). Ours is a monolithic `DatePicker` (button + inline popover + `Calendar`).

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Range picker | Mode | "Range Picker" example uses `mode="range"` with `numberOfMonths={2}` and a `DateRange` value | `DatePicker` hard-codes `CalendarMode::Single` (`Calendar` itself supports `Range`) | HIGH |
| Dropdown caption (date of birth) | Prop | `captionLayout="dropdown"` for month and year selects | Calendar has no caption-layout option | MED |
| Typed-input variant | Variant / behavior | "Input" example: a text input parses the typed date, ArrowDown opens the calendar, and the selected `month` follows | Button-only trigger. No text entry | MED |
| Controlled `open` | Controlled | Examples drive `open`/`onOpenChange` (e.g. close on select) | Open state is internal (`use_toggle`). No `open`/`on_open_change` | MED |
| Popover focus and labelling | A11y | Popover moves focus into the calendar and returns it on close. Labelled by the field | `role="dialog"` div without a label. Focus is not moved into the calendar and not restored | MED |
| `number_of_months`, `align`/`side_offset` | Props | Passed through to Calendar and Popover | Not exposed on `DatePicker` | LOW |
| Time picker / natural-language examples | Examples | Time input next to the calendar, and `chrono-node` parsing | Not supported (composition recipes, not core) | LOW |

## Dialog

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Accessible name/description | A11y | Popup is automatically labelled by `DialogTitle` and described by `DialogDescription` | `role="dialog" aria-modal` without `aria-labelledby`/`aria-describedby`. Title and Description have no ids | HIGH |
| Built-in close (X) button + `showCloseButton` | Prop / sub-component | `DialogContent` renders an X by default. `showCloseButton={false}` hides it | No built-in close button | MED |
| Controlled `open` without callback | Controlled | `open` is honored whenever it is passed | Controlled mode only applies when `on_open_change` is set. `open` alone is ignored | MED |
| Body scroll lock / inert background | Behavior | Modal dialog locks page scroll and makes content underneath inert | No scroll lock (none in dialog.rs or hooks) | MED |
| `modal`, initial/final focus, nested dialogs | Props / behavior | Base UI Dialog: `modal`, `initialFocus`, `finalFocus`, nested stacking | Focus trap + restore exist. No `modal=false`, no initial-focus target, no nested handling | LOW |

## Direction

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Components don't consume direction | Behavior | RTL examples on every component page expect menus and popovers to flip (`side="inline-end"`) | `use_direction` isn't used anywhere outside direction.rs | MED |
| Prop name | Props | `<DirectionProvider direction="rtl">` + `useDirection()` | Prop is `dir` (`Direction::Rtl`). Hook matches | LOW |
| Extra wrapper element | Behavior | Provider is context only. Docs set `dir` on `<html>` | Renders a wrapper `<div dir=...>`, which can affect layout | LOW |

## Drawer

Upstream base drawer moved from Vaul to **Base UI Drawer**.

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Swipe-to-dismiss gestures | Behavior | Drag or swipe along `swipeDirection` to close. `data-swiping` state | No pointer or touch handling. It behaves like a side sheet | HIGH |
| Direction naming and default | Props | `swipeDirection`: `up`/`right`/`down`/`left`, default bottom (`down`) | `side: Position`, default `Right` | MED |
| Snap points | Props / events | `snapPoints`, controlled `snapPoint` + `onSnapPointChange`, `data-expanded` | Missing | MED |
| Body scroll lock | Behavior | Modal drawer locks scroll | No scroll lock | MED |
| Controlled `open` without callback | Controlled | `open` honored directly. Also `onOpenChangeComplete` | Same issue as Dialog: controlled only when `on_open_change` is set | MED |
| Swipe handle | Prop / sub-component | `showSwipeHandle` on `Drawer`. `DrawerSwipeHandle`, `DrawerPortal`, `DrawerOverlay` exported | Missing | LOW |
| Non-modal modes | Props | `modal={false}`, `modal="trap-focus"`, `disablePointerDismissal` | Always modal (overlay-click can be disabled via `close_on_overlay_click`) | LOW |
| Nested drawers | Behavior | Parent drawers stay mounted and stack behind the front one (`data-nested-drawer-open`) | No nesting support | LOW |

## Dropdown Menu

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Submenus | Sub-components | `DropdownMenuSub` + `DropdownMenuSubTrigger` + `DropdownMenuSubContent` (used in the Submenu and Complex examples) | `DropdownMenuSub` ignores its `open`/`on_open_change` and renders a `<div role="menu">`. SubTrigger and SubContent are missing | HIGH |
| RadioGroup value wiring | Controlled / event | `<DropdownMenuRadioGroup value onValueChange>` with `<DropdownMenuRadioItem value>` children | RadioGroup provides a `DropdownMenuRadioContext`, but `DropdownMenuRadioItem` never reads it. It uses its own `selected` and `on_select`, so the group's `value`/`on_value_change` do nothing | HIGH |
| Keyboard navigation / typeahead | Behavior / a11y | Arrow Up/Down roving focus, Home/End, typeahead, ArrowDown on the trigger opens the menu and focuses the first item | Not implemented (the doc comment claims "Arrow keys" support, but there is no Arrow handling). Items are tab stops with Enter/Space | HIGH |
| `DropdownMenuShortcut` | Sub-component | Key hint in items | Missing | MED |
| `variant="destructive"` on items | Variant | "Destructive" example | No `variant` prop | MED |
| Content placement | Props | `side`, `align`, `sideOffset`, `alignOffset` on `DropdownMenuContent` | No placement props | MED |
| `DropdownMenuGroup` / `DropdownMenuPortal` | Sub-components | Group wrapper used in most examples. Portal wraps SubContent | Missing (Content already renders in `Portal`) | LOW |
| CheckboxItem uncontrolled; controlled-root quirk | Controlled | `defaultChecked`. Root `open` is honored whenever it is passed | `checked` controlled only. Root is controlled only when `on_open_change` is set | LOW |

---

## Totals

| Component | HIGH | MED | LOW |
|---|---|---|---|
| Chart | 2 | 5 | 0 |
| Checkbox | 1 | 1 | 2 |
| Collapsible | 1 | 1 | 2 |
| Combobox | 4 | 5 | 3 |
| Command | 3 | 3 | 2 |
| Context Menu | 2 | 4 | 4 |
| Data Table | 0 | 4 | 3 |
| Date Picker | 1 | 4 | 2 |
| Dialog | 1 | 3 | 1 |
| Direction | 0 | 1 | 2 |
| Drawer | 1 | 4 | 3 |
| Dropdown Menu | 3 | 3 | 2 |
| **Total** | **19** | **38** | **26** |
