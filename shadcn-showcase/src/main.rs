//! shadcn-rs Showcase Application
//!
//! This application demonstrates all components in the shadcn-rs library.

mod components;
mod pages;
mod routes;

use yew::prelude::*;
use yew_router::prelude::*;

use components::{Sidebar, ThemeToggle};
use pages::*;
use routes::Route;

/// Main application component
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="app-layout">
                <Sidebar />
                <div class="app-main">
                    <header class="app-header">
                        <div class="app-header-content">
                            <button class="mobile-menu-btn" aria-label="Toggle menu">
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <line x1="3" y1="12" x2="21" y2="12"></line>
                                    <line x1="3" y1="6" x2="21" y2="6"></line>
                                    <line x1="3" y1="18" x2="21" y2="18"></line>
                                </svg>
                            </button>
                            <div class="app-header-spacer"></div>
                            <ThemeToggle />
                        </div>
                    </header>
                    <main class="app-content">
                        <Switch<Route> render={switch} />
                    </main>
                </div>
            </div>
        </BrowserRouter>
    }
}

/// Route switch function
fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },

        // Getting Started
        Route::Installation => html! { <InstallationPage /> },
        Route::QuickStart => html! { <QuickStartPage /> },
        Route::Theming => html! { <ThemingPage /> },
        Route::Accessibility => html! { <AccessibilityPage /> },
        Route::Migration => html! { <MigrationPage /> },

        // Tier 1 - Foundational
        Route::Alert => html! { <AlertPage /> },
        Route::Avatar => html! { <AvatarPage /> },
        Route::Badge => html! { <BadgePage /> },
        Route::Button => html! { <ButtonPage /> },
        Route::Card => html! { <CardPage /> },
        Route::Kbd => html! { <KbdPage /> },
        Route::Separator => html! { <SeparatorPage /> },
        Route::Skeleton => html! { <SkeletonPage /> },
        Route::Spinner => html! { <SpinnerPage /> },
        Route::Typography => html! { <TypographyPage /> },

        // Tier 2 - Form Components
        Route::Checkbox => html! { <CheckboxPage /> },
        Route::Form => html! { <FormPage /> },
        Route::Input => html! { <InputPage /> },
        Route::Label => html! { <LabelPage /> },
        Route::Progress => html! { <ProgressPage /> },
        Route::Radio => html! { <RadioPage /> },
        Route::Select => html! { <SelectPage /> },
        Route::Slider => html! { <SliderPage /> },
        Route::Switch => html! { <SwitchPage /> },
        Route::Textarea => html! { <TextareaPage /> },

        // Tier 3 - Layout & Structure
        Route::AspectRatio => html! { <AspectRatioPage /> },
        Route::Empty => html! { <EmptyPage /> },
        Route::Item => html! { <ItemPage /> },
        Route::Resizable => html! { <ResizablePage /> },
        Route::ScrollArea => html! { <ScrollAreaPage /> },
        Route::Table => html! { <TablePage /> },

        // Tier 4 - Interactive
        Route::ButtonGroup => html! { <ButtonGroupPage /> },
        Route::Collapsible => html! { <CollapsiblePage /> },
        Route::Field => html! { <FieldPage /> },
        Route::InputGroup => html! { <InputGroupPage /> },
        Route::Toggle => html! { <TogglePage /> },
        Route::ToggleGroup => html! { <ToggleGroupPage /> },

        // Tier 5 - Overlays & Popups
        Route::Accordion => html! { <AccordionPage /> },
        Route::AlertDialog => html! { <AlertDialogPage /> },
        Route::Dialog => html! { <DialogPage /> },
        Route::Drawer => html! { <DrawerPage /> },
        Route::DropdownMenu => html! { <DropdownMenuPage /> },
        Route::HoverCard => html! { <HoverCardPage /> },
        Route::Popover => html! { <PopoverPage /> },
        Route::Sheet => html! { <SheetPage /> },
        Route::Tabs => html! { <TabsPage /> },
        Route::Tooltip => html! { <TooltipPage /> },

        // Tier 6 - Navigation
        Route::Breadcrumb => html! { <BreadcrumbPage /> },
        Route::ContextMenu => html! { <ContextMenuPage /> },
        Route::Menubar => html! { <MenubarPage /> },
        Route::NavigationMenu => html! { <NavigationMenuPage /> },
        Route::Pagination => html! { <PaginationPage /> },
        Route::Sidebar => html! { <SidebarPage /> },

        // Tier 7 - Advanced Forms
        Route::Calendar => html! { <CalendarPage /> },
        Route::Combobox => html! { <ComboboxPage /> },
        Route::Command => html! { <CommandPage /> },
        Route::DatePicker => html! { <DatePickerPage /> },
        Route::InputOtp => html! { <InputOtpPage /> },

        // Tier 8 - Complex
        Route::Carousel => html! { <CarouselPage /> },
        Route::Chart => html! { <ChartPage /> },
        Route::DataTable => html! { <DataTablePage /> },
        Route::Sonner => html! { <SonnerPage /> },
        Route::Toast => html! { <ToastPage /> },

        // 404
        Route::NotFound => html! { <NotFoundPage /> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
