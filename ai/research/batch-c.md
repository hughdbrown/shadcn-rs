# Upstream gap analysis — batch C

Components: empty, field, hover-card, input, input-group, input-otp, item, kbd, label, menubar, native-select, navigation-menu.

Date: 2026-09-24. Upstream: `https://ui.shadcn.com/docs/components/base/<name>` (Base UI flavour).

## Method and sources

- I tried Firecrawl `firecrawl_scrape` first. The `empty` page loaded, but every code block came back empty. The next three requests hit the rate limit (HTTP 429).
- So all 12 pages were fetched live with `curl` from the markdown version of the same URLs (`https://ui.shadcn.com/docs/components/base/<name>.md`). That source has the same content, including the example code. All 12 returned HTTP 200.
- Hover Card, Label, Menubar and Navigation Menu send their API reference to Base UI, so I also read the Base UI API tables: `base-ui.com/react/components/{preview-card,menubar,menu,navigation-menu}.md`. Base UI's Hover Card is now called **Preview Card**. Base UI's `label.md` returned 404, so the Label comparison uses only the shadcn page.
- I read each of our files in `shadcn-rs/src/components/<name>.rs` in full, plus the matching rules in `shadcn-rs/styles/components.css`.
- React-only mechanics are left out: `asChild`, the `render={<a/>}` polymorphism, `nativeButton`, `data-slot`, and `className` as a function. **Noted once here:** upstream's usual way to make `Item`, `NavigationMenuLink`, `HoverCardTrigger`, `TooltipTrigger` and `DropdownMenuTrigger` render as a link or button is `render`. Yew has no equivalent, so a Yew port needs explicit `href` props or `as`-style enum props instead. Where that choice matters for behaviour, I list it as a gap below.

Priority key: **HIGH** = common usage breaks or a key feature is missing. **MED** = noticeable gap with a workaround. **LOW** = polish, naming, or rarely used.

---

## Cross-cutting finding

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Stylesheet has no rules for several components | Styling / behaviour | Every component ships with styles | `styles/*.css` has **no** rules at all for `.empty*`, `.item*`, `.kbd*`, `.input-otp*` or `.menubar*`. `.hover-card-{top,bottom,left,right,...}` side classes and `.field-help` / `.field-error-message` also have no rules. As a result, menubar content is never hidden (see Menubar) and hover-card side classes do nothing. | MED |

---

## Empty

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `EmptyMedia` with `variant: "default" \| "icon"` | Sub-component / variant | Media slot takes any node: an icon component (`variant="icon"` draws the icon tile), an `Avatar`, or an avatar group | `icon: Option<AttrValue>` is **text only**. You can't pass a Yew icon component or `<Avatar>`. No icon/default variant. | HIGH |
| `EmptyHeader`, `EmptyTitle`, `EmptyDescription`, `EmptyContent` | Sub-components | Composable tree `Empty > EmptyHeader > (Media, Title, Description)` + `EmptyContent` (buttons, an `InputGroup`, links) | Monolithic props (`title`, `description`, `action`). `children` replaces title/description, but there are no styled parts to compose with. | MED |
| Rich title/description | Props | Title and description take JSX, e.g. a "Contact support" link inside the description | `title` and `description` are `AttrValue`, so they are plain text | LOW |
| Live-region role | A11y | Plain `div`, no role | Root has `role="status"`, which turns it into a polite live region. Screen readers announce it whenever it mounts or changes. | LOW |

## Field

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `FieldSet` + `FieldLegend` (`variant: "legend" \| "label"`) | Sub-components | Semantic `<fieldset>`/`<legend>` grouping, used for radio and checkbox groups. The `label` variant is for nested sets. | Missing. No `<fieldset>` anywhere in the Field family. | HIGH |
| `orientation: "vertical" \| "horizontal" \| "responsive"` | Prop | Horizontal is used for Checkbox, Switch and Radio rows and for inline input+button. Responsive uses container queries. | Missing. The layout is always label, then control, then help, stacked vertically. You can't put a checkbox before its label. | HIGH |
| `FieldLabel`, `FieldDescription`, `FieldError` as children | Sub-components | Free ordering. For example, the description can sit **before** the input (password example). A `FieldLabel` can wrap a whole `Field` for the "choice card" pattern. | Only `label`, `help_text` and `error` props, in a fixed order. There are no standalone parts. | MED |
| `FieldGroup` | Sub-component | Stacks fields, sets spacing, hosts the container query for responsive orientation | Missing | MED |
| `FieldContent` + `FieldTitle` | Sub-components | Column that groups label and description beside a checkbox or switch. `FieldTitle` gives label styling without a `<label>`. | Missing | MED |
| `FieldError errors=[...]` | Prop | Takes an array of `{message}`, or Standard Schema issues. Several messages render as a list. | `error: Option<AttrValue>`, one string only | MED |
| `role="group"` on `Field` | A11y | `Field` outputs `role="group"` so nested controls pick up the label | The root is a plain `<div>` with no role | MED |
| Description and error linked to the control | A11y | Upstream pairs `aria-invalid` on the control with Field state | We generate `{id}-help` and `{id}-error` ids, but nothing sets `aria-describedby` on the child control. There is no context to pass the ids down, so the user has to wire them by hand. | MED |
| `FieldSeparator` (optional inline text, e.g. "Or continue with") | Sub-component | Divider inside `FieldGroup` | Missing | LOW |
| `data-disabled` field state | Prop / state | `<Field data-disabled>` styles the whole block as disabled | No disabled prop on `Field` | LOW |
| Error class clash | Validation state | `data-invalid` switches the block into an error state | Rust puts `field-error` on the **container**, but in CSS `.field-error` is the small red *message* style, so the whole field turns into red small text. `field-help` and `field-error-message` have no CSS. | LOW |

## Hover Card

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Open and close delays | Props / behaviour | `delay` (Base UI default 600 ms) and `closeDelay` (default 300 ms) on `HoverCardTrigger`. The docs demo uses `delay={10} closeDelay={100}`. | `open_delay` and `close_delay` exist on `HoverCard`, but they are destructured to `_` and **ignored**. The card opens and closes instantly on mouseenter/mouseleave. | HIGH |
| Pointer can reach the card | Behaviour | The card stays open while the pointer moves from the trigger into the popup, so links inside the card can be clicked | The trigger's `mouseleave` closes it immediately, and the content has no enter/leave handlers. With no close delay, you can't move the pointer into the card. | HIGH |
| Anchored positioning | Behaviour | Positioner anchors to the trigger. `side` (default bottom), `align` (default center), `sideOffset`, `alignOffset`, collision avoidance and padding, `sticky`. | Content is rendered through `Portal` (which does no positioning) with `position:absolute` and a `hover-card-<side>` class that has no CSS rule. It is not placed relative to the trigger. | HIGH |
| `sideOffset`, `alignOffset`, collision handling | Props | Listed above | Missing | MED |
| Controlled mode | Controlled/uncontrolled | `open` + `onOpenChange` + `defaultOpen` | `open: bool` only counts when `on_open_change` is also set, because controlled mode is detected by whether the callback exists. You can't have a read-only controlled `open`. | MED |
| Trigger is the element itself | Behaviour / a11y | The trigger *is* the link or button (via `render`) | Always wraps children in `<div tabindex="0">`. Wrapping a link gives **two tab stops**. | MED |
| `align` type | Prop | `"start" \| "center" \| "end"` | Free-form `Option<AttrValue>` turned into a class that has no CSS | LOW |
| `onOpenChangeComplete` | Event | Fires after the open/close animation | Missing | LOW |
| Arrow default | Prop | No arrow in the shadcn styling (Base UI has an optional `Arrow` part) | `show_arrow` defaults to `true` | LOW |
| Popup semantics | A11y | "For sighted users". The popup has no landmark role. | `role="region" aria-label="Additional information"` adds a landmark for screen-reader users on every hover | LOW |

## Input

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `default_value` | Controlled/uncontrolled | Native `defaultValue` works | The prop exists but is destructured to `_` and **ignored**. An uncontrolled input can't have an initial value. | MED |
| Native attribute pass-through | Props | All `<input>` props pass through: `min`, `max`, `step`, `minLength`, `maxLength`, `pattern`, `inputMode`, `accept`, `multiple` (file example), `autoFocus`, `title`, `form`, `aria-labelledby` | Only a fixed list. None of these are available, so number, file and length constraints can't be set. | MED |
| `aria-invalid` output | A11y | Present only when invalid | `aria_invalid.or(Some(error))` always writes `aria-invalid="false"` | LOW |
| Field state propagation | Validation state | `data-invalid` and `data-disabled` on `Field` style the whole block | `Input` has `error` and `disabled`, but `Field` can't pass either down (see Field) | LOW |
| Extra events | Events | Any DOM event | No `onclick`, `onpaste` or `onselect` | LOW |

## Input Group

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `InputGroupAddon` with `align: "inline-start" \| "inline-end" \| "block-start" \| "block-end"` | Sub-component / prop | Addons go in any of 4 positions. Block positions make header/footer toolbars above or below an input or textarea (file name + copy button, char count + Post button). | Only `prefix`, `suffix`, `addon_before` and `addon_after` props, all inline. **Block alignment is impossible.** | HIGH |
| `InputGroupButton` (`size: xs \| icon-xs \| sm \| icon-sm`, default `xs`; `variant` default `ghost`) | Sub-component | Compact buttons sized for inside the group (copy, search, dropdown trigger, popover trigger) | Missing. Users drop in a full-size `Button` with no group sizing. | MED |
| `InputGroupInput` / `InputGroupTextarea` | Sub-components | Borderless control. Focus ring and `aria-invalid` styling move to the whole group. | Missing. `children` is a normal `Input` with its own border and focus ring. There is no textarea variant with block addons. | MED |
| `InputGroupText` | Sub-component | Styled static text ("https://", "USD", "0/280") | Missing. Only raw `Html` in a prefix span. | MED |
| Focus order | A11y | Docs: addons **must follow** the input in the DOM and be positioned visually with `align`, for correct focus order | `addon_before` and `prefix` render **before** the input in the DOM, so a focusable button in a prefix is tabbed before the input | LOW |
| `size` prop | Prop | n/a (no size upstream) | `size: Option<Size>` is accepted but ignored | LOW |
| Custom control hook (`data-slot="input-group-control"`) | Behaviour | Any third-party control gets the group focus styling | No equivalent | LOW |

## Input OTP

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Controlled `value` | Controlled/uncontrolled | `value` + `onChange` fully controlled (docs "Controlled" example) | `value` is read **only at mount** (`use_state` init). Later prop changes, such as clearing after a failed verify, are ignored. | HIGH |
| `InputOTPGroup`, `InputOTPSlot index`, `InputOTPSeparator` | Sub-components | Composable slots and groups with separators (3-3, 2-2-2 layouts are in Usage) | One monolithic component with `length`. No groups and no separators. | MED |
| `pattern` enforcement | Validation | `pattern={REGEXP_ONLY_DIGITS}` / `..._AND_CHARS` **rejects** characters that don't match, including on paste | `pattern` is only set as the HTML `pattern` attribute on each box, which does nothing while typing. Any character, including in pasted text, is accepted. | MED |
| `inputmode` | Behaviour | Follows the pattern (alphanumeric allowed) | Hard-coded to `inputmode="numeric"`. Mobile users get a numeric keypad even with an alphanumeric pattern. | MED |
| Overwriting a filled slot | Keyboard | Typing on a filled slot replaces the character at the caret | Each box has `maxlength="1"` and the handler keeps the *first* character, so typing into a filled box does nothing unless the text is selected | MED |
| Form integration (`id`, `name`, `required`) | Props | One real input, so `required`, `id` (targeted by `FieldLabel htmlFor`) and form submission work | No `id`, `name` or `required`. The value is never submitted with a form, and a `<label for>` can't target it. | MED |
| `aria-invalid` on slots | Validation state | Docs "Invalid" example sets `aria-invalid` per slot | No invalid prop | LOW |
| Prop naming `maxLength` | Prop | `maxLength` | `length` | LOW |
| `containerClassName`, `textAlign`, `pushPasswordManagerStrategy`, `onComplete` | Props | input-otp library props | `on_complete` exists. The others are missing. | LOW |
| Completion check | Behaviour | Counts characters | `combined.len() == length` counts **bytes**, so it misfires on non-ASCII characters | LOW |

## Item

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `ItemMedia`, `ItemContent`, `ItemTitle`, `ItemDescription`, `ItemActions`, `ItemHeader`, `ItemFooter` | Sub-components | Composable layout: media + content + **actions** (buttons, chevron), header image, footer | Monolithic: `icon` (text-only `AttrValue`), `children` as title, `description`. **No actions slot**, no header or footer, and you can't pass an avatar or icon component. | HIGH |
| `variant: "default" \| "outline" \| "muted"` | Variant | Three styles | Missing | MED |
| `size: "default" \| "sm" \| "xs"` | Size | Three densities (xs is used inside dropdown menu items) | Missing | MED |
| `ItemGroup` + `ItemSeparator` | Sub-components | List container (`role="list"` with `role="listitem"` items in the examples) and dividers | Missing | MED |
| `ItemMedia variant: "default" \| "icon" \| "image"` | Variant | Icon tile or image thumbnail | Missing (only a text span) | MED |
| Link item | Behaviour | Item rendered as `<a href>` (via `render`); hover and focus land on the anchor | No `href`. Always a `<div role="button">`. | MED |
| Role and keyboard | A11y / keyboard | A plain display container unless it is rendered as a link | Always `role="button" tabindex="0"`, even with no `onclick`, so static rows are announced as buttons and become tab stops. Enter/Space do **not** activate `onclick`, because a div with role button has no native keyboard activation. | MED |
| `aria-selected` on a button | A11y | n/a | `aria-selected` is not valid on `role="button"` | LOW |

## Kbd

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `KbdGroup` | Sub-component | Groups keys (`Ctrl` `B`, `⌘` `K`) with consistent spacing | Missing (only `Kbd`, exported alone from `mod.rs`) | MED |

(Our `size` prop is extra and not upstream. `.kbd` has no CSS; see Cross-cutting.)

## Label

No meaningful gaps. Upstream Label is a plain `<label htmlFor>`; ours has `html_for`, `id`, `disabled` and a `required` indicator. The Base UI Label API page returned 404, so this rests on the shadcn page only.

## Menubar

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Open/close state | Behaviour / controlled | Each `MenubarMenu` is a Menu root with `open`, `defaultOpen` and `onOpenChange`. Clicking a trigger opens it; clicking outside, Escape, or selecting an item (`closeOnClick`, default true) closes it. | **Stateless.** `MenubarMenu` provides no context. The trigger hard-codes `aria-expanded="false"`. `MenubarContent` is always rendered, and there is no `.menubar*` CSS to hide it. | HIGH |
| Keyboard navigation | Keyboard | ArrowLeft/Right move between triggers (loop), ArrowDown opens and focuses the first item, Up/Down move through items, Right/Left open and close submenus, Escape closes and returns focus, type-ahead, roving tabindex | Trigger: ArrowDown/Enter/Space re-fire `click`, and Escape only calls `preventDefault`. No roving focus between triggers or items. | HIGH |
| `MenubarSubTrigger` + `MenubarSubContent` | Sub-components | Nested submenus (`Share >`, `Find >` in Usage and the demo). The SubmenuTrigger takes `openOnHover`, `delay` and `closeDelay`. | `MenubarSub` is only a `<div>`. The trigger and content parts don't exist. | HIGH |
| Radio items work | Behaviour / events | `MenubarRadioGroup value/defaultValue/onValueChange`. The item shows checked state. | `MenubarRadioItem` **ignores** `value` (`value: _`) and never reads `MenubarRadioContext`. It never shows as checked, never sets `aria-checked`, and never calls the group's `onchange`. | HIGH |
| `MenubarShortcut` | Sub-component | Right-aligned shortcut hint (`⌘T`), used in almost every example | Missing | MED |
| `MenubarGroup`, `MenubarLabel` | Sub-components | Group wrapper and non-interactive heading | Missing | MED |
| Item activation from keyboard | Keyboard | Enter/Space selects the highlighted item | `MenubarItem` `onkeydown` calls `preventDefault` on Enter/Space and does **nothing else**, so keyboard users can't activate items. Checkbox and radio items have no tabindex or keydown at all. | MED |
| `MenubarCheckboxItem` uncontrolled | Controlled/uncontrolled | `checked`, `defaultChecked` and `onCheckedChange` | Controlled only (`checked` + `onchange`). No `default_checked`, so without a callback the item never toggles. | MED |
| Content positioning | Props | Positioner `side`, `align`, `sideOffset`, `alignOffset` (defaults bottom/start in shadcn) | No positioning props and no CSS | MED |
| `MenubarItem inset`, `variant="destructive"` | Props / variant | Inset aligns items that have no icon. Destructive is used for Delete. | Missing | LOW |
| Root props `disabled`, `loopFocus`, `modal`, `orientation` | Props | Menubar root options | Missing | LOW |

## Native Select

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `value` / `default_value` do nothing | Controlled/uncontrolled | Native `value` / `defaultValue` select the matching option | Both are written as `value={value.or(default_value)}` on `<select>`. Yew 0.21 only sets the `value` **property** for `input` and `textarea` (`vtag.rs:142-143`). On `<select>` it becomes an HTML attribute, which browsers ignore. Controlled and default selection don't work; the only workaround is `selected` on each `NativeSelectOption`. | HIGH |
| `aria-describedby` | A11y | Pass-through, used to link Field description and error | Missing (we have `aria_label`, `aria_labelledby`, `aria_invalid`) | LOW |
| Custom class on wrapper | Props | `className` on the wrapper and select | `class` goes on the `<select>`, and the wrapper div can't be changed | LOW |
| Value-typed change callback / `oninput` | Events | Native events | Only a raw `onchange: Callback<Event>`. No `Callback<String>` convenience and no `oninput`. | LOW |

## Navigation Menu

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Open state / active item | Behaviour / controlled | Root `value`, `defaultValue` and `onValueChange` (the open item's value), plus `Item value`. Content shows only for the active item. | **Stateless.** No value props. The trigger hard-codes `aria-expanded="false"`. Every `NavigationMenuContent` is always rendered, and the CSS (`position:absolute; top:0; left:0`) stacks them over the list. | HIGH |
| Keyboard | Keyboard | Arrow keys move between triggers and links, ArrowDown or Enter opens and moves into content, Escape closes and returns focus (`escape-key` / `list-navigation` reasons) | ArrowDown/Right/Enter/Space re-fire `click` on the trigger, and Escape only calls `preventDefault`. No focus movement. | HIGH |
| Hover open with `delay` / `closeDelay` | Props / behaviour | Opens on hover, 50 ms defaults on Root | Optional `onmouseenter` and `onmouseleave` callbacks only; no built-in hover behaviour and no delays | MED |
| Viewport / Positioner | Sub-component / props | A shared viewport that resizes and animates between contents. `side`, `align`, `sideOffset` and `alignOffset`, collision handling. | Missing | MED |
| `NavigationMenuLink closeOnClick` | Prop | Closes the menu after a link is clicked | Missing (`active` and `aria-current` are present) | LOW |
| `navigationMenuTriggerStyle()` | Helper | Makes a top-level link look like a trigger (the "Docs" item) | No helper. Workaround: add class `navigation-menu-trigger`. | LOW |
| Trigger `disabled` | Prop | Supported | Missing | LOW |
| `NavigationMenuIndicator` follows the active trigger | Behaviour | Moves under the open trigger | Renders a static arrow that is not tied to any state | LOW |
| `aria-orientation` on `<nav>` | A11y | n/a | Not a valid ARIA attribute for the navigation role | LOW |

---

## Totals

| Component | HIGH | MED | LOW |
|---|---|---|---|
| Cross-cutting | 0 | 1 | 0 |
| Empty | 1 | 1 | 2 |
| Field | 2 | 6 | 3 |
| Hover Card | 3 | 3 | 4 |
| Input | 0 | 2 | 3 |
| Input Group | 1 | 3 | 3 |
| Input OTP | 1 | 5 | 4 |
| Item | 1 | 6 | 1 |
| Kbd | 0 | 1 | 0 |
| Label | 0 | 0 | 0 |
| Menubar | 4 | 5 | 2 |
| Native Select | 1 | 0 | 3 |
| Navigation Menu | 2 | 2 | 5 |
| **Total** | **16** | **35** | **30** |
