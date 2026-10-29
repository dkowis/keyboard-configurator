use crate::fl;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// A mode/pattern for the keyboard's LEDs to display
#[non_exhaustive]
pub struct Mode {
    /// Index (as used in firmware)
    pub index: u8,
    /// Textual ID of mode, for `ListBox` or debugging
    pub id: &'static str,
    /// Display name of mode
    pub name: String,
    /// Hue setting has effect in this mode
    pub has_hue: bool,
    /// Speed setting has effect in this mode
    pub has_speed: bool,
}

impl Mode {
    const fn new(
        index: u8,
        id: &'static str,
        name: String,
        has_hue: bool,
        has_speed: bool,
    ) -> Self {
        Self {
            index,
            id,
            name,
            has_hue,
            has_speed,
        }
    }

    /// Return slice of all `Mode`s, ordered as they should be displayed
    pub fn all() -> &'static [Mode] {
        &MODES
    }

    /// Get `Mode` corresponding to index (as used in firmware)
    pub fn from_index(index: u8) -> Option<&'static Self> {
        static MODE_BY_INDEX: Lazy<HashMap<u8, &Mode>> =
            Lazy::new(|| MODES.iter().map(|i| (i.index, i)).collect());
        MODE_BY_INDEX.get(&index).cloned()
    }

    /// Get `Mode` corresponding to textual ID
    pub fn from_id(id: &str) -> Option<&'static Self> {
        static MODE_BY_ID: Lazy<HashMap<&str, &Mode>> =
            Lazy::new(|| MODES.iter().map(|i| (i.id, i)).collect());
        MODE_BY_ID.get(&id).cloned()
    }

    /// `true` for Per Key mode, otherwise `false`
    pub fn is_per_key(&self) -> bool {
        self.index == 1
    }

    pub fn is_disabled(&self) -> bool {
        self.index == 14
    }
}

static MODES: Lazy<Vec<Mode>> = Lazy::new(|| {
    vec![
        Mode::new(14, "DISABLED", fl!("mode-disabled"), false, false),
        Mode::new(0, "SOLID_COLOR", fl!("mode-solid-color"), true, false),
        Mode::new(1, "PER_KEY", fl!("mode-per-key"), true, false),
        Mode::new(13, "ACTIVE_KEYS", fl!("mode-active-keys"), true, false),
        Mode::new(2, "CYCLE_ALL", fl!("mode-cycle-all"), false, true),
        Mode::new(
            3,
            "CYCLE_LEFT_RIGHT",
            fl!("mode-cycle-left-right"),
            false,
            true,
        ),
        Mode::new(4, "CYCLE_UP_DOWN", fl!("mode-cycle-up-down"), false, true),
        Mode::new(5, "CYCLE_OUT_IN", fl!("mode-cycle-out-in"), false, true),
        Mode::new(
            6,
            "CYCLE_OUT_IN_DUAL",
            fl!("mode-cycle-out-in-dual"),
            false,
            true,
        ),
        Mode::new(
            7,
            "RAINBOW_MOVING_CHEVRON",
            fl!("mode-rainbow-moving-chevron"),
            false,
            true,
        ),
        Mode::new(8, "CYCLE_PINWHEEL", fl!("mode-cycle-pinwheel"), false, true),
        Mode::new(9, "CYCLE_SPIRAL", fl!("mode-cycle-spiral"), false, true),
        Mode::new(10, "RAINDROPS", fl!("mode-raindrops"), false, false),
        Mode::new(11, "SPLASH", fl!("mode-splash"), false, true),
        Mode::new(12, "MULTISPLASH", fl!("mode-multisplash"), false, true),
        Mode::new(15, "ALPHAS_MODS", fl!("mode-alphas-mods"), true, true),
        Mode::new(
            16,
            "GRADIENT_UP_DOWN",
            fl!("mode-gradient-up-down"),
            true,
            true,
        ),
        Mode::new(
            17,
            "GRADIENT_LEFT_RIGHT",
            fl!("mode-gradient-left-right"),
            true,
            true,
        ),
        Mode::new(18, "BREATHING", fl!("mode-breathing"), true, true),
        Mode::new(19, "BAND_SAT", fl!("mode-band-sat"), true, true),
        Mode::new(20, "BAND_VAL", fl!("mode-band-val"), true, true),
        Mode::new(
            21,
            "BAND_PINWHEEL_SAT",
            fl!("mode-band-pinwheel-sat"),
            true,
            true,
        ),
        Mode::new(
            22,
            "BAND_PINWHEEL_VAL",
            fl!("mode-band-pinwheel-val"),
            true,
            true,
        ),
        Mode::new(
            23,
            "BAND_SPIRAL_SAT",
            fl!("mode-band-spiral-sat"),
            true,
            true,
        ),
        Mode::new(
            24,
            "BAND_SPIRAL_VAL",
            fl!("mode-band-spiral-val"),
            true,
            true,
        ),
        Mode::new(25, "DUAL_BEACON", fl!("mode-dual-beacon"), true, true),
        Mode::new(
            26,
            "RAINBOW_BEACON",
            fl!("mode-rainbow-beacon"),
            false,
            true,
        ),
        Mode::new(
            27,
            "RAINBOW_PINWHEELS",
            fl!("mode-rainbow-pinwheels"),
            false,
            true,
        ),
        Mode::new(
            28,
            "JELLYBEAN_RAINDROPS",
            fl!("mode-jellybean-raindrops"),
            false,
            true,
        ),
        Mode::new(29, "HUE_BREATHING", fl!("mode-hue-breathing"), true, true),
        Mode::new(30, "HUE_PENDULUM", fl!("mode-hue-pendulum"), true, true),
        Mode::new(31, "HUE_WAVE", fl!("mode-hue-wave"), true, true),
        Mode::new(32, "PIXEL_RAIN", fl!("mode-pixel-rain"), false, true),
        Mode::new(33, "PIXEL_FLOW", fl!("mode-pixel-flow"), false, true),
        Mode::new(34, "PIXEL_FRACTAL", fl!("mode-pixel-fractal"), true, true),
        Mode::new(
            35,
            "TYPING_HEATMAP",
            fl!("mode-typing-heatmap"),
            false,
            false,
        ),
        Mode::new(36, "DIGITAL_RAIN", fl!("mode-digital-rain"), false, false),
        Mode::new(
            37,
            "SOLID_REACTIVE_SIMPLE",
            fl!("mode-solid-reactive-simple"),
            true,
            true,
        ),
        Mode::new(38, "SOLID_REACTIVE", fl!("mode-solid-reactive"), true, true),
        Mode::new(
            39,
            "SOLID_REACTIVE_WIDE",
            fl!("mode-solid-reactive-wide"),
            true,
            true,
        ),
        Mode::new(
            40,
            "SOLID_REACTIVE_MULTIWIDE",
            fl!("mode-solid-reactive-multiwide"),
            true,
            true,
        ),
        Mode::new(
            41,
            "SOLID_REACTIVE_CROSS",
            fl!("mode-solid-reactive-cross"),
            true,
            true,
        ),
        Mode::new(
            42,
            "SOLID_REACTIVE_MULTICROSS",
            fl!("mode-solid-reactive-multicross"),
            true,
            true,
        ),
        Mode::new(
            43,
            "SOLID_REACTIVE_NEXUS",
            fl!("mode-solid-reactive-nexus"),
            true,
            true,
        ),
        Mode::new(
            44,
            "SOLID_REACTIVE_MULTINEXUS",
            fl!("mode-solid-reactive-multinexus"),
            true,
            true,
        ),
        Mode::new(45, "SOLID_SPLASH", fl!("mode-solid-splash"), true, true),
        Mode::new(
            46,
            "SOLID_MULTISPLASH",
            fl!("mode-solid-multisplash"),
            true,
            true,
        ),
    ]
});
