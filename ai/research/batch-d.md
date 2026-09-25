# Batch D: upstream shadcn/ui (Base UI flavour) vs shadcn-rs

Date: 2026-09-24. Components: pagination, popover, progress, radio-group, resizable, scroll-area, select, separator, sheet, sidebar, skeleton, slider.

## Sources

- Firecrawl hit its shared rate limit on every call, so no page was scraped through it. The upstream docs were fetched as raw MDX instead, from `https://ui.shadcn.com/docs/components/base/<name>.md`. This is the same content the HTML page renders, and all 12 returned HTTP 200.
- Most of these pages link out to Base UI for their API Reference. For those, prop tables were pulled from `https://base-ui.com/react/components/<name>.md` (popover, progress, radio-group, scroll-area, select, separator, slider; sheet points to Base UI Dialog). Resizable points to react-resizable-panels v4, which has no fetched API table. For that component, gaps are taken from the shadcn page (orientation, `withHandle`, `defaultSize`, the v4 rename table including `onLayoutChange`, and "keyboard support" in the page description).
- React-only mechanics are ignored throughout. Upstream uses `render` (formerly `asChild`) to make a trigger, button or link render as another element. Yew has no equivalent, and it is noted here only once.
- Priority: **HIGH** means common usage breaks or a key feature is missing. **MED** means a noticeable feature or a11y gap. **LOW** means polish or niche.

---

## Pagination

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `href` optional | Prop | `PaginationLink` / `Previous` / `Next` take `href` like any anchor. Upstream examples always pass it, but it is not structurally required. | `href: AttrValue` is **required** on Link/Previous/Next. An SPA that pages with `onclick` only still has to pass a dummy href. | LOW |
| `text` prop on Previous/Next | Prop | `<PaginationPrevious text={...}/>` overrides the label (added for i18n/RTL). | We use `children` for the same thing, so it is equivalent in function. Only the name differs. | LOW |
| `size` on PaginationLink | Prop | Upstream PaginationLink passes Button `size` through (not shown in the docs prose; it is in the component source). | There is no size prop. | LOW |

Otherwise we match: Pagination/Content/Item/Link/Previous/Next/Ellipsis all exist, and `is_active` sets `aria-current="page"`. Our `disabled` is an extra feature upstream does not have.

## Popover

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Root state does nothing | Controlled/uncontrolled | Root has `open`, `defaultOpen` and `onOpenChange`, and Trigger toggles the popup. | `Popover` destructures `open: _, default_open: _, on_open_change: _` and **ignores them**. `PopoverTrigger` is a plain `<div>` with no click handler, and there is no context. The caller has to hold state and pass `open` and `on_close` straight to `PopoverContent`. | HIGH |
| No anchoring to the trigger | Behavior | The Positioner places the popup against the trigger using `side`, `align`, `sideOffset`, `alignOffset` and collision avoidance. | Content is portaled to `<body>` with `position:absolute` and a position class only. Nothing measures the trigger, so the popup does not sit next to it. | HIGH |
| `align` ignored | Prop | `align` is `start`/`center`/`end` on PopoverContent and is shown in the docs ("Align" example). | `align: Option<AttrValue>` is accepted and then discarded (`align: _`). | MED |
| PopoverHeader / PopoverTitle / PopoverDescription | Sub-component | Shown in the "Basic" example. Title and Description also label the dialog. | Missing. Content also has no `aria-labelledby` or `aria-describedby`. | MED |
| Focus management | A11y | Popup has `initialFocus` and `finalFocus`, focus moves into the popup and returns to the trigger, and `modal` defaults to `'trap-focus'`. | There is no focus move or restore. Escape and outside click are handled. | MED |
| `sideOffset`, `alignOffset`, collision props | Prop | These are Positioner props. | Missing. | LOW |
| `openOnHover`, `delay`, `closeDelay` | Prop | These are Trigger props. | Missing. | LOW |
| PopoverClose, Arrow, `onOpenChangeComplete` | Sub-component/Event | Base UI parts and callback. | Missing. | LOW |
| Trigger a11y attrs | A11y | Trigger is a button with `aria-expanded` and `aria-haspopup`. | The trigger is a `<div>` with no ARIA and no keyboard activation. | MED |

## Progress

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| ProgressLabel / ProgressValue | Sub-component | Composable label and formatted value ("Label" example). | Only a `show_label: bool` that renders a percentage. A custom label must be wired through `aria_labelledby` by hand. | MED |
| `min` | Prop | `min` defaults to 0. | Only `max` exists, and `min` is fixed at 0. | LOW |
| `format`, `locale`, `getAriaValueText`, `aria-valuetext` | Prop | These format the value and control the SR text. | Missing. | LOW |

Indeterminate (upstream `value={null}`) is covered by our `indeterminate` and `value: None`. The "Controlled" example is only a changing `value`, which we support.

## Radio Group (`radio.rs`)

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| RadioGroup props all ignored | Controlled/uncontrolled + events | RadioGroup has `value`, `defaultValue`, `onValueChange`, `name`, `disabled`, `required` and `readOnly`. Items get their checked state from the group. | `RadioGroup` destructures `name`, `value`, `default_value`, `disabled`, `required` and `onchange` as `_` and renders a bare `role="radiogroup"` div. There is no context, so `onchange: Callback<String>` **never fires** and the "Disabled" example (`disabled` on the group) does nothing. | HIGH |
| RadioGroupItem driven by group | Sub-component | `<RadioGroupItem value id/>` reads its checked state and name from the group. | `Radio` is a standalone `<input type=radio>`. The caller must set `name`, `checked` and `onchange` on every item. | HIGH |
| `default_checked` ignored | Uncontrolled | Uncontrolled default. | `Radio` destructures `default_checked: _`, so it has no effect. | MED |
| Arrow-key roving focus | Keyboard | Arrow keys move and select within the group, as one tab stop. | It works natively **only if** the caller gives every Radio the same `name`, because the group does not propagate it. | MED (follows from the first row) |
| `readOnly`, `form`, `inputRef` | Prop | These are Base UI Root props. | Missing (we have `node_ref` on Radio). | LOW |

Invalid state (`aria-invalid` on the item) is supported through `error` and `aria_invalid`.

## Resizable

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Only 2 panels | Behavior | Any number of panels and handles, including nested groups. | Group state is hard-coded to `vec![50.0, 50.0]`. Handles don't know their index, and dragging always emits `[p, 100-p]`. A third panel can't be resized. | HIGH |
| `default_size` ignored | Prop | `defaultSize` sets the initial layout. The docs use 25/75, 50/50 and similar splits. | Panels 0 and 1 always start at 50/50 because the context size overrides `default_size`. | HIGH |
| Keyboard resizing | Keyboard/A11y | The page is titled "...with keyboard support". The handle is a focusable separator driven by arrow keys (plus Home/End and Enter to collapse in react-resizable-panels). | The handle has `tabindex="0"` but **no keydown handler**, so it is mouse-only. | HIGH |
| `min_size` / `max_size` / `collapsible` ignored | Prop | These are per-panel constraints and collapse support. | They are accepted and discarded (`_`). A 10 to 90% clamp is hard-coded. | MED |
| `withHandle` | Prop | `<ResizableHandle withHandle/>` shows a grip ("Handle" example). | Missing. | MED |
| `onLayoutChange` (v4) / `onResize` | Event | This is a layout callback. | No callbacks at all. | MED |
| Touch/pointer drag | Behavior | react-resizable-panels uses pointer events. | Only mousedown/mousemove/mouseup are handled, so touch devices can't resize. | MED |
| Separator ARIA values | A11y | The handle exposes `aria-valuenow`, `aria-valuemin`, `aria-valuemax` and `aria-controls`. | Only `role="separator"` and `aria-orientation`. | LOW |
| Naming: `ResizablePanelGroup` | API | Upstream's wrapper is `ResizablePanelGroup` with `orientation`. | Ours is `Resizable` with `orientation`. A type alias would help people porting code. | LOW |
| Persistence (`autoSaveId`-style), imperative collapse/expand | Feature | Library features. | Missing. | LOW |

## Scroll Area

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `ScrollBar` sub-component | Sub-component | `<ScrollBar orientation="horizontal"/>` enables horizontal scrolling ("Horizontal" example). | There is no ScrollBar. We use `direction: Vertical/Horizontal/Both` on the root instead. It does the same job but is not API-compatible. | LOW |
| Custom overlay scrollbar/thumb | Behavior | Base UI renders its own Scrollbar, Thumb and Corner, which appear on hover or scroll. | We use native scrollbars styled with CSS. | LOW |
| Overflow-edge state (`overflowEdgeThreshold`, data attrs) | Prop | Used for edge fades. | Roughly covered by `show_shadow`. | LOW |

No functional gap for common usage.

## Select

We have a native `Select` wrapping `<select>`, and a compound `SelectAdvanced` with Trigger, Value, Content, Item, Group, Label and Separator. The gaps below are for `SelectAdvanced`.

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Label for controlled/default value | Behavior | `items` on Root (`{label, value}[]`) lets `SelectValue` show the label for `value` or `defaultValue`. Every docs example passes `items`. | Labels are only recorded when an item is clicked. With `value` or `default_value` set, `SelectValue` shows the placeholder or nothing until the user picks something. | HIGH |
| Keyboard navigation in the list | Keyboard/A11y | Arrow keys move the highlight, Home/End work, and typeahead jumps to matching items. The list uses `highlightItemOnHover`. | Only Enter/Space on an already-focused item. There is no arrow navigation, typeahead, highlight or `aria-activedescendant`. | HIGH |
| Focus into and out of the popup | A11y | Opening moves focus to the selected item, and closing returns it to the trigger (`finalFocus`). | Neither happens. | MED |
| Controlled `open` toggling | Controlled | `open` and `onOpenChange` work as a pair. | `toggle_open` flips `internal_open`, not the controlled value. In controlled mode the emitted value can be wrong. | MED |
| Multiple as an array | Prop/Event | With `multiple`, `value` and `onValueChange` use an array. | Controlled `value` is a single `AttrValue`. `on_value_change` emits a **comma-joined string**, which breaks on values that contain commas. | MED |
| Form integration | Prop | Root has `name`, `required`, `form` and `autoComplete`, backed by a hidden input. | `SelectAdvanced` has no `name` or hidden input, so it doesn't submit with a form. The native `Select` does. | MED |
| Positioning props | Prop | `alignItemWithTrigger` (default true, shown in docs), `side`, `align`, `sideOffset`. | Missing. Content is absolutely positioned under the trigger. | MED |
| `aria-invalid` on SelectTrigger | Prop/A11y | "Invalid" example. | `SelectTrigger` has no `aria_invalid` or error prop. (The native `Select` has `error`.) | LOW |
| SelectScrollUpButton / SelectScrollDownButton | Sub-component | Used for long lists ("Scrollable" example). | Missing. The list scrolls natively. | LOW |
| `readOnly`, `onOpenChangeComplete`, `isItemEqualToValue`, `modal` | Prop/Event | Base UI Root. | Missing. | LOW |

Extra on our side: a `searchable` filter input, which is a Combobox-like feature upstream does not have on Select.

## Separator

No meaningful gaps. One note: our `decorative` defaults to `true` (`role="none"`), while Base UI Separator always renders `role="separator"`. That is a LOW semantic default difference inherited from the Radix-era shadcn API.

## Sheet

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `side` on SheetContent | Prop | The docs say "Use the `side` prop on `SheetContent`" (top, right, bottom, left). | `side` lives on the `Sheet` root. `SheetContent`'s `side` is **ignored inside a Sheet** because the context always overrides it, so upstream-style code silently gets the root default. | MED |
| Built-in close button and `showCloseButton` | Prop/Sub-part | SheetContent renders an X close button by default, and `showCloseButton={false}` hides it. | There is no built-in close button. Users must add `SheetClose` themselves. | MED |
| Title/Description labelling | A11y | Dialog Title and Description label the popup (`aria-labelledby`, `aria-describedby`). | `SheetTitle` is a plain `<h2>` and the content has no `aria-labelledby` or `aria-describedby`. | MED |
| Controlled detection | Controlled | `open` plus `onOpenChange`. | The code treats the sheet as controlled only when `on_open_change` is set. Passing `open=true` without a callback is ignored. | LOW |
| Scroll lock, `modal`, `initialFocus`/`finalFocus`, `onOpenChangeComplete` | Behavior/Prop | Base UI Dialog. | Focus trap and focus restore **are** implemented (`use_focus_trap`). There is no body scroll lock and none of the other props. | LOW |

## Sidebar

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Sidebar doesn't follow provider state | Behavior | `SidebarTrigger` and `SidebarRail` toggle the provider, and `Sidebar` collapses automatically (`data-state`, `data-collapsible`). | `Sidebar` reads only `is_mobile` from the context. Collapse is driven by its own `collapsed` prop. The provider adds `sidebar-provider-open`/`-closed` classes, but no CSS uses them. The caller has to pass `collapsed={!use_sidebar().open}` by hand. | HIGH |
| `collapsible` modes | Prop | `offcanvas` (default), `icon` and `none`. | There is one generic `collapsed` style, with no icon-rail mode and no offcanvas/none choice. | HIGH |
| `variant` | Prop | `sidebar`, `floating` and `inset`. | Missing. `SidebarInset` exists, but there is no inset or floating variant on `Sidebar`. | MED |
| `side` | Prop | `left` or `right`. | Missing (left only). | MED |
| Separate mobile state | Controlled | `useSidebar` returns `state`, `open`, `setOpen`, `openMobile`, `setOpenMobile`, `isMobile` and `toggleSidebar`. On mobile the sidebar renders inside a Sheet. | The context has a single `open`, `is_mobile`, `toggle` and `set_open`. There is no `openMobile`/`state`, and mobile is a backdrop plus aside, not a Sheet (so no focus trap or Escape). | MED |
| Keyboard shortcut Cmd/Ctrl+B | Keyboard | The provider toggles on `cmd+b` / `ctrl+b`. | Missing. | MED |
| SidebarMenuSub / SidebarMenuSubItem / SidebarMenuSubButton | Sub-component | Nested submenus, used in the main demo. | Missing. | MED |
| SidebarMenuAction (`showOnHover`) | Sub-component | Per-item action button. | Missing. | MED |
| SidebarGroupAction | Sub-component | Group header action. | Missing. | LOW |
| SidebarMenuBadge | Sub-component | Count badge. | Missing. | LOW |
| SidebarMenuSkeleton (`showIcon`) | Sub-component | Loading rows. | Missing. | LOW |
| SidebarInput | Sub-component | Search input styled for the sidebar. | Missing. | LOW |
| SidebarMenuButton `size`, `variant`, `tooltip` | Prop | `size="lg"` is used in the demo. `tooltip` shows the label when collapsed to icons. | Only `href`, `active`, `disabled` and `onclick`. There is no tooltip, which matters once icon mode exists. | MED |
| Width CSS vars | Styling hook | `--sidebar-width` and `--sidebar-width-mobile` on the provider. | Not verified to be exposed. Styling only. | LOW |

Already present: SidebarProvider (with `default_open`, `open`, `on_open_change`), `use_sidebar`, SidebarTrigger, SidebarRail, SidebarInset, Header/Content/Footer, Group/GroupLabel/GroupContent, Menu/MenuItem/MenuButton, and Separator.

## Skeleton

No meaningful gaps. Upstream is a styled `<div>` that takes only `className`. We add `width`, `height`, `shape` and `animate`. LOW note: we put `aria-live="polite"` and `aria-busy` on **every** skeleton, which upstream does not do. Many skeletons on one page make many live regions. Consider putting that on the container instead.

## Slider

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `orientation="vertical"` | Prop | "Vertical" example. | Missing (horizontal only). | MED |
| `onValueCommitted` | Event | Fires once at the end of a drag or keypress. | Missing. We only have `onchange`, which fires on every `input` event (equivalent to `onValueChange`). | MED |
| Thumb collision / ordering | Behavior | `thumbCollisionBehavior` (`push` by default in the root table, plus `swap` and `none`) and `minStepsBetweenValues`. | Thumbs are independent stacked `<input type=range>` elements. They can cross, and the emitted `Vec` can come out unordered (for example `[80, 20]`). | MED |
| Track press moves the nearest thumb | Behavior | A track press picks the closest thumb. | With stacked native inputs, a track click goes to whichever input is on top, not the nearest thumb. This is inferred from the structure and not tested in a browser. | LOW |
| `largeStep` (PageUp/PageDown, Shift+Arrow) | Keyboard | Default 10. | Native range inputs give browser-defined PageUp/PageDown behavior, with no prop to set it. | LOW |
| Per-thumb labels (`getAriaLabel`) and `getAriaValueText` / `format` | A11y | Each thumb gets its own label and value text. | Every thumb shares the same `aria_label`. | LOW |
| `name` / `form` | Prop | Form submission. | No `name` prop. | LOW |

Supported: `value`/`default_value` (controlled and uncontrolled), `min`, `max`, `step`, `disabled`, range and multi-thumb (any number of values in the `Vec`), and native arrow/Home/End keys.

---

## Totals

| Component | HIGH | MED | LOW |
|---|---|---|---|
| Pagination | 0 | 0 | 3 |
| Popover | 2 | 4 | 3 |
| Progress | 0 | 1 | 2 |
| Radio Group | 2 | 2 | 1 |
| Resizable | 3 | 4 | 3 |
| Scroll Area | 0 | 0 | 3 |
| Select | 2 | 5 | 3 |
| Separator | 0 | 0 | 0 (1 note) |
| Sheet | 0 | 3 | 2 |
| Sidebar | 2 | 7 | 5 |
| Skeleton | 0 | 0 | 0 (1 note) |
| Slider | 0 | 3 | 4 |
| **Total** | **11** | **29** | **29** |
