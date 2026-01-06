//! UI Components
//!
//! This module contains all shadcn-rs UI components organized by tier.

// Tier 1 - Foundational components
pub mod alert;
pub mod badge;
pub mod button;
pub mod card;
pub mod separator;
pub mod skeleton;

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

// Re-export Tier 1 components
pub use alert::{Alert, AlertDescription, AlertTitle};
pub use badge::Badge;
pub use button::Button;
pub use card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
pub use separator::{Separator, SeparatorOrientation};
pub use skeleton::{Skeleton, SkeletonShape};

// Re-export Tier 2 components
pub use checkbox::Checkbox;
pub use form::{Form, FormField, FormMessage, FormMessageType};
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
