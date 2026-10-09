//! # FreyaCN Theme System
//!
//! This module provides a complete theming system inspired by shadcn/ui and built for Freya.
//! It includes a full Tailwind‑style color palette, semantic theme fields, and hooks to
//! consume and provide the theme in your components.
//!
//! ## Overview
//!
//! The theme is composed of two main parts:
//! - **`Colors`**: a palette of 10 shades (50–950) for each of the common Tailwind color families
//!   (slate, gray, zinc, neutral, red, orange, amber, yellow, lime, green, emerald, teal, cyan,
//!   sky, blue, indigo, violet, purple, fuchsia, pink, rose, plus stone, mauve, olive, mist, taupe).
//! - **`Theme`**: a semantic collection of colors used by your UI components (background, foreground,
//!   primary, secondary, accent, destructive, border, etc.). It also stores the full `Colors` palette
//!   and a `is_dark` flag.
//!
//! You can build a theme by choosing a **base palette** (the neutral shades used for backgrounds,
//! borders, and other structural elements) and a **theme palette** (the accent colour used for
//! primary actions, links, and interactive elements).
//!
//! ## Usage
//!
//! ### Setting the root theme
//! In your application’s root component, call `use_init_cn_theme` with the theme you want.
//!
//! ```no_run
//! # use freyacn::theme::*;
//! # use freya::prelude::*;
//! fn App() -> impl IntoElement {
//!     // Create a light theme with a slate base and blue accent.
//!     let theme = Theme::base_color("slate", false).theme_color("blue");
//!     use_init_cn_theme(theme);
//!
//!     // Your UI…
//!     rect()
//!         .expanded()
//!         .background(theme.background)   // use the theme directly
//!         .child("Hello, world!")
//! }
//! ```
//!
//! ### Using the theme in a component
//! Inside any component, call `use_cn_theme()` to retrieve the current theme.
//!
//! ```no_run
//! # use freyacn::theme::*;
//! # use freya::prelude::*;
//! fn MyButton() -> impl IntoElement {
//!     let theme = use_cn_theme();
//!     Button::new()
//!         .background(theme.primary)
//!         .color(theme.primary_foreground)
//!         .child("Click me")
//! }
//! ```
//!
//! ### Overriding the theme for a subtree
//! Use `use_provide_cn_theme` to supply a different theme to a specific branch of your UI.
//!
//! ```no_run
//! # use freyacn::theme::*;
//! # use freya::prelude::*;
//! fn DarkPanel() -> impl IntoElement {
//!     let dark_theme = Theme::base_color("zinc", true).theme_color("rose");
//!     use_provide_cn_theme(dark_theme);
//!     // All children here will see the dark theme.
//!     rect()
//!         .expanded()
//!         .background(dark_theme.background)
//! }
//! ```

use freya::prelude::*;
use freya::prelude::{
    Readable, State, provide_context, try_consume_context, use_consume, use_hook,
};

/// Convert an RGB tuple into a Freya Color.
const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgb(r, g, b)
}

// ------------------------------------------------------------------------
// Color palette
// ------------------------------------------------------------------------

/// Complete FreyaCN color palette (Tailwind/shadcn style).
///
/// This struct contains all the colours from the Tailwind‑style palettes used by shadcn/ui.
/// It includes 10 shades (50, 100, 200, …, 900, 950) for each of the following families:
///
/// - **Neutrals**: `slate`, `gray`, `zinc`, `neutral`, `stone`, `mauve`, `olive`, `mist`, `taupe`
/// - **Accents**: `red`, `orange`, `amber`, `yellow`, `lime`, `green`, `emerald`, `teal`, `cyan`,
///   `sky`, `blue`, `indigo`, `violet`, `purple`, `fuchsia`, `pink`, `rose`
///
/// Plus the basic `white` and `black`.
///
/// # Example
/// ```
/// use freyacn::theme::Colors;
/// let colors = Colors::new();
/// let slate_500 = colors.slate_500;   // tailwind slate-500
/// let blue_400 = colors.blue_400;     // tailwind blue-400
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Colors {
    // basic
    pub white: Color,
    pub black: Color,

    // Stone
    pub stone_50: Color,
    pub stone_100: Color,
    pub stone_200: Color,
    pub stone_300: Color,
    pub stone_400: Color,
    pub stone_500: Color,
    pub stone_600: Color,
    pub stone_700: Color,
    pub stone_800: Color,
    pub stone_900: Color,
    pub stone_950: Color,

    // Mauve
    pub mauve_50: Color,
    pub mauve_100: Color,
    pub mauve_200: Color,
    pub mauve_300: Color,
    pub mauve_400: Color,
    pub mauve_500: Color,
    pub mauve_600: Color,
    pub mauve_700: Color,
    pub mauve_800: Color,
    pub mauve_900: Color,
    pub mauve_950: Color,

    // Olive
    pub olive_50: Color,
    pub olive_100: Color,
    pub olive_200: Color,
    pub olive_300: Color,
    pub olive_400: Color,
    pub olive_500: Color,
    pub olive_600: Color,
    pub olive_700: Color,
    pub olive_800: Color,
    pub olive_900: Color,
    pub olive_950: Color,

    // Mist
    pub mist_50: Color,
    pub mist_100: Color,
    pub mist_200: Color,
    pub mist_300: Color,
    pub mist_400: Color,
    pub mist_500: Color,
    pub mist_600: Color,
    pub mist_700: Color,
    pub mist_800: Color,
    pub mist_900: Color,
    pub mist_950: Color,

    // Taupe
    pub taupe_50: Color,
    pub taupe_100: Color,
    pub taupe_200: Color,
    pub taupe_300: Color,
    pub taupe_400: Color,
    pub taupe_500: Color,
    pub taupe_600: Color,
    pub taupe_700: Color,
    pub taupe_800: Color,
    pub taupe_900: Color,
    pub taupe_950: Color,

    // Slate
    pub slate_50: Color,
    pub slate_100: Color,
    pub slate_200: Color,
    pub slate_300: Color,
    pub slate_400: Color,
    pub slate_500: Color,
    pub slate_600: Color,
    pub slate_700: Color,
    pub slate_800: Color,
    pub slate_900: Color,
    pub slate_950: Color,

    // Gray
    pub gray_50: Color,
    pub gray_100: Color,
    pub gray_200: Color,
    pub gray_300: Color,
    pub gray_400: Color,
    pub gray_500: Color,
    pub gray_600: Color,
    pub gray_700: Color,
    pub gray_800: Color,
    pub gray_900: Color,
    pub gray_950: Color,

    // Zinc
    pub zinc_50: Color,
    pub zinc_100: Color,
    pub zinc_200: Color,
    pub zinc_300: Color,
    pub zinc_400: Color,
    pub zinc_500: Color,
    pub zinc_600: Color,
    pub zinc_700: Color,
    pub zinc_800: Color,
    pub zinc_900: Color,
    pub zinc_950: Color,

    // Neutral
    pub neutral_50: Color,
    pub neutral_100: Color,
    pub neutral_200: Color,
    pub neutral_300: Color,
    pub neutral_400: Color,
    pub neutral_500: Color,
    pub neutral_600: Color,
    pub neutral_700: Color,
    pub neutral_800: Color,
    pub neutral_900: Color,
    pub neutral_950: Color,

    // Red
    pub red_50: Color,
    pub red_100: Color,
    pub red_200: Color,
    pub red_300: Color,
    pub red_400: Color,
    pub red_500: Color,
    pub red_600: Color,
    pub red_700: Color,
    pub red_800: Color,
    pub red_900: Color,
    pub red_950: Color,

    // Orange
    pub orange_50: Color,
    pub orange_100: Color,
    pub orange_200: Color,
    pub orange_300: Color,
    pub orange_400: Color,
    pub orange_500: Color,
    pub orange_600: Color,
    pub orange_700: Color,
    pub orange_800: Color,
    pub orange_900: Color,
    pub orange_950: Color,

    // Amber
    pub amber_50: Color,
    pub amber_100: Color,
    pub amber_200: Color,
    pub amber_300: Color,
    pub amber_400: Color,
    pub amber_500: Color,
    pub amber_600: Color,
    pub amber_700: Color,
    pub amber_800: Color,
    pub amber_900: Color,
    pub amber_950: Color,

    // Yellow
    pub yellow_50: Color,
    pub yellow_100: Color,
    pub yellow_200: Color,
    pub yellow_300: Color,
    pub yellow_400: Color,
    pub yellow_500: Color,
    pub yellow_600: Color,
    pub yellow_700: Color,
    pub yellow_800: Color,
    pub yellow_900: Color,
    pub yellow_950: Color,

    // Lime
    pub lime_50: Color,
    pub lime_100: Color,
    pub lime_200: Color,
    pub lime_300: Color,
    pub lime_400: Color,
    pub lime_500: Color,
    pub lime_600: Color,
    pub lime_700: Color,
    pub lime_800: Color,
    pub lime_900: Color,
    pub lime_950: Color,

    // Green
    pub green_50: Color,
    pub green_100: Color,
    pub green_200: Color,
    pub green_300: Color,
    pub green_400: Color,
    pub green_500: Color,
    pub green_600: Color,
    pub green_700: Color,
    pub green_800: Color,
    pub green_900: Color,
    pub green_950: Color,

    // Emerald
    pub emerald_50: Color,
    pub emerald_100: Color,
    pub emerald_200: Color,
    pub emerald_300: Color,
    pub emerald_400: Color,
    pub emerald_500: Color,
    pub emerald_600: Color,
    pub emerald_700: Color,
    pub emerald_800: Color,
    pub emerald_900: Color,
    pub emerald_950: Color,

    // Teal
    pub teal_50: Color,
    pub teal_100: Color,
    pub teal_200: Color,
    pub teal_300: Color,
    pub teal_400: Color,
    pub teal_500: Color,
    pub teal_600: Color,
    pub teal_700: Color,
    pub teal_800: Color,
    pub teal_900: Color,
    pub teal_950: Color,

    // Cyan
    pub cyan_50: Color,
    pub cyan_100: Color,
    pub cyan_200: Color,
    pub cyan_300: Color,
    pub cyan_400: Color,
    pub cyan_500: Color,
    pub cyan_600: Color,
    pub cyan_700: Color,
    pub cyan_800: Color,
    pub cyan_900: Color,
    pub cyan_950: Color,

    // Sky
    pub sky_50: Color,
    pub sky_100: Color,
    pub sky_200: Color,
    pub sky_300: Color,
    pub sky_400: Color,
    pub sky_500: Color,
    pub sky_600: Color,
    pub sky_700: Color,
    pub sky_800: Color,
    pub sky_900: Color,
    pub sky_950: Color,

    // Blue
    pub blue_50: Color,
    pub blue_100: Color,
    pub blue_200: Color,
    pub blue_300: Color,
    pub blue_400: Color,
    pub blue_500: Color,
    pub blue_600: Color,
    pub blue_700: Color,
    pub blue_800: Color,
    pub blue_900: Color,
    pub blue_950: Color,

    // Indigo
    pub indigo_50: Color,
    pub indigo_100: Color,
    pub indigo_200: Color,
    pub indigo_300: Color,
    pub indigo_400: Color,
    pub indigo_500: Color,
    pub indigo_600: Color,
    pub indigo_700: Color,
    pub indigo_800: Color,
    pub indigo_900: Color,
    pub indigo_950: Color,

    // Violet
    pub violet_50: Color,
    pub violet_100: Color,
    pub violet_200: Color,
    pub violet_300: Color,
    pub violet_400: Color,
    pub violet_500: Color,
    pub violet_600: Color,
    pub violet_700: Color,
    pub violet_800: Color,
    pub violet_900: Color,
    pub violet_950: Color,

    // Purple
    pub purple_50: Color,
    pub purple_100: Color,
    pub purple_200: Color,
    pub purple_300: Color,
    pub purple_400: Color,
    pub purple_500: Color,
    pub purple_600: Color,
    pub purple_700: Color,
    pub purple_800: Color,
    pub purple_900: Color,
    pub purple_950: Color,

    // Fuchsia
    pub fuchsia_50: Color,
    pub fuchsia_100: Color,
    pub fuchsia_200: Color,
    pub fuchsia_300: Color,
    pub fuchsia_400: Color,
    pub fuchsia_500: Color,
    pub fuchsia_600: Color,
    pub fuchsia_700: Color,
    pub fuchsia_800: Color,
    pub fuchsia_900: Color,
    pub fuchsia_950: Color,

    // Pink
    pub pink_50: Color,
    pub pink_100: Color,
    pub pink_200: Color,
    pub pink_300: Color,
    pub pink_400: Color,
    pub pink_500: Color,
    pub pink_600: Color,
    pub pink_700: Color,
    pub pink_800: Color,
    pub pink_900: Color,
    pub pink_950: Color,

    // Rose
    pub rose_50: Color,
    pub rose_100: Color,
    pub rose_200: Color,
    pub rose_300: Color,
    pub rose_400: Color,
    pub rose_500: Color,
    pub rose_600: Color,
    pub rose_700: Color,
    pub rose_800: Color,
    pub rose_900: Color,
    pub rose_950: Color,
}

impl Colors {
    /// Creates a new colour palette with all the default Tailwind‑style colours.
    pub fn new() -> Self {
        Self {
            // basic
            white: rgb(255, 255, 255),
            black: rgb(0, 0, 0),

            // Stone
            stone_50: rgb(250, 250, 249),
            stone_100: rgb(245, 245, 244),
            stone_200: rgb(231, 229, 228),
            stone_300: rgb(214, 211, 209),
            stone_400: rgb(166, 160, 155),
            stone_500: rgb(121, 113, 107),
            stone_600: rgb(87, 83, 77),
            stone_700: rgb(68, 64, 59),
            stone_800: rgb(41, 37, 36),
            stone_900: rgb(28, 25, 23),
            stone_950: rgb(12, 10, 9),

            // Mauve
            mauve_50: rgb(250, 250, 250),
            mauve_100: rgb(243, 241, 243),
            mauve_200: rgb(231, 228, 231),
            mauve_300: rgb(215, 208, 215),
            mauve_400: rgb(168, 158, 169),
            mauve_500: rgb(121, 105, 123),
            mauve_600: rgb(89, 76, 91),
            mauve_700: rgb(70, 57, 71),
            mauve_800: rgb(42, 33, 44),
            mauve_900: rgb(29, 22, 30),
            mauve_950: rgb(12, 9, 12),

            // Olive
            olive_50: rgb(251, 251, 249),
            olive_100: rgb(244, 244, 240),
            olive_200: rgb(232, 232, 227),
            olive_300: rgb(216, 216, 208),
            olive_400: rgb(171, 171, 156),
            olive_500: rgb(124, 124, 103),
            olive_600: rgb(91, 91, 75),
            olive_700: rgb(71, 71, 57),
            olive_800: rgb(43, 43, 34),
            olive_900: rgb(29, 29, 22),
            olive_950: rgb(12, 12, 9),

            // Mist
            mist_50: rgb(249, 251, 251),
            mist_100: rgb(241, 243, 243),
            mist_200: rgb(227, 231, 232),
            mist_300: rgb(208, 214, 216),
            mist_400: rgb(156, 168, 171),
            mist_500: rgb(103, 120, 124),
            mist_600: rgb(75, 88, 91),
            mist_700: rgb(57, 68, 71),
            mist_800: rgb(34, 41, 43),
            mist_900: rgb(22, 27, 29),
            mist_950: rgb(9, 11, 12),

            // Taupe
            taupe_50: rgb(251, 250, 249),
            taupe_100: rgb(243, 241, 241),
            taupe_200: rgb(232, 228, 227),
            taupe_300: rgb(216, 210, 208),
            taupe_400: rgb(171, 160, 156),
            taupe_500: rgb(124, 109, 103),
            taupe_600: rgb(91, 79, 75),
            taupe_700: rgb(71, 60, 57),
            taupe_800: rgb(43, 36, 34),
            taupe_900: rgb(29, 24, 22),
            taupe_950: rgb(12, 10, 9),

            // Slate
            slate_50: rgb(248, 250, 252),
            slate_100: rgb(241, 245, 249),
            slate_200: rgb(226, 232, 240),
            slate_300: rgb(203, 213, 225),
            slate_400: rgb(148, 163, 184),
            slate_500: rgb(100, 116, 139),
            slate_600: rgb(71, 85, 105),
            slate_700: rgb(51, 65, 85),
            slate_800: rgb(30, 41, 59),
            slate_900: rgb(15, 23, 42),
            slate_950: rgb(2, 6, 23),

            // Gray
            gray_50: rgb(249, 250, 251),
            gray_100: rgb(243, 244, 246),
            gray_200: rgb(229, 231, 235),
            gray_300: rgb(209, 213, 219),
            gray_400: rgb(156, 163, 175),
            gray_500: rgb(107, 114, 128),
            gray_600: rgb(75, 85, 99),
            gray_700: rgb(55, 65, 81),
            gray_800: rgb(31, 41, 55),
            gray_900: rgb(17, 24, 39),
            gray_950: rgb(3, 7, 18),

            // Zinc
            zinc_50: rgb(250, 250, 250),
            zinc_100: rgb(244, 244, 245),
            zinc_200: rgb(228, 228, 231),
            zinc_300: rgb(212, 212, 216),
            zinc_400: rgb(161, 161, 170),
            zinc_500: rgb(113, 113, 122),
            zinc_600: rgb(82, 82, 91),
            zinc_700: rgb(63, 63, 70),
            zinc_800: rgb(39, 39, 42),
            zinc_900: rgb(24, 24, 27),
            zinc_950: rgb(9, 9, 11),

            // Neutral
            neutral_50: rgb(250, 250, 250),
            neutral_100: rgb(245, 245, 245),
            neutral_200: rgb(229, 229, 229),
            neutral_300: rgb(212, 212, 212),
            neutral_400: rgb(163, 163, 163),
            neutral_500: rgb(115, 115, 115),
            neutral_600: rgb(82, 82, 82),
            neutral_700: rgb(64, 64, 64),
            neutral_800: rgb(38, 38, 38),
            neutral_900: rgb(23, 23, 23),
            neutral_950: rgb(10, 10, 10),

            // Red
            red_50: rgb(254, 242, 242),
            red_100: rgb(254, 226, 226),
            red_200: rgb(254, 202, 202),
            red_300: rgb(252, 165, 165),
            red_400: rgb(248, 113, 113),
            red_500: rgb(239, 68, 68),
            red_600: rgb(220, 38, 38),
            red_700: rgb(185, 28, 28),
            red_800: rgb(153, 27, 27),
            red_900: rgb(127, 29, 29),
            red_950: rgb(69, 10, 10),

            // Orange
            orange_50: rgb(255, 247, 237),
            orange_100: rgb(255, 237, 213),
            orange_200: rgb(254, 215, 170),
            orange_300: rgb(253, 186, 116),
            orange_400: rgb(251, 146, 60),
            orange_500: rgb(249, 115, 22),
            orange_600: rgb(234, 88, 12),
            orange_700: rgb(194, 65, 12),
            orange_800: rgb(154, 52, 18),
            orange_900: rgb(124, 45, 18),
            orange_950: rgb(67, 20, 7),

            // Amber
            amber_50: rgb(255, 251, 235),
            amber_100: rgb(254, 243, 199),
            amber_200: rgb(253, 230, 138),
            amber_300: rgb(252, 211, 77),
            amber_400: rgb(251, 191, 36),
            amber_500: rgb(245, 158, 11),
            amber_600: rgb(217, 119, 6),
            amber_700: rgb(180, 83, 9),
            amber_800: rgb(146, 64, 14),
            amber_900: rgb(120, 53, 15),
            amber_950: rgb(69, 26, 3),

            // Yellow
            yellow_50: rgb(254, 252, 232),
            yellow_100: rgb(254, 249, 195),
            yellow_200: rgb(254, 240, 138),
            yellow_300: rgb(253, 224, 71),
            yellow_400: rgb(250, 204, 21),
            yellow_500: rgb(234, 179, 8),
            yellow_600: rgb(202, 138, 4),
            yellow_700: rgb(161, 98, 7),
            yellow_800: rgb(133, 77, 14),
            yellow_900: rgb(113, 63, 18),
            yellow_950: rgb(66, 32, 6),

            // Lime
            lime_50: rgb(247, 254, 231),
            lime_100: rgb(236, 252, 203),
            lime_200: rgb(217, 249, 157),
            lime_300: rgb(190, 242, 100),
            lime_400: rgb(163, 230, 53),
            lime_500: rgb(132, 204, 22),
            lime_600: rgb(101, 163, 13),
            lime_700: rgb(77, 124, 15),
            lime_800: rgb(63, 98, 18),
            lime_900: rgb(54, 83, 20),
            lime_950: rgb(26, 46, 5),

            // Green
            green_50: rgb(240, 253, 244),
            green_100: rgb(220, 252, 231),
            green_200: rgb(187, 247, 208),
            green_300: rgb(134, 239, 172),
            green_400: rgb(74, 222, 128),
            green_500: rgb(34, 197, 94),
            green_600: rgb(22, 163, 74),
            green_700: rgb(21, 128, 61),
            green_800: rgb(22, 101, 52),
            green_900: rgb(20, 83, 45),
            green_950: rgb(5, 46, 22),

            // Emerald
            emerald_50: rgb(236, 253, 245),
            emerald_100: rgb(209, 250, 229),
            emerald_200: rgb(167, 243, 208),
            emerald_300: rgb(110, 231, 183),
            emerald_400: rgb(52, 211, 153),
            emerald_500: rgb(16, 185, 129),
            emerald_600: rgb(5, 150, 105),
            emerald_700: rgb(4, 120, 87),
            emerald_800: rgb(6, 95, 70),
            emerald_900: rgb(6, 78, 59),
            emerald_950: rgb(2, 44, 34),

            // Teal
            teal_50: rgb(240, 253, 250),
            teal_100: rgb(204, 251, 241),
            teal_200: rgb(153, 246, 228),
            teal_300: rgb(94, 234, 212),
            teal_400: rgb(45, 212, 191),
            teal_500: rgb(20, 184, 166),
            teal_600: rgb(13, 148, 136),
            teal_700: rgb(15, 118, 110),
            teal_800: rgb(17, 94, 89),
            teal_900: rgb(19, 78, 74),
            teal_950: rgb(4, 47, 46),

            // Cyan
            cyan_50: rgb(236, 254, 255),
            cyan_100: rgb(207, 250, 254),
            cyan_200: rgb(165, 243, 252),
            cyan_300: rgb(103, 232, 249),
            cyan_400: rgb(34, 211, 238),
            cyan_500: rgb(6, 182, 212),
            cyan_600: rgb(8, 145, 178),
            cyan_700: rgb(14, 116, 144),
            cyan_800: rgb(21, 94, 117),
            cyan_900: rgb(22, 78, 99),
            cyan_950: rgb(8, 51, 68),

            // Sky
            sky_50: rgb(240, 249, 255),
            sky_100: rgb(224, 242, 254),
            sky_200: rgb(186, 230, 253),
            sky_300: rgb(125, 211, 252),
            sky_400: rgb(56, 189, 248),
            sky_500: rgb(14, 165, 233),
            sky_600: rgb(2, 132, 199),
            sky_700: rgb(3, 105, 161),
            sky_800: rgb(7, 89, 133),
            sky_900: rgb(12, 74, 110),
            sky_950: rgb(8, 47, 73),

            // Blue
            blue_50: rgb(239, 246, 255),
            blue_100: rgb(219, 234, 254),
            blue_200: rgb(191, 219, 254),
            blue_300: rgb(147, 197, 253),
            blue_400: rgb(96, 165, 250),
            blue_500: rgb(59, 130, 246),
            blue_600: rgb(37, 99, 235),
            blue_700: rgb(29, 78, 216),
            blue_800: rgb(30, 64, 175),
            blue_900: rgb(30, 58, 138),
            blue_950: rgb(23, 37, 84),

            // Indigo
            indigo_50: rgb(238, 242, 255),
            indigo_100: rgb(224, 231, 255),
            indigo_200: rgb(199, 210, 254),
            indigo_300: rgb(165, 180, 252),
            indigo_400: rgb(129, 140, 248),
            indigo_500: rgb(99, 102, 241),
            indigo_600: rgb(79, 70, 229),
            indigo_700: rgb(67, 56, 202),
            indigo_800: rgb(55, 48, 163),
            indigo_900: rgb(49, 46, 129),
            indigo_950: rgb(30, 27, 75),

            // Violet
            violet_50: rgb(245, 243, 255),
            violet_100: rgb(237, 233, 254),
            violet_200: rgb(221, 214, 254),
            violet_300: rgb(196, 181, 253),
            violet_400: rgb(167, 139, 250),
            violet_500: rgb(139, 92, 246),
            violet_600: rgb(124, 58, 237),
            violet_700: rgb(109, 40, 217),
            violet_800: rgb(91, 33, 182),
            violet_900: rgb(76, 29, 149),
            violet_950: rgb(46, 16, 101),

            // Purple
            purple_50: rgb(250, 245, 255),
            purple_100: rgb(243, 232, 255),
            purple_200: rgb(233, 213, 255),
            purple_300: rgb(216, 180, 254),
            purple_400: rgb(192, 132, 252),
            purple_500: rgb(168, 85, 247),
            purple_600: rgb(147, 51, 234),
            purple_700: rgb(126, 34, 206),
            purple_800: rgb(107, 33, 168),
            purple_900: rgb(88, 28, 135),
            purple_950: rgb(59, 7, 100),

            // Fuchsia
            fuchsia_50: rgb(253, 244, 255),
            fuchsia_100: rgb(250, 232, 255),
            fuchsia_200: rgb(245, 208, 254),
            fuchsia_300: rgb(240, 171, 252),
            fuchsia_400: rgb(232, 121, 249),
            fuchsia_500: rgb(217, 70, 239),
            fuchsia_600: rgb(192, 38, 211),
            fuchsia_700: rgb(162, 28, 175),
            fuchsia_800: rgb(134, 25, 143),
            fuchsia_900: rgb(112, 26, 117),
            fuchsia_950: rgb(74, 4, 78),

            // Pink
            pink_50: rgb(253, 242, 248),
            pink_100: rgb(252, 231, 243),
            pink_200: rgb(251, 207, 232),
            pink_300: rgb(249, 168, 212),
            pink_400: rgb(244, 114, 182),
            pink_500: rgb(236, 72, 153),
            pink_600: rgb(219, 39, 119),
            pink_700: rgb(190, 24, 93),
            pink_800: rgb(157, 23, 77),
            pink_900: rgb(131, 24, 67),
            pink_950: rgb(80, 7, 36),

            // Rose
            rose_50: rgb(255, 241, 242),
            rose_100: rgb(255, 228, 230),
            rose_200: rgb(254, 205, 211),
            rose_300: rgb(253, 164, 175),
            rose_400: rgb(251, 113, 133),
            rose_500: rgb(244, 63, 94),
            rose_600: rgb(225, 29, 72),
            rose_700: rgb(190, 18, 60),
            rose_800: rgb(159, 18, 57),
            rose_900: rgb(136, 19, 55),
            rose_950: rgb(76, 5, 25),
        }
    }
}

impl Default for Colors {
    fn default() -> Self {
        Self::new()
    }
}

// ------------------------------------------------------------------------
// Semantic theme
// ------------------------------------------------------------------------

/// Semantic FreyaCN theme.
///
/// This is the central theme object used by your UI components. It is built from a
/// `Colors` palette and defines the semantic colours for backgrounds, text, borders,
/// interactive elements, and more.
///
/// All colour fields are public and intended to be read directly. The theme also carries
/// a `is_dark` flag and the full palette (`colors`) for when you need to access specific
/// shade values.
///
/// # Example
/// ```
/// # use freyacn::theme::Theme;
/// let theme = Theme::base_color("slate", false).theme_color("blue");
/// let background = theme.background;
/// let primary = theme.primary;
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Theme {
    /// Whether the theme is in dark mode.
    pub is_dark: bool,
    /// The full colour palette.
    pub colors: Colors,

    // ----- Core -----
    /// Main background colour.
    pub background: Color,
    /// Main foreground (text) colour.
    pub foreground: Color,

    // ----- Primary -----
    /// Primary accent colour (e.g., main call‑to‑action).
    pub primary: Color,
    /// Foreground colour that contrasts with `primary`.
    pub primary_foreground: Color,

    // ----- Secondary -----
    /// Secondary background colour (less prominent than `primary`).
    pub secondary: Color,
    /// Foreground colour that contrasts with `secondary`.
    pub secondary_foreground: Color,

    // ----- Muted -----
    /// Muted background (often used for subtle backgrounds or disabled states).
    pub muted: Color,
    /// Foreground colour that contrasts with `muted`.
    pub muted_foreground: Color,

    // ----- Accent -----
    /// Accent colour (similar to `primary` but may be used for less prominent actions).
    pub accent: Color,
    /// Foreground colour that contrasts with `accent`.
    pub accent_foreground: Color,

    // ----- Destructive -----
    /// Destructive colour (e.g., for delete or warning actions).
    pub destructive: Color,
    /// Foreground colour that contrasts with `destructive`.
    pub destructive_foreground: Color,

    // ----- Card / Popover -----
    /// Background for card‑like surfaces.
    pub card: Color,
    /// Foreground colour that contrasts with `card`.
    pub card_foreground: Color,

    /// Background for popover surfaces.
    pub popover: Color,
    /// Foreground colour that contrasts with `popover`.
    pub popover_foreground: Color,

    // ----- Borders & Inputs -----
    /// Border colour for general use.
    pub border: Color,
    /// Border colour for input fields.
    pub input: Color,
    /// Focus ring colour.
    pub ring: Color,

    // ----- Charts -----
    /// Primary chart colour.
    pub chart_1: Color,
    /// Secondary chart colour.
    pub chart_2: Color,
    /// Tertiary chart colour.
    pub chart_3: Color,
    /// Quaternary chart colour.
    pub chart_4: Color,
    /// Quinary chart colour.
    pub chart_5: Color,
}

impl Theme {
    /// Helper to get the luminance of a color (0.0 – 1.0).
    fn luminance(color: Color) -> f32 {
        let r = color.r() as f32 / 255.0;
        let g = color.g() as f32 / 255.0;
        let b = color.b() as f32 / 255.0;
        0.299 * r + 0.587 * g + 0.114 * b
    }

    /// Build a theme from a **base palette** (neutral colours) and a dark/light mode.
    ///
    /// This sets the `background`, `foreground`, `muted`, `muted_foreground`, `border`,
    /// `input`, `card`, `popover`, `secondary`, `secondary_foreground` to the appropriate
    /// shades of the chosen base palette.
    ///
    /// The `primary`, `primary_foreground`, `accent`, `accent_foreground`, and `ring`
    /// are initially set to the base palette's primary shades (900/50 for light,
    /// 50/900 for dark). You can override them later with `.theme_color()`.
    ///
    /// # Arguments
    /// - `base`: the name of the base palette (e.g., `"slate"`, `"stone"`, `"neutral"`).
    /// - `dark`: whether the theme should be dark (`true`) or light (`false`).
    ///
    /// # Panics
    /// Panics if the `base` palette name is unknown.
    pub fn base_color(base: &str, dark: bool) -> Self {
        let colors = Colors::default();
        let (bg, fg, muted, muted_fg, border, input, card, popover, secondary, secondary_fg) =
            if dark {
                match base {
                    "slate" => (
                        colors.slate_950,
                        colors.slate_50,
                        colors.slate_800,
                        colors.slate_400,
                        colors.slate_800,
                        colors.slate_800,
                        colors.slate_950,
                        colors.slate_950,
                        colors.slate_800,
                        colors.slate_50,
                    ),
                    "stone" => (
                        colors.stone_950,
                        colors.stone_50,
                        colors.stone_800,
                        colors.stone_400,
                        colors.stone_800,
                        colors.stone_800,
                        colors.stone_950,
                        colors.stone_950,
                        colors.stone_800,
                        colors.stone_50,
                    ),
                    "mauve" => (
                        colors.mauve_950,
                        colors.mauve_50,
                        colors.mauve_800,
                        colors.mauve_400,
                        colors.mauve_800,
                        colors.mauve_800,
                        colors.mauve_950,
                        colors.mauve_950,
                        colors.mauve_800,
                        colors.mauve_50,
                    ),
                    "olive" => (
                        colors.olive_950,
                        colors.olive_50,
                        colors.olive_800,
                        colors.olive_400,
                        colors.olive_800,
                        colors.olive_800,
                        colors.olive_950,
                        colors.olive_950,
                        colors.olive_800,
                        colors.olive_50,
                    ),
                    "mist" => (
                        colors.mist_950,
                        colors.mist_50,
                        colors.mist_800,
                        colors.mist_400,
                        colors.mist_800,
                        colors.mist_800,
                        colors.mist_950,
                        colors.mist_950,
                        colors.mist_800,
                        colors.mist_50,
                    ),
                    "taupe" => (
                        colors.taupe_950,
                        colors.taupe_50,
                        colors.taupe_800,
                        colors.taupe_400,
                        colors.taupe_800,
                        colors.taupe_800,
                        colors.taupe_950,
                        colors.taupe_950,
                        colors.taupe_800,
                        colors.taupe_50,
                    ),
                    "zinc" => (
                        colors.zinc_950,
                        colors.zinc_50,
                        colors.zinc_800,
                        colors.zinc_400,
                        colors.zinc_800,
                        colors.zinc_800,
                        colors.zinc_950,
                        colors.zinc_950,
                        colors.zinc_800,
                        colors.zinc_50,
                    ),
                    "neutral" => (
                        colors.neutral_950,
                        colors.neutral_50,
                        colors.neutral_800,
                        colors.neutral_400,
                        colors.neutral_800,
                        colors.neutral_800,
                        colors.neutral_950,
                        colors.neutral_950,
                        colors.neutral_800,
                        colors.neutral_50,
                    ),
                    _ => panic!("Unknown base palette: {}", base),
                }
            } else {
                match base {
                    "slate" => (
                        colors.slate_50,
                        colors.slate_950,
                        colors.slate_100,
                        colors.slate_500,
                        colors.slate_200,
                        colors.slate_200,
                        colors.white,
                        colors.white,
                        colors.slate_100,
                        colors.slate_900,
                    ),
                    "stone" => (
                        colors.stone_50,
                        colors.stone_950,
                        colors.stone_100,
                        colors.stone_500,
                        colors.stone_200,
                        colors.stone_200,
                        colors.white,
                        colors.white,
                        colors.stone_100,
                        colors.stone_900,
                    ),
                    "mauve" => (
                        colors.mauve_50,
                        colors.mauve_950,
                        colors.mauve_100,
                        colors.mauve_500,
                        colors.mauve_200,
                        colors.mauve_200,
                        colors.white,
                        colors.white,
                        colors.mauve_100,
                        colors.mauve_900,
                    ),
                    "olive" => (
                        colors.olive_50,
                        colors.olive_950,
                        colors.olive_100,
                        colors.olive_500,
                        colors.olive_200,
                        colors.olive_200,
                        colors.white,
                        colors.white,
                        colors.olive_100,
                        colors.olive_900,
                    ),
                    "mist" => (
                        colors.mist_50,
                        colors.mist_950,
                        colors.mist_100,
                        colors.mist_500,
                        colors.mist_200,
                        colors.mist_200,
                        colors.white,
                        colors.white,
                        colors.mist_100,
                        colors.mist_900,
                    ),
                    "taupe" => (
                        colors.taupe_50,
                        colors.taupe_950,
                        colors.taupe_100,
                        colors.taupe_500,
                        colors.taupe_200,
                        colors.taupe_200,
                        colors.white,
                        colors.white,
                        colors.taupe_100,
                        colors.taupe_900,
                    ),
                    "zinc" => (
                        colors.zinc_50,
                        colors.zinc_950,
                        colors.zinc_100,
                        colors.zinc_500,
                        colors.zinc_200,
                        colors.zinc_200,
                        colors.white,
                        colors.white,
                        colors.zinc_100,
                        colors.zinc_900,
                    ),
                    "neutral" => (
                        colors.neutral_50,
                        colors.neutral_950,
                        colors.neutral_100,
                        colors.neutral_500,
                        colors.neutral_200,
                        colors.neutral_200,
                        colors.white,
                        colors.white,
                        colors.neutral_100,
                        colors.neutral_900,
                    ),
                    _ => panic!("Unknown base palette: {}", base),
                }
            };

        // Default primary from the base palette.
        let (primary, primary_fg) = if dark {
            match base {
                "slate" => (colors.slate_50, colors.slate_900),
                "stone" => (colors.stone_50, colors.stone_900),
                "mauve" => (colors.mauve_50, colors.mauve_900),
                "olive" => (colors.olive_50, colors.olive_900),
                "mist" => (colors.mist_50, colors.mist_900),
                "taupe" => (colors.taupe_50, colors.taupe_900),
                "zinc" => (colors.zinc_50, colors.zinc_900),
                "neutral" => (colors.neutral_50, colors.neutral_900),
                _ => panic!("Unknown base palette: {}", base),
            }
        } else {
            match base {
                "slate" => (colors.slate_900, colors.slate_50),
                "stone" => (colors.stone_900, colors.stone_50),
                "mauve" => (colors.mauve_900, colors.mauve_50),
                "olive" => (colors.olive_900, colors.olive_50),
                "mist" => (colors.mist_900, colors.mist_50),
                "taupe" => (colors.taupe_900, colors.taupe_50),
                "zinc" => (colors.zinc_900, colors.zinc_50),
                "neutral" => (colors.neutral_900, colors.neutral_50),
                _ => panic!("Unknown base palette: {}", base),
            }
        };

        let accent = primary;
        let accent_fg = primary_fg;
        let ring = primary;

        let destructive = if dark { colors.red_800 } else { colors.red_500 };
        let destructive_fg = colors.white;

        let chart_1 = colors.blue_500;
        let chart_2 = colors.green_500;
        let chart_3 = colors.yellow_500;
        let chart_4 = colors.red_500;
        let chart_5 = colors.violet_500;

        Self {
            is_dark: dark,
            colors,
            background: bg,
            foreground: fg,
            primary,
            primary_foreground: primary_fg,
            secondary,
            secondary_foreground: secondary_fg,
            muted,
            muted_foreground: muted_fg,
            accent,
            accent_foreground: accent_fg,
            destructive,
            destructive_foreground: destructive_fg,
            card,
            card_foreground: fg,
            popover,
            popover_foreground: fg,
            border,
            input,
            ring,
            chart_1,
            chart_2,
            chart_3,
            chart_4,
            chart_5,
        }
    }

    /// Override the **accent (theme) colour** using any palette from `Colors`.
    ///
    /// This sets the `primary`, `primary_foreground`, `accent`, `accent_foreground`,
    /// and `ring` fields to the appropriate shades of the chosen theme palette.
    ///
    /// The shade used depends on the theme’s `is_dark` flag:
    /// - **Light mode**: uses the 500 shade of the palette (e.g., `blue_500`).
    /// - **Dark mode**: uses the 400 shade of the palette (e.g., `blue_400`).
    ///
    /// The `primary_foreground` is automatically set to either `white` or `black`
    /// based on the luminance of the chosen primary colour to ensure sufficient contrast.
    ///
    /// # Arguments
    /// - `theme`: the name of the accent palette (e.g., `"blue"`, `"red"`, `"violet"`).
    ///
    /// # Panics
    /// Panics if the `theme` name is unknown.
    pub fn theme_color(self, theme: &str) -> Self {
        let colors = self.colors;
        let primary = if self.is_dark {
            match theme {
                "stone" => colors.stone_400,
                "mauve" => colors.mauve_400,
                "olive" => colors.olive_400,
                "mist" => colors.mist_400,
                "taupe" => colors.taupe_400,
                "slate" => colors.slate_400,
                "gray" => colors.gray_400,
                "zinc" => colors.zinc_400,
                "neutral" => colors.neutral_50,
                "red" => colors.red_400,
                "orange" => colors.orange_400,
                "amber" => colors.amber_400,
                "yellow" => colors.yellow_400,
                "lime" => colors.lime_400,
                "green" => colors.green_400,
                "emerald" => colors.emerald_400,
                "teal" => colors.teal_400,
                "cyan" => colors.cyan_400,
                "sky" => colors.sky_400,
                "blue" => colors.blue_400,
                "indigo" => colors.indigo_400,
                "violet" => colors.violet_400,
                "purple" => colors.purple_400,
                "fuchsia" => colors.fuchsia_400,
                "pink" => colors.pink_400,
                "rose" => colors.rose_400,
                _ => panic!("Unknown theme palette: {}", theme),
            }
        } else {
            match theme {
                "stone" => colors.stone_950,
                "mauve" => colors.mauve_950,
                "olive" => colors.olive_950,
                "mist" => colors.mist_950,
                "taupe" => colors.taupe_950,
                "slate" => colors.slate_950,
                "gray" => colors.gray_500,
                "zinc" => colors.zinc_500,
                "neutral" => colors.neutral_950,
                "red" => colors.red_500,
                "orange" => colors.orange_500,
                "amber" => colors.amber_500,
                "yellow" => colors.yellow_500,
                "lime" => colors.lime_500,
                "green" => colors.green_500,
                "emerald" => colors.emerald_500,
                "teal" => colors.teal_500,
                "cyan" => colors.cyan_500,
                "sky" => colors.sky_500,
                "blue" => colors.blue_500,
                "indigo" => colors.indigo_500,
                "violet" => colors.violet_500,
                "purple" => colors.purple_500,
                "fuchsia" => colors.fuchsia_500,
                "pink" => colors.pink_500,
                "rose" => colors.rose_500,
                _ => panic!("Unknown theme palette: {}", theme),
            }
        };

        let primary_fg = if Self::luminance(primary) < 0.5 {
            colors.white
        } else {
            colors.black
        };

        let accent = primary;
        let accent_fg = primary_fg;
        let ring = primary;

        Self {
            primary,
            primary_foreground: primary_fg,
            accent,
            accent_foreground: accent_fg,
            ring,
            ..self
        }
    }

    // ------------------------------------------------------------------------
    // Convenience constructors for each base palette, accepting a theme.
    // ------------------------------------------------------------------------

    /// Creates a theme with the `stone` base and the given accent theme.
    pub fn stone(dark: bool, theme: &str) -> Self {
        Self::base_color("stone", dark).theme_color(theme)
    }
    /// Creates a theme with the `mauve` base and the given accent theme.
    pub fn mauve(dark: bool, theme: &str) -> Self {
        Self::base_color("mauve", dark).theme_color(theme)
    }
    /// Creates a theme with the `olive` base and the given accent theme.
    pub fn olive(dark: bool, theme: &str) -> Self {
        Self::base_color("olive", dark).theme_color(theme)
    }
    /// Creates a theme with the `mist` base and the given accent theme.
    pub fn mist(dark: bool, theme: &str) -> Self {
        Self::base_color("mist", dark).theme_color(theme)
    }
    /// Creates a theme with the `taupe` base and the given accent theme.
    pub fn taupe(dark: bool, theme: &str) -> Self {
        Self::base_color("taupe", dark).theme_color(theme)
    }
    /// Creates a theme with the `zinc` base and the given accent theme.
    pub fn zinc(dark: bool, theme: &str) -> Self {
        Self::base_color("zinc", dark).theme_color(theme)
    }
    /// Creates a theme with the `neutral` base and the given accent theme.
    pub fn neutral(dark: bool, theme: &str) -> Self {
        Self::base_color("neutral", dark).theme_color(theme)
    }
}

// ================================================================
// Hooks – Theme access and initialization
// ================================================================

/// Provides a custom FreyaCN theme, reusing an existing context if one already exists.
///
/// If a theme context is already present in the current scope, this will update it
/// with the new theme value; otherwise, it creates a new context.
///
/// This is the recommended way to set the root theme for your application.
///
/// # Example
/// ```
/// # use freyacn::theme::{use_init_cn_theme, Theme};
/// # use freya::prelude::*;
/// fn App() -> impl IntoElement {
///     let theme = Theme::base_color("slate", false).theme_color("blue");
///     use_init_cn_theme(theme);
///
///     // Your app components...
///     rect().expanded().child(MyComponent())
/// }
/// ```
pub fn use_init_cn_theme(theme: Theme) -> State<Theme> {
    use_hook(|| {
        if let Some(mut existing) = try_consume_context::<State<Theme>>() {
            existing.set(theme);
            existing
        } else {
            let state = State::create(theme);
            provide_context(state);
            state
        }
    })
}

/// Provides a custom FreyaCN theme, always creating a new context.
///
/// Unlike `use_init_cn_theme`, this will never reuse an existing context.
/// Use this when you need to override the theme for a subtree, regardless
/// of any parent theme.
///
/// # Example
/// ```
/// # use freyacn::theme::{use_provide_cn_theme, Theme};
/// # use freya::prelude::*;
/// fn DarkSubtree() -> impl IntoElement {
///     let dark_theme = Theme::base_color("zinc", true).theme_color("rose");
///     use_provide_cn_theme(dark_theme);
///     // Children here will see the dark theme.
/// }
/// ```
pub fn use_provide_cn_theme(theme: Theme) -> State<Theme> {
    use_hook(|| {
        let state = State::create(theme);
        provide_context(state);
        state
    })
}

/// Subscribes to the current FreyaCN theme.
///
/// This hook will panic if no theme has been provided in the context.
/// For a fallback that doesn't panic, use `get_cn_theme_or_default()` instead.
///
/// # Example
/// ```
/// # use freyacn::theme::use_cn_theme;
/// # use freya::prelude::*;
/// fn MyButton() -> impl IntoElement {
///     let theme = use_cn_theme();
///     let background = theme.background;
///     // ...
/// }
/// ```
pub fn use_cn_theme() -> State<Theme> {
    use_consume::<State<Theme>>()
}

/// Subscribes to the current FreyaCN theme, falling back to a default theme if none is provided.
///
/// The default theme is a light slate theme with slate accent.
/// This is useful for built‑in components that should work even without an explicit theme provider.
///
/// # Example
/// ```
/// # use freyacn::theme::get_cn_theme_or_default;
/// # use freya::prelude::*;
/// fn MyComponent() -> impl IntoElement {
///     let theme = get_cn_theme_or_default();
///     // theme will always be valid
/// }
/// ```
pub fn get_cn_theme_or_default() -> Readable<Theme> {
    try_consume_context::<State<Theme>>()
        .map(|v| v.into())
        .unwrap_or_else(|| {
            Theme::base_color("slate", false)
                .theme_color("slate")
                .into()
        })
}
