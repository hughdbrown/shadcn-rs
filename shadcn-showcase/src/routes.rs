//! Route definitions for the showcase application

use yew_router::prelude::*;

/// Main route enum for the application
#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    /// Home page
    #[at("/")]
    Home,

    // Getting Started
    #[at("/getting-started/installation")]
    Installation,
    #[at("/getting-started/quick-start")]
    QuickStart,
    #[at("/getting-started/theming")]
    Theming,
    #[at("/getting-started/accessibility")]
    Accessibility,
    #[at("/getting-started/migration")]
    Migration,

    // Tier 1 - Foundational
    #[at("/components/alert")]
    Alert,
    #[at("/components/avatar")]
    Avatar,
    #[at("/components/badge")]
    Badge,
    #[at("/components/button")]
    Button,
    #[at("/components/card")]
    Card,
    #[at("/components/kbd")]
    Kbd,
    #[at("/components/separator")]
    Separator,
    #[at("/components/skeleton")]
    Skeleton,
    #[at("/components/spinner")]
    Spinner,
    #[at("/components/typography")]
    Typography,

    // Tier 2 - Form Components
    #[at("/components/checkbox")]
    Checkbox,
    #[at("/components/form")]
    Form,
    #[at("/components/input")]
    Input,
    #[at("/components/label")]
    Label,
    #[at("/components/progress")]
    Progress,
    #[at("/components/radio")]
    Radio,
    #[at("/components/select")]
    Select,
    #[at("/components/slider")]
    Slider,
    #[at("/components/switch")]
    Switch,
    #[at("/components/textarea")]
    Textarea,

    // Tier 3 - Layout & Structure
    #[at("/components/aspect-ratio")]
    AspectRatio,
    #[at("/components/empty")]
    Empty,
    #[at("/components/item")]
    Item,
    #[at("/components/resizable")]
    Resizable,
    #[at("/components/scroll-area")]
    ScrollArea,
    #[at("/components/table")]
    Table,

    // Tier 4 - Interactive
    #[at("/components/button-group")]
    ButtonGroup,
    #[at("/components/collapsible")]
    Collapsible,
    #[at("/components/field")]
    Field,
    #[at("/components/input-group")]
    InputGroup,
    #[at("/components/toggle")]
    Toggle,
    #[at("/components/toggle-group")]
    ToggleGroup,

    // Tier 5 - Overlays & Popups
    #[at("/components/accordion")]
    Accordion,
    #[at("/components/alert-dialog")]
    AlertDialog,
    #[at("/components/dialog")]
    Dialog,
    #[at("/components/drawer")]
    Drawer,
    #[at("/components/dropdown-menu")]
    DropdownMenu,
    #[at("/components/hover-card")]
    HoverCard,
    #[at("/components/popover")]
    Popover,
    #[at("/components/sheet")]
    Sheet,
    #[at("/components/tabs")]
    Tabs,
    #[at("/components/tooltip")]
    Tooltip,

    // Tier 6 - Navigation
    #[at("/components/breadcrumb")]
    Breadcrumb,
    #[at("/components/context-menu")]
    ContextMenu,
    #[at("/components/menubar")]
    Menubar,
    #[at("/components/navigation-menu")]
    NavigationMenu,
    #[at("/components/pagination")]
    Pagination,
    #[at("/components/sidebar")]
    Sidebar,

    // Tier 7 - Advanced Forms
    #[at("/components/calendar")]
    Calendar,
    #[at("/components/combobox")]
    Combobox,
    #[at("/components/command")]
    Command,
    #[at("/components/date-picker")]
    DatePicker,
    #[at("/components/input-otp")]
    InputOtp,

    // Tier 8 - Complex
    #[at("/components/carousel")]
    Carousel,
    #[at("/components/chart")]
    Chart,
    #[at("/components/data-table")]
    DataTable,
    #[at("/components/sonner")]
    Sonner,
    #[at("/components/toast")]
    Toast,

    /// 404 page
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// Navigation item for the sidebar
#[derive(Clone, PartialEq)]
pub struct NavItem {
    pub label: &'static str,
    pub route: Route,
}

/// Navigation group for organizing items
#[derive(Clone, PartialEq)]
pub struct NavGroup {
    pub title: &'static str,
    pub items: Vec<NavItem>,
}

/// Get all navigation groups for the sidebar
pub fn get_nav_groups() -> Vec<NavGroup> {
    vec![
        NavGroup {
            title: "Getting Started",
            items: vec![
                NavItem { label: "Installation", route: Route::Installation },
                NavItem { label: "Quick Start", route: Route::QuickStart },
                NavItem { label: "Theming", route: Route::Theming },
                NavItem { label: "Accessibility", route: Route::Accessibility },
                NavItem { label: "Migration", route: Route::Migration },
            ],
        },
        NavGroup {
            title: "Foundational",
            items: vec![
                NavItem { label: "Alert", route: Route::Alert },
                NavItem { label: "Avatar", route: Route::Avatar },
                NavItem { label: "Badge", route: Route::Badge },
                NavItem { label: "Button", route: Route::Button },
                NavItem { label: "Card", route: Route::Card },
                NavItem { label: "Kbd", route: Route::Kbd },
                NavItem { label: "Separator", route: Route::Separator },
                NavItem { label: "Skeleton", route: Route::Skeleton },
                NavItem { label: "Spinner", route: Route::Spinner },
                NavItem { label: "Typography", route: Route::Typography },
            ],
        },
        NavGroup {
            title: "Form Components",
            items: vec![
                NavItem { label: "Checkbox", route: Route::Checkbox },
                NavItem { label: "Form", route: Route::Form },
                NavItem { label: "Input", route: Route::Input },
                NavItem { label: "Label", route: Route::Label },
                NavItem { label: "Progress", route: Route::Progress },
                NavItem { label: "Radio", route: Route::Radio },
                NavItem { label: "Select", route: Route::Select },
                NavItem { label: "Slider", route: Route::Slider },
                NavItem { label: "Switch", route: Route::Switch },
                NavItem { label: "Textarea", route: Route::Textarea },
            ],
        },
        NavGroup {
            title: "Layout & Structure",
            items: vec![
                NavItem { label: "Aspect Ratio", route: Route::AspectRatio },
                NavItem { label: "Empty", route: Route::Empty },
                NavItem { label: "Item", route: Route::Item },
                NavItem { label: "Resizable", route: Route::Resizable },
                NavItem { label: "Scroll Area", route: Route::ScrollArea },
                NavItem { label: "Table", route: Route::Table },
            ],
        },
        NavGroup {
            title: "Interactive",
            items: vec![
                NavItem { label: "Button Group", route: Route::ButtonGroup },
                NavItem { label: "Collapsible", route: Route::Collapsible },
                NavItem { label: "Field", route: Route::Field },
                NavItem { label: "Input Group", route: Route::InputGroup },
                NavItem { label: "Toggle", route: Route::Toggle },
                NavItem { label: "Toggle Group", route: Route::ToggleGroup },
            ],
        },
        NavGroup {
            title: "Overlays & Popups",
            items: vec![
                NavItem { label: "Accordion", route: Route::Accordion },
                NavItem { label: "Alert Dialog", route: Route::AlertDialog },
                NavItem { label: "Dialog", route: Route::Dialog },
                NavItem { label: "Drawer", route: Route::Drawer },
                NavItem { label: "Dropdown Menu", route: Route::DropdownMenu },
                NavItem { label: "Hover Card", route: Route::HoverCard },
                NavItem { label: "Popover", route: Route::Popover },
                NavItem { label: "Sheet", route: Route::Sheet },
                NavItem { label: "Tabs", route: Route::Tabs },
                NavItem { label: "Tooltip", route: Route::Tooltip },
            ],
        },
        NavGroup {
            title: "Navigation",
            items: vec![
                NavItem { label: "Breadcrumb", route: Route::Breadcrumb },
                NavItem { label: "Context Menu", route: Route::ContextMenu },
                NavItem { label: "Menubar", route: Route::Menubar },
                NavItem { label: "Navigation Menu", route: Route::NavigationMenu },
                NavItem { label: "Pagination", route: Route::Pagination },
                NavItem { label: "Sidebar", route: Route::Sidebar },
            ],
        },
        NavGroup {
            title: "Advanced Forms",
            items: vec![
                NavItem { label: "Calendar", route: Route::Calendar },
                NavItem { label: "Combobox", route: Route::Combobox },
                NavItem { label: "Command", route: Route::Command },
                NavItem { label: "Date Picker", route: Route::DatePicker },
                NavItem { label: "Input OTP", route: Route::InputOtp },
            ],
        },
        NavGroup {
            title: "Complex",
            items: vec![
                NavItem { label: "Carousel", route: Route::Carousel },
                NavItem { label: "Chart", route: Route::Chart },
                NavItem { label: "Data Table", route: Route::DataTable },
                NavItem { label: "Sonner", route: Route::Sonner },
                NavItem { label: "Toast", route: Route::Toast },
            ],
        },
    ]
}
