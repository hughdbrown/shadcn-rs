//! UI Components
//!
//! This module contains all shadcn-rs UI components organized by tier.

// Tier 1 - Foundational components
pub mod alert;
pub mod avatar;
pub mod badge;
pub mod button;
pub mod card;
pub mod kbd;
pub mod separator;
pub mod skeleton;
pub mod spinner;
pub mod typography;

// Tier 2 - Form components
pub mod checkbox;
pub mod form;
pub mod input;
pub mod label;
pub mod progress;
pub mod radio;
pub mod select;
pub mod slider;
pub mod switch;
pub mod textarea;

// Tier 3 - Overlay & Interactive components
pub mod accordion;
pub mod dialog;
pub mod drawer;
pub mod dropdown_menu;
pub mod popover;
pub mod tabs;
pub mod tooltip;

// Tier 6 - Navigation components
pub mod breadcrumb;
pub mod context_menu;
pub mod menubar;
pub mod navigation_menu;
pub mod pagination;
pub mod sidebar;

// Tier 7 - Advanced Forms
pub mod calendar;
pub mod combobox;
pub mod command;
pub mod date_picker;

// Re-export Tier 1 components
pub use alert::{Alert, AlertDescription, AlertTitle};
pub use avatar::{Avatar, AvatarShape};
pub use badge::Badge;
pub use button::Button;
pub use card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
pub use kbd::Kbd;
pub use separator::{Separator, SeparatorOrientation};
pub use skeleton::{Skeleton, SkeletonShape};
pub use spinner::Spinner;
pub use typography::{FontWeight, TextAlign, TextColor, Typography, TypographyVariant};

// Re-export Tier 2 components
pub use checkbox::Checkbox;
pub use form::{
    Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage,
    FormMessageType,
};
pub use input::Input;
pub use label::Label;
pub use progress::Progress;
pub use radio::{Radio, RadioGroup};
pub use select::Select;
pub use slider::Slider;
pub use switch::Switch;
pub use textarea::{Textarea, TextareaResize};

// Re-export Tier 3 components
pub use accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger, AccordionType};
pub use dialog::{
    Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger,
};
pub use drawer::{
    Drawer, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader, DrawerTitle,
    DrawerTrigger,
};
pub use dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator,
    DropdownMenuTrigger,
};
pub use popover::{Popover, PopoverContent, PopoverTrigger};
pub use tabs::{Tabs, TabsContent, TabsList, TabsOrientation, TabsTrigger};
pub use tooltip::{Tooltip, TooltipContent, TooltipTrigger};

// Re-export Tier 6 components
pub use breadcrumb::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator,
};
pub use context_menu::{
    ContextMenu, ContextMenuCheckboxItem, ContextMenuContent, ContextMenuItem,
    ContextMenuRadioGroup, ContextMenuRadioItem, ContextMenuSeparator, ContextMenuSub,
    ContextMenuTrigger,
};
pub use menubar::{
    Menubar, MenubarCheckboxItem, MenubarContent, MenubarItem, MenubarMenu, MenubarRadioGroup,
    MenubarRadioItem, MenubarSeparator, MenubarSub, MenubarTrigger,
};
pub use navigation_menu::{
    NavigationMenu, NavigationMenuContent, NavigationMenuIndicator, NavigationMenuItem,
    NavigationMenuLink, NavigationMenuList, NavigationMenuTrigger,
};
pub use pagination::{
    Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink,
    PaginationNext, PaginationPrevious,
};
pub use sidebar::{
    Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu,
    SidebarMenuButton, SidebarMenuItem, SidebarSeparator,
};

// Re-export Tier 7 components
pub use calendar::{Calendar, CalendarMode};
pub use combobox::{
    Combobox, ComboboxContent, ComboboxEmpty, ComboboxGroup, ComboboxInput, ComboboxItem,
    ComboboxSeparator, ComboboxTrigger,
};
pub use command::{
    Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandSeparator,
    CommandShortcut,
};
pub use date_picker::DatePicker;
