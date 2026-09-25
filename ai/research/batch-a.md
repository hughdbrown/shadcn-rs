# Upstream gap analysis, batch A (accordion to carousel)

Source: `https://ui.shadcn.com/docs/components/base/<name>.md` (the Markdown version of each docs page, fetched 2026-09-24, all 12 returned HTTP 200). For components that defer to Base UI, the prop tables came from `https://base-ui.com/react/components/<name>.md` (accordion, alert-dialog, avatar). Calendar defers to React DayPicker and carousel defers to Embla, so those rows use only what the shadcn docs examples show.

Out of scope: `asChild`/`render` polymorphism, which upstream uses on almost every part (Yew cannot spread arbitrary props onto a child element). Where the missing polymorphism affects real usage, the row names the Yew equivalent (for example an `href` prop) instead. Also out of scope: Tailwind-only styling and RTL.

Priority: **HIGH** means common usage breaks or a key feature is missing. **MED** means a documented feature or a11y gap with a workaround. **LOW** means minor.

---

## Accordion (`accordion.rs`)

Upstream API: `Accordion` (`multiple`, `defaultValue: Value[]`, `value: Value[]`, `onValueChange(value[])`, `disabled`, `keepMounted`, `hiddenUntilFound`), `AccordionItem` (`value`, `disabled`, `onOpenChange`), `AccordionTrigger` and `AccordionContent`. Base UI has **deprecated** `orientation` and `loopFocus` and removed roving arrow-key focus, following an APG update. Tab is now the only way to move between triggers.

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Array-valued `value`/`default_value` | prop | `defaultValue={["a","b"]}` / `value` are arrays, so several items can start open in `multiple` mode | `default_value`/`value` are `Option<AttrValue>` (one string). A multiple accordion cannot start with 2+ items open or be controlled | HIGH |
| `on_value_change` payload | event | `onValueChange(value: Value[])` receives the full set of open items | Emits only the `String` of the item that was toggled. The caller cannot tell whether it opened or closed | MED |
| True controlled mode | behavior | When `value` is set, the component only reports changes and the parent owns state | Internal `use_state` always updates on click. `value` is only copied in via `use_effect_with`, so the UI can drift from the parent | MED |
| Disabled item semantics | behavior | Disabled item: the trigger is disabled/`aria-disabled` and ignores interaction | `AccordionItem.disabled` blocks the toggle, but the `<button>` has no `disabled`/`aria-disabled` attribute, so it stays focusable and announces as active | MED |
| Trigger/panel ARIA linkage and heading | behavior | Base UI renders `Accordion.Header` (a heading), with the trigger's `aria-controls` pointing to the panel id and the panel's `aria-labelledby` pointing to the trigger | No ids, no `aria-controls`, no `aria-labelledby` on `role="region"`, and the trigger is not inside a heading element | MED |
| Root `disabled` | prop | `disabled` on `Accordion` disables every item | Missing | LOW |
| Item `on_open_change` | event | `AccordionItem.onOpenChange(open)` | Missing | LOW |
| `keep_mounted` / `hidden_until_found` | prop | Keep closed panels in the DOM, and let the browser's find-in-page (Ctrl+F) search and open them | Closed content returns `html!{}` (unmounted) | LOW |
| Outdated doc comment | behavior | No arrow-key nav (deprecated upstream) | Rustdoc claims "Keyboard navigation (Arrow keys, Home, End)", which is not implemented. Fix the comment rather than add the feature | LOW |

Note: our `accordion_type: Single | Multiple` plus `collapsible` covers upstream's `multiple` boolean, so this is not a gap.

---

## Alert (`alert.rs`)

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `AlertAction` | subcomponent | Action element (for example a button) positioned absolutely in the top-right of the alert | Missing | MED |
| Leading icon layout | behavior | The basic example is icon + title + description, and the alert lays out as a grid with the icon in its own column | Our `.alert` CSS has no icon slot, so an icon child stacks above the title | LOW |

Variants `default`/`destructive` match. Other `Variant` values are accepted but unstyled, which is harmless.

---

## Alert Dialog (`alert_dialog.rs`)

Upstream API: `AlertDialogContent size="default"|"sm"`, `AlertDialogMedia`, and `AlertDialogAction`/`AlertDialogCancel` accept `variant` (examples use `variant="destructive"` and `variant="outline"`). The Base UI root has `open`, `defaultOpen`, `onOpenChange(open, details)` and `onOpenChangeComplete(open)`. The popup has `initialFocus`/`finalFocus`.

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Controlled vs uncontrolled detection | behavior | `open` is optional. `onOpenChange` can observe an uncontrolled dialog | `is_open = if on_open_change.is_some() { open } else { internal }`. Passing `on_open_change` without `open` (for example with `default_open`) locks the dialog at `open=false`, so it never opens. `open` should be `Option<bool>` | HIGH |
| Accessible name/description | behavior | Title and description label the popup (`aria-labelledby`/`aria-describedby`) | `AlertDialogTitle` (`<h2>`) and `AlertDialogDescription` (`<p>`) have no ids, and the `role="alertdialog"` element has no `aria-labelledby`/`aria-describedby` | MED |
| `AlertDialogMedia` | subcomponent | Icon/image slot above the title (examples "Media", "Small with Media") | Missing | MED |
| `size` on content | prop | `size: "default" \| "sm"` on `AlertDialogContent` | Missing | MED |
| `variant` on Action/Cancel | prop | `<AlertDialogAction variant="destructive">`, `<AlertDialogCancel variant="outline">` (Button variants) | Plain `<button class="alert-dialog-action">` with no `variant`, so you cannot build the "Destructive" example | MED |
| Trigger semantics | behavior | Trigger is a real button, with `aria-haspopup="dialog"` and `aria-expanded` state | `AlertDialogTrigger` is a `<div onclick>` wrapper around the children, with no ARIA state | LOW |
| `on_open_change_complete` | event | Fires after open/close animations finish | Missing | LOW |
| Initial/final focus control | prop | `initialFocus` / `finalFocus` on the popup | The focus trap restores focus to the previous element, but the caller cannot choose the initial target (APG recommends focusing Cancel in destructive dialogs) | LOW |

---

## Aspect Ratio (`aspect_ratio.rs`)

No meaningful gaps: `ratio` + `class` + children match upstream. Upstream marks `ratio` as required, while ours defaults to 1.0, which is harmless.

---

## Avatar (`avatar.rs`)

Upstream API: composable `Avatar` (`size: default|sm|lg`), `AvatarImage` (`src`, `alt`, and Base UI's `onLoadingStatusChange`), `AvatarFallback` (arbitrary children, Base UI `delay`), `AvatarBadge`, `AvatarGroup` and `AvatarGroupCount`.

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Size/shape CSS classes don't match the stylesheet (bug) | behavior | `size` changes the rendered size | Component emits `avatar-xs`…`avatar-2xl` and `avatar-circle`/`avatar-square`, but `styles/components.css` defines `.avatar.size-*` and `.avatar.shape-*`. Size and shape currently have no effect. `.avatar-initials` also has no CSS rule | HIGH |
| Composable Image/Fallback | subcomponent | `<Avatar><AvatarImage/><AvatarFallback>…</AvatarFallback></Avatar>`. The fallback can hold any content (icons, custom initials) | Monolithic props: `src`, `alt`, `initials: AttrValue` and `fallback_icon: AttrValue` (text only) | MED |
| `AvatarBadge` | subcomponent | Status dot or icon at the bottom right | Missing | MED |
| `AvatarGroup` + `AvatarGroupCount` | subcomponent | Overlapping stack plus "+N" count | Missing | MED |
| `on_loading_status_change` | event | Callback for image idle/loading/loaded/error | Only an internal `onerror` flag | LOW |
| Fallback `delay` | prop | Waits N ms before showing the fallback, to avoid a flash while the image loads | Missing | LOW |
| Double accessible name | behavior | Only the image `alt` names the avatar | The wrapper has `role="img" aria-label=…` and the inner `<img alt=…>` repeats it. It defaults to "Avatar" even when decorative | LOW |

Our sizes (`Xs`…`Xl2`) and `shape` are a superset of upstream's, so they are not a gap.

---

## Badge (`badge.rs`)

Upstream `variant`: `default | secondary | destructive | outline | ghost | link`.

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `ghost` and `link` variants unstyled | prop | Both are documented variants | `Variant::Ghost`/`Link` type-check, but there are no `.badge.variant-ghost`/`.badge.variant-link` CSS rules, so they render as a bare span | MED |
| Link badge | prop | `<Badge render={<a href=…/>}>` renders the badge as a link | Always a `<span>`. The Yew equivalent would be an `href: Option<AttrValue>` that switches to `<a>` | LOW |

---

## Breadcrumb (`breadcrumb.rs`)

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `BreadcrumbEllipsis` component | subcomponent | Standalone component you can place anywhere or use as a `DropdownMenuTrigger` ("Collapsed" and "Dropdown" examples). It includes sr-only "More" text | Not a component. An ellipsis appears only when `BreadcrumbList.max_items` auto-collapses. It can't be composed with a dropdown | MED |
| `max_items` collapse drops separators | behavior | Not in upstream (manual composition) | `max_items` counts `BreadcrumbSeparator` children as items and slices them out. The result is `First … Last` with no separators around the ellipsis. The ellipsis `<li role="presentation">` has no accessible text | MED |
| `BreadcrumbPage` semantics | behavior | Renders `role="link" aria-disabled="true" aria-current="page"` | Only `aria-current="page"` | LOW |
| Router link integration | prop | `BreadcrumbLink render={<Link …/>}` for client-side routing | `href` + `onclick` only. A yew-router `Link` can't take the breadcrumb styling (workaround: `onclick` with `prevent_default`) | LOW |

---

## Button (`button.rs`)

Upstream `variant`: `default | outline | ghost | destructive | secondary | link`. `size`: `default | xs | sm | lg | icon | icon-xs | icon-sm | icon-lg`.

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Icon sizes | prop | `icon`, `icon-xs`, `icon-sm`, `icon-lg` give a square icon-only button | `Size` is `Xs`…`Xl2` only. Icon-only buttons get text padding | MED |
| `buttonVariants` helper | prop | `buttonVariants({variant, size})` returns classes so `<a>` links can look like buttons. Upstream explicitly says not to render `<Button>` as `<a>` | No public helper. Users must hand-write `"btn variant-x size-y"` | MED |
| Native attribute passthrough | prop | Accepts all `<button>` props (`name`, `value`, `form`, `title`, `aria-expanded`, `aria-controls`, `aria-haspopup`, focus events…) | Only `type`, `onclick`, `aria_label`, `id`, `style`, `class` and `disabled`. You can't use it as a form submit with `name`/`value`, or as a disclosure/menu trigger with correct ARIA | MED |

Our extra `Primary` variant and `loading`/`full_width` props are additions, not gaps. Upstream implements a loading state by putting a `<Spinner>` inside the button.

---

## Button Group (`button_group.rs`)

Upstream API: `ButtonGroup` (`orientation`), `ButtonGroupSeparator` (`orientation`, default vertical) and `ButtonGroupText`. The docs' Accessibility section says: `role="group"`, Tab between buttons, and label with `aria-label`/`aria-labelledby`.

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `ButtonGroupSeparator` | subcomponent | Divider between non-outline buttons, and the basis of split buttons | Missing | MED |
| `ButtonGroupText` | subcomponent | Text or label segment inside a group | Missing | MED |
| `aria_label` / `aria_labelledby` | prop | The docs tell users to label the group | Neither prop exists, so the group can't be labelled | MED |
| `size` prop ignored | behavior | Upstream sets size on individual buttons | `size: Option<Size>` is accepted then discarded (`size: _`). Either wire it up or remove it | LOW |
| `connected` has no CSS | behavior | n/a | Adds a `button-group-connected` class that no CSS rule uses. Buttons are always joined | LOW |
| Nested groups / non-Button children | behavior | Nested `ButtonGroup`s get spacing. Input, Select and DropdownMenu triggers join the group | The CSS only targets `> .btn`, so inputs, selects and nested groups don't get the joined-border treatment | LOW |

---

## Calendar (`calendar.rs`)

Upstream wraps React DayPicker. The examples use `mode` (single/range), `selected`/`onSelect`, `month`/`onMonthChange`, `defaultMonth`, `numberOfMonths`, `captionLayout="dropdown"`, `disabled` (matchers), `modifiers`, `showOutsideDays`, `showWeekNumber`, `timeZone`, `locale`, `dir` and `buttonVariant`.

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Keyboard grid navigation | behavior | DayPicker grid: arrow keys move by day/week, PageUp/PageDown by month, Home/End to week start/end, and one roving tab stop | No key handling. Every day is its own Tab stop (about 30 to 42 per month) | HIGH |
| Grid ARIA | behavior | `role="grid"` with `gridcell`s that carry selection, and today marked `aria-current="date"` | Root is `role="application"`. `aria-selected` sits on `<button>` elements, which is invalid ARIA. Today has no `aria-current` | MED |
| Controlled visible month | prop/event | `month` + `onMonthChange`, `defaultMonth` | The visible month is internal state, seeded once from `selected`. The caller can't set or observe it, and it doesn't follow later changes to `selected` | MED |
| Month/year dropdown caption | prop | `captionLayout="dropdown"` (month and year selects) | Only prev/next buttons. Picking a far-off year (for example a birthdate) is impractical | MED |
| Disabled matchers and custom modifiers | prop | `disabled` accepts dates, ranges, `{before}`, `{after}` and `{dayOfWeek}`. `modifiers` + `modifiersClassNames` style custom sets ("Booked dates" example) | `min_date`, `max_date` and an explicit `disabled_dates` list. No custom modifier classes | LOW |
| Locale | prop | `locale` localizes month/day names and formatting | English month and day names hard-coded | LOW |
| `time_zone` | prop | `timeZone` for correct "today"/selection | Uses the browser's local time only | LOW |
| `show_outside_days` | prop | Show or hide leading/trailing days from adjacent months (shown by default in shadcn) | Always blank `aria-hidden` cells | LOW |

We already have `mode` (Single/Multiple/Range), `number_of_months`, `show_week_numbers` and `first_day_of_week`, so those are not gaps.

---

## Card (`card.rs`)

Upstream API: `Card` (`size: default|sm`), `CardHeader`, `CardTitle`, `CardDescription`, `CardAction`, `CardContent` and `CardFooter`.

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| `CardAction` | subcomponent | Places a button or badge in the header's top-right, laid out as a grid next to the title and description | Missing. The header is a plain block | MED |
| `size` | prop | `size="sm"` gives tighter spacing (via `--card-spacing`) | Missing | LOW |

Our `CardTitle` renders a heading with a `level`, while upstream renders a `div`. Ours is arguably better, so this is not a gap.

---

## Carousel (`carousel.rs`)

Upstream wraps Embla. The docs cover item sizes via `basis-*`, spacing, `orientation="horizontal|vertical"`, `opts` (`align`, `loop`, `direction`), `setApi` (`scrollSnapList`, `selectedScrollSnap`, `on("select")`) and `plugins` (Autoplay with `stopOnInteraction`, plus stop/reset on mouse enter/leave). It is described as "A carousel with motion and swipe."

| Gap | Kind | Upstream detail | Our current state | Priority |
|---|---|---|---|---|
| Multiple items per view | behavior | `CarouselItem className="basis-1/2 lg:basis-1/3"` shows several items and scrolls by snap point ("Sizes" example) | `CarouselContent` uses `translateX(-{current*100}%)`, which assumes one full-width slide per view. With `basis-1/2` each step skips a whole viewport, and the slide count and indicators are wrong | HIGH |
| Swipe/drag | behavior | Touch or pointer drag to scroll | No pointer or touch handling. Buttons, arrow keys and indicators only | HIGH |
| `orientation` | prop | `orientation="vertical"` | Hard-coded `orientation-horizontal` class, even though `.carousel.orientation-vertical` CSS exists. No vertical support | MED |
| Carousel ARIA | behavior | Root: `role="region" aria-roledescription="carousel"`. Items: `role="group" aria-roledescription="slide"` | Root has `role="region"`, but no roledescription and a fixed `aria-label="Carousel"`. Items have no role. `aria-live="polite"` stays on during autoplay (APG says turn live announcements off while rotating) | MED |
| Autoplay pause/stop | behavior | Autoplay plugin with `stopOnInteraction`, and the example stops on `mouseenter` and resets on `mouseleave` | `autoplay` interval never pauses on hover, focus or interaction (a WCAG 2.2.2 concern) | MED |
| `align` option | prop | `opts={{ align: "start" }}` (start/center/end) | Missing | LOW |
| `loop` default | behavior | Embla's `loop` defaults to false | `loop_slides` defaults to `true` | LOW |
| API/`setApi` equivalents | event | Read the snap count, `canScrollPrev`/`Next`, and subscribe to `select`/`reInit` | `current` + `on_slide_change` cover selection. Slide count isn't available to the caller outside the carousel's children | LOW |
| Indicator ARIA | behavior | n/a (indicators are our addition) | `role="tablist"` whose children are plain buttons with `aria-selected` and no `role="tab"`, which is invalid | LOW |

---

## Totals

| Component | HIGH | MED | LOW |
|---|---|---|---|
| Accordion | 1 | 4 | 4 |
| Alert | 0 | 1 | 1 |
| Alert Dialog | 1 | 4 | 3 |
| Aspect Ratio | 0 | 0 | 0 |
| Avatar | 1 | 3 | 3 |
| Badge | 0 | 1 | 1 |
| Breadcrumb | 0 | 2 | 2 |
| Button | 0 | 3 | 0 |
| Button Group | 0 | 3 | 3 |
| Calendar | 1 | 3 | 4 |
| Card | 0 | 1 | 1 |
| Carousel | 2 | 3 | 4 |
| **Total** | **6** | **28** | **26** |
