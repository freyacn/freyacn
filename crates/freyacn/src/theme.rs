//! # FreyaCN Theme System
//!
//! Aligned with **shadcn/ui v4** (`globals.css`, neutral base).
//!
//! ## Token values
//!
//! Every semantic token in the `neutral` base resolves to the exact
//! shadcn OKLCH value converted to sRGB. Other bases (`slate`, `stone`,
//! `zinc`, …) derive their surfaces from the Tailwind palette using the
//! same *structure* — `background < card < popover = muted = secondary
//! < accent` in dark, inverted in light.
//!
//! | Token | Light | Dark |
//! |---|---|---|
//! | `background` | `#ffffff` | `rgb(10,10,10)` |
//! | `foreground` | `rgb(10,10,10)` | `rgb(250,250,250)` |
//! | `card` | `#ffffff` | `rgb(23,23,23)` |
//! | `popover` | `#ffffff` | `rgb(38,38,38)` |
//! | `primary` | `rgb(23,23,23)` | `rgb(229,229,229)` |
//! | `secondary` | `rgb(245,245,245)` | `rgb(38,38,38)` |
//! | `muted` | `rgb(245,245,245)` | `rgb(38,38,38)` |
//! | `muted-foreground` | `rgb(115,115,115)` | `rgb(161,161,161)` |
//! | `accent` | `rgb(245,245,245)` | `rgb(64,64,64)` |
//! | `destructive` | `rgb(231,0,11)` | `rgb(255,100,103)` |
//! | `border` | `rgb(229,229,229)` | `white / 10%` |
//! | `input` | `rgb(229,229,229)` | `white / 15%` |
//! | `ring` | `rgb(161,161,161)` | `rgb(115,115,115)` |
//!
//! ## Radius
//!
//! shadcn uses a single `--radius: 0.625rem` (10px) with derived steps:
//! `sm = 6px`, `md = 8px`, `lg = 10px`, `xl = 14px`. `Theme::radius_lg`
//! is what `rounded-lg` maps to in shadcn — components that want an
//! exact shadcn corner should read it rather than hard-coding `8.0`.

use freya::prelude::*;
use freya::prelude::{
    Readable, State, provide_context, try_consume_context, use_consume, use_hook,
};
use paste::paste;

/// Convert an RGB tuple into a Freya Color.
const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgb(r, g, b)
}

// ------------------------------------------------------------------------
// Shadcn-exact tokens
// ------------------------------------------------------------------------

/// Exact shadcn/ui v4 token colours, converted from the OKLCH values in
/// `globals.css` to sRGB. Only the `neutral` base uses these; other
/// bases derive from the Tailwind palette.
mod shadcn {
    use super::{Color, rgb};

    // ---- Light (`.root`) ----
    pub const LIGHT_BACKGROUND: Color = rgb(255, 255, 255);
    pub const LIGHT_FOREGROUND: Color = rgb(10, 10, 10);
    pub const LIGHT_CARD: Color = rgb(255, 255, 255);
    pub const LIGHT_CARD_FOREGROUND: Color = rgb(10, 10, 10);
    pub const LIGHT_POPOVER: Color = rgb(255, 255, 255);
    pub const LIGHT_POPOVER_FOREGROUND: Color = rgb(10, 10, 10);
    pub const LIGHT_PRIMARY: Color = rgb(23, 23, 23);
    pub const LIGHT_PRIMARY_FOREGROUND: Color = rgb(250, 250, 250);
    pub const LIGHT_SECONDARY: Color = rgb(245, 245, 245);
    pub const LIGHT_SECONDARY_FOREGROUND: Color = rgb(23, 23, 23);
    pub const LIGHT_MUTED: Color = rgb(245, 245, 245);
    pub const LIGHT_MUTED_FOREGROUND: Color = rgb(115, 115, 115);
    pub const LIGHT_ACCENT: Color = rgb(245, 245, 245);
    pub const LIGHT_ACCENT_FOREGROUND: Color = rgb(23, 23, 23);
    pub const LIGHT_DESTRUCTIVE: Color = rgb(231, 0, 11);
    pub const LIGHT_BORDER: Color = rgb(229, 229, 229);
    pub const LIGHT_INPUT: Color = rgb(229, 229, 229);
    pub const LIGHT_RING: Color = rgb(161, 161, 161);
    pub const LIGHT_SIDEBAR: Color = rgb(250, 250, 250);
    pub const LIGHT_SIDEBAR_FOREGROUND: Color = rgb(10, 10, 10);
    pub const LIGHT_SIDEBAR_PRIMARY: Color = rgb(23, 23, 23);
    pub const LIGHT_SIDEBAR_PRIMARY_FOREGROUND: Color = rgb(250, 250, 250);
    pub const LIGHT_SIDEBAR_ACCENT: Color = rgb(245, 245, 245);
    pub const LIGHT_SIDEBAR_ACCENT_FOREGROUND: Color = rgb(23, 23, 23);
    pub const LIGHT_SIDEBAR_BORDER: Color = rgb(229, 229, 229);
    pub const LIGHT_SIDEBAR_RING: Color = rgb(161, 161, 161);

    pub const LIGHT_CHART_1: Color = rgb(245, 74, 0);
    pub const LIGHT_CHART_2: Color = rgb(0, 150, 137);
    pub const LIGHT_CHART_3: Color = rgb(16, 78, 100);
    pub const LIGHT_CHART_4: Color = rgb(255, 185, 0);
    pub const LIGHT_CHART_5: Color = rgb(254, 153, 0);

    // ---- Dark (`.dark`) ----
    pub const DARK_BACKGROUND: Color = rgb(10, 10, 10);
    pub const DARK_FOREGROUND: Color = rgb(250, 250, 250);
    pub const DARK_CARD: Color = rgb(23, 23, 23);
    pub const DARK_CARD_FOREGROUND: Color = rgb(250, 250, 250);
    pub const DARK_POPOVER: Color = rgb(38, 38, 38);
    pub const DARK_POPOVER_FOREGROUND: Color = rgb(250, 250, 250);
    pub const DARK_PRIMARY: Color = rgb(229, 229, 229);
    pub const DARK_PRIMARY_FOREGROUND: Color = rgb(23, 23, 23);
    pub const DARK_SECONDARY: Color = rgb(38, 38, 38);
    pub const DARK_SECONDARY_FOREGROUND: Color = rgb(250, 250, 250);
    pub const DARK_MUTED: Color = rgb(38, 38, 38);
    pub const DARK_MUTED_FOREGROUND: Color = rgb(161, 161, 161);
    pub const DARK_ACCENT: Color = rgb(64, 64, 64);
    pub const DARK_ACCENT_FOREGROUND: Color = rgb(250, 250, 250);
    pub const DARK_DESTRUCTIVE: Color = rgb(255, 100, 103);

    /// `oklch(1 0 0 / 10%)` — white at 10% alpha.
    pub const DARK_BORDER: Color = Color::from_argb(26, 255, 255, 255);
    /// `oklch(1 0 0 / 15%)` — white at 15% alpha.
    pub const DARK_INPUT: Color = Color::from_argb(38, 255, 255, 255);

    pub const DARK_RING: Color = rgb(115, 115, 115);

    pub const DARK_SIDEBAR: Color = rgb(23, 23, 23);
    pub const DARK_SIDEBAR_FOREGROUND: Color = rgb(250, 250, 250);
    pub const DARK_SIDEBAR_PRIMARY: Color = rgb(20, 71, 230);
    pub const DARK_SIDEBAR_PRIMARY_FOREGROUND: Color = rgb(250, 250, 250);
    pub const DARK_SIDEBAR_ACCENT: Color = rgb(38, 38, 38);
    pub const DARK_SIDEBAR_ACCENT_FOREGROUND: Color = rgb(250, 250, 250);
    pub const DARK_SIDEBAR_BORDER: Color = Color::from_argb(26, 255, 255, 255);
    pub const DARK_SIDEBAR_RING: Color = rgb(82, 82, 82);

    pub const DARK_CHART_1: Color = rgb(20, 71, 230);
    pub const DARK_CHART_2: Color = rgb(0, 188, 125);
    pub const DARK_CHART_3: Color = rgb(254, 153, 0);
    pub const DARK_CHART_4: Color = rgb(173, 70, 255);
    pub const DARK_CHART_5: Color = rgb(255, 31, 86);
}

// ------------------------------------------------------------------------
// Color palette
// ------------------------------------------------------------------------

/// Complete FreyaCN color palette (Tailwind/shadcn style).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Colors {
    pub white: Color,
    pub black: Color,

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
    /// Creates a new colour palette with all the default Tailwind-style colours.
    pub fn new() -> Self {
        Self {
            white: rgb(255, 255, 255),
            black: rgb(0, 0, 0),

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
// Palette-derived surfaces
// ------------------------------------------------------------------------

/// Internal bundle of the surface tokens for a given base palette.
struct SurfaceTokens {
    bg: Color,
    fg: Color,
    card: Color,
    popover: Color,
    muted: Color,
    muted_fg: Color,
    secondary: Color,
    secondary_fg: Color,
    accent: Color,
    accent_fg: Color,
    border: Color,
    input: Color,
    ring: Color,
    primary: Color,
    primary_fg: Color,
}

/// Build shadcn-shaped surfaces from a Tailwind palette family.
///
/// Dark: `bg 950 → card 900 → popover/muted/secondary 800 → accent 700`,
/// with `ring = 500` and `primary = 200`. Light: `bg 50 → muted/secondary/
/// accent 100 → border 200`, with `card = white`, `ring = 400`, and
/// `primary = 900`.
macro_rules! surface_tokens {
    ($c:expr, $family:ident, $dark:expr) => {{
        let c = $c;
        if $dark {
            paste! {
                SurfaceTokens {
                    bg: c.[<$family _950>],
                    fg: c.[<$family _50>],
                    card: c.[<$family _900>],
                    popover: c.[<$family _800>],
                    muted: c.[<$family _800>],
                    muted_fg: c.[<$family _400>],
                    secondary: c.[<$family _800>],
                    secondary_fg: c.[<$family _50>],
                    accent: c.[<$family _700>],
                    accent_fg: c.[<$family _50>],
                    border: c.[<$family _800>],
                    input: c.[<$family _800>],
                    ring: c.[<$family _500>],
                    primary: c.[<$family _200>],
                    primary_fg: c.[<$family _900>],
                }
            }
        } else {
            paste! {
                SurfaceTokens {
                    bg: c.[<$family _50>],
                    fg: c.[<$family _950>],
                    card: c.white,
                    popover: c.white,
                    muted: c.[<$family _100>],
                    muted_fg: c.[<$family _500>],
                    secondary: c.[<$family _100>],
                    secondary_fg: c.[<$family _900>],
                    accent: c.[<$family _100>],
                    accent_fg: c.[<$family _900>],
                    border: c.[<$family _200>],
                    input: c.[<$family _200>],
                    ring: c.[<$family _400>],
                    primary: c.[<$family _900>],
                    primary_fg: c.[<$family _50>],
                }
            }
        }
    }};
}

// ------------------------------------------------------------------------
// Semantic theme
// ------------------------------------------------------------------------

/// Semantic FreyaCN theme, aligned with shadcn/ui v4.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Theme {
    pub is_dark: bool,
    pub colors: Colors,

    pub background: Color,
    pub foreground: Color,

    pub primary: Color,
    pub primary_foreground: Color,

    pub secondary: Color,
    pub secondary_foreground: Color,

    pub muted: Color,
    pub muted_foreground: Color,

    pub accent: Color,
    pub accent_foreground: Color,

    pub destructive: Color,
    pub destructive_foreground: Color,

    pub card: Color,
    pub card_foreground: Color,

    pub popover: Color,
    pub popover_foreground: Color,

    pub border: Color,
    pub input: Color,
    pub ring: Color,

    pub chart_1: Color,
    pub chart_2: Color,
    pub chart_3: Color,
    pub chart_4: Color,
    pub chart_5: Color,

    pub sidebar: Color,
    pub sidebar_foreground: Color,
    pub sidebar_primary: Color,
    pub sidebar_primary_foreground: Color,
    pub sidebar_accent: Color,
    pub sidebar_accent_foreground: Color,
    pub sidebar_border: Color,
    pub sidebar_ring: Color,

    pub disabled: Color,
    pub disabled_foreground: Color,

    pub radius_sm: f32,
    pub radius_md: f32,
    pub radius_lg: f32,
    pub radius_xl: f32,
}

impl Theme {
    fn luminance(color: Color) -> f32 {
        let r = color.r() as f32 / 255.0;
        let g = color.g() as f32 / 255.0;
        let b = color.b() as f32 / 255.0;
        0.299 * r + 0.587 * g + 0.114 * b
    }

    /// Start with a **light** theme using shadcn's default `neutral` base.
    pub fn light() -> Self {
        Self::from_base("neutral", false)
    }

    /// Start with a **dark** theme using shadcn's default `neutral` base.
    pub fn dark() -> Self {
        Self::from_base("neutral", true)
    }

    /// Change the base palette, keeping the current light/dark mode.
    pub fn base_color(self, base: &str) -> Self {
        Self::from_base(base, self.is_dark)
    }

    /// Build a theme from a base palette and a dark/light flag.
    ///
    /// `"neutral"` produces shadcn's exact `globals.css` values; every
    /// other palette derives the same surface *structure* from the
    /// Tailwind scale.
    ///
    /// # Panics
    /// Panics if `base` is not a known palette name.
    pub fn from_base(base: &str, dark: bool) -> Self {
        let colors = Colors::default();

        if base == "neutral" {
            return Self::from_shadcn_neutral(colors, dark);
        }

        let t = match base {
            "slate" => surface_tokens!(colors, slate, dark),
            "stone" => surface_tokens!(colors, stone, dark),
            "mauve" => surface_tokens!(colors, mauve, dark),
            "olive" => surface_tokens!(colors, olive, dark),
            "mist" => surface_tokens!(colors, mist, dark),
            "taupe" => surface_tokens!(colors, taupe, dark),
            "zinc" => surface_tokens!(colors, zinc, dark),
            "gray" => surface_tokens!(colors, gray, dark),
            _ => panic!("Unknown base palette: {}", base),
        };

        let destructive = if dark { colors.red_400 } else { colors.red_600 };
        let chart_1 = if dark {
            shadcn::DARK_CHART_1
        } else {
            shadcn::LIGHT_CHART_1
        };
        let chart_2 = if dark {
            shadcn::DARK_CHART_2
        } else {
            shadcn::LIGHT_CHART_2
        };
        let chart_3 = if dark {
            shadcn::DARK_CHART_3
        } else {
            shadcn::LIGHT_CHART_3
        };
        let chart_4 = if dark {
            shadcn::DARK_CHART_4
        } else {
            shadcn::LIGHT_CHART_4
        };
        let chart_5 = if dark {
            shadcn::DARK_CHART_5
        } else {
            shadcn::LIGHT_CHART_5
        };

        Self {
            is_dark: dark,
            colors,
            background: t.bg,
            foreground: t.fg,
            primary: t.primary,
            primary_foreground: t.primary_fg,
            secondary: t.secondary,
            secondary_foreground: t.secondary_fg,
            muted: t.muted,
            muted_foreground: t.muted_fg,
            accent: t.accent,
            accent_foreground: t.accent_fg,
            destructive,
            destructive_foreground: colors.white,
            card: t.card,
            card_foreground: t.fg,
            popover: t.popover,
            popover_foreground: t.fg,
            border: t.border,
            input: t.input,
            ring: t.ring,
            chart_1,
            chart_2,
            chart_3,
            chart_4,
            chart_5,
            sidebar: t.bg,
            sidebar_foreground: t.fg,
            sidebar_primary: t.primary,
            sidebar_primary_foreground: t.primary_fg,
            sidebar_accent: t.accent,
            sidebar_accent_foreground: t.accent_fg,
            sidebar_border: t.border,
            sidebar_ring: t.ring,
            disabled: t.muted,
            disabled_foreground: t.muted_fg,
            radius_sm: 6.0,
            radius_md: 8.0,
            radius_lg: 10.0,
            radius_xl: 14.0,
        }
    }

    /// Build the exact shadcn/ui `neutral` theme.
    fn from_shadcn_neutral(colors: Colors, dark: bool) -> Self {
        if dark {
            Self {
                is_dark: true,
                colors,
                background: shadcn::DARK_BACKGROUND,
                foreground: shadcn::DARK_FOREGROUND,
                primary: shadcn::DARK_PRIMARY,
                primary_foreground: shadcn::DARK_PRIMARY_FOREGROUND,
                secondary: shadcn::DARK_SECONDARY,
                secondary_foreground: shadcn::DARK_SECONDARY_FOREGROUND,
                muted: shadcn::DARK_MUTED,
                muted_foreground: shadcn::DARK_MUTED_FOREGROUND,
                accent: shadcn::DARK_ACCENT,
                accent_foreground: shadcn::DARK_ACCENT_FOREGROUND,
                destructive: shadcn::DARK_DESTRUCTIVE,
                destructive_foreground: colors.white,
                card: shadcn::DARK_CARD,
                card_foreground: shadcn::DARK_CARD_FOREGROUND,
                popover: shadcn::DARK_POPOVER,
                popover_foreground: shadcn::DARK_POPOVER_FOREGROUND,
                border: shadcn::DARK_BORDER,
                input: shadcn::DARK_INPUT,
                ring: shadcn::DARK_RING,
                chart_1: shadcn::DARK_CHART_1,
                chart_2: shadcn::DARK_CHART_2,
                chart_3: shadcn::DARK_CHART_3,
                chart_4: shadcn::DARK_CHART_4,
                chart_5: shadcn::DARK_CHART_5,
                sidebar: shadcn::DARK_SIDEBAR,
                sidebar_foreground: shadcn::DARK_SIDEBAR_FOREGROUND,
                sidebar_primary: shadcn::DARK_SIDEBAR_PRIMARY,
                sidebar_primary_foreground: shadcn::DARK_SIDEBAR_PRIMARY_FOREGROUND,
                sidebar_accent: shadcn::DARK_SIDEBAR_ACCENT,
                sidebar_accent_foreground: shadcn::DARK_SIDEBAR_ACCENT_FOREGROUND,
                sidebar_border: shadcn::DARK_SIDEBAR_BORDER,
                sidebar_ring: shadcn::DARK_SIDEBAR_RING,
                disabled: shadcn::DARK_MUTED,
                disabled_foreground: shadcn::DARK_MUTED_FOREGROUND,
                radius_sm: 6.0,
                radius_md: 8.0,
                radius_lg: 10.0,
                radius_xl: 14.0,
            }
        } else {
            Self {
                is_dark: false,
                colors,
                background: shadcn::LIGHT_BACKGROUND,
                foreground: shadcn::LIGHT_FOREGROUND,
                primary: shadcn::LIGHT_PRIMARY,
                primary_foreground: shadcn::LIGHT_PRIMARY_FOREGROUND,
                secondary: shadcn::LIGHT_SECONDARY,
                secondary_foreground: shadcn::LIGHT_SECONDARY_FOREGROUND,
                muted: shadcn::LIGHT_MUTED,
                muted_foreground: shadcn::LIGHT_MUTED_FOREGROUND,
                accent: shadcn::LIGHT_ACCENT,
                accent_foreground: shadcn::LIGHT_ACCENT_FOREGROUND,
                destructive: shadcn::LIGHT_DESTRUCTIVE,
                destructive_foreground: colors.white,
                card: shadcn::LIGHT_CARD,
                card_foreground: shadcn::LIGHT_CARD_FOREGROUND,
                popover: shadcn::LIGHT_POPOVER,
                popover_foreground: shadcn::LIGHT_POPOVER_FOREGROUND,
                border: shadcn::LIGHT_BORDER,
                input: shadcn::LIGHT_INPUT,
                ring: shadcn::LIGHT_RING,
                chart_1: shadcn::LIGHT_CHART_1,
                chart_2: shadcn::LIGHT_CHART_2,
                chart_3: shadcn::LIGHT_CHART_3,
                chart_4: shadcn::LIGHT_CHART_4,
                chart_5: shadcn::LIGHT_CHART_5,
                sidebar: shadcn::LIGHT_SIDEBAR,
                sidebar_foreground: shadcn::LIGHT_SIDEBAR_FOREGROUND,
                sidebar_primary: shadcn::LIGHT_SIDEBAR_PRIMARY,
                sidebar_primary_foreground: shadcn::LIGHT_SIDEBAR_PRIMARY_FOREGROUND,
                sidebar_accent: shadcn::LIGHT_SIDEBAR_ACCENT,
                sidebar_accent_foreground: shadcn::LIGHT_SIDEBAR_ACCENT_FOREGROUND,
                sidebar_border: shadcn::LIGHT_SIDEBAR_BORDER,
                sidebar_ring: shadcn::LIGHT_SIDEBAR_RING,
                disabled: shadcn::LIGHT_MUTED,
                disabled_foreground: shadcn::LIGHT_MUTED_FOREGROUND,
                radius_sm: 6.0,
                radius_md: 8.0,
                radius_lg: 10.0,
                radius_xl: 14.0,
            }
        }
    }

    /// Override the accent colour from a Tailwind palette family.
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

        Self {
            primary,
            primary_foreground: primary_fg,
            accent: primary,
            accent_foreground: primary_fg,
            ring: primary,
            sidebar_primary: primary,
            sidebar_primary_foreground: primary_fg,
            sidebar_accent: primary,
            sidebar_accent_foreground: primary_fg,
            sidebar_ring: primary,
            ..self
        }
    }

    // ---- Base palette shortcuts ----
    pub fn stone(self) -> Self {
        self.base_color("stone")
    }
    pub fn mauve(self) -> Self {
        self.base_color("mauve")
    }
    pub fn olive(self) -> Self {
        self.base_color("olive")
    }
    pub fn mist(self) -> Self {
        self.base_color("mist")
    }
    pub fn taupe(self) -> Self {
        self.base_color("taupe")
    }
    pub fn slate(self) -> Self {
        self.base_color("slate")
    }
    pub fn gray(self) -> Self {
        self.base_color("gray")
    }
    pub fn zinc(self) -> Self {
        self.base_color("zinc")
    }
    pub fn neutral(self) -> Self {
        self.base_color("neutral")
    }

    // ---- Radius overrides ----
    pub fn radius_sm(mut self, value: f32) -> Self {
        self.radius_sm = value;
        self
    }
    pub fn radius_md(mut self, value: f32) -> Self {
        self.radius_md = value;
        self
    }
    pub fn radius_lg(mut self, value: f32) -> Self {
        self.radius_lg = value;
        self
    }
    pub fn radius_xl(mut self, value: f32) -> Self {
        self.radius_xl = value;
        self
    }
    pub fn radius(mut self, value: f32) -> Self {
        self.radius_sm = (value - 4.0).max(0.0);
        self.radius_md = (value - 2.0).max(0.0);
        self.radius_lg = value;
        self.radius_xl = value + 4.0;
        self
    }
}

// ================================================================
// Hooks
// ================================================================

/// Provides a custom FreyaCN theme, reusing an existing context if one
/// already exists.
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
pub fn use_provide_cn_theme(theme: Theme) -> State<Theme> {
    use_hook(|| {
        let state = State::create(theme);
        provide_context(state);
        state
    })
}

/// Subscribes to the current FreyaCN theme.
pub fn use_cn_theme() -> State<Theme> {
    use_consume::<State<Theme>>()
}

/// Subscribes to the current FreyaCN theme, falling back to a default
/// light neutral theme if none is provided.
pub fn get_cn_theme_or_default() -> Readable<Theme> {
    try_consume_context::<State<Theme>>()
        .map(|v| v.into())
        .unwrap_or_else(|| Theme::light().into())
}
