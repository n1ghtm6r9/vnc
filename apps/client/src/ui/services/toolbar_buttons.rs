use egui::{Button, Label, Ui};

use crate::{
    ACTUAL_SIZE_BUTTON, BALANCED_QUALITY, DISCONNECT_BUTTON, FAST_QUALITY, FIT_BUTTON, FULLSCREEN_BUTTON,
    ImageQuality, LOSSLESS_QUALITY, OverlayAction, QUALITY_LABEL, REFRESH_BUTTON, SOUND_HERE, SOUND_LABEL,
    SOUND_NEEDS_HIGH_PERFORMANCE, SOUND_ON_MAC, SWAP_CONTROL_AND_COMMAND_BUTTON, ScaleMode, SoundPlace, ToolbarView,
};

pub fn toolbar_buttons(ui: &mut Ui, view: &ToolbarView, actions: &mut Vec<OverlayAction>) {
    for (mode, label) in [(ScaleMode::Fit, FIT_BUTTON), (ScaleMode::Actual, ACTUAL_SIZE_BUTTON)] {
        if ui.selectable_label(view.scale_mode == mode, label).clicked() {
            actions.push(OverlayAction::ScaleMode(mode));
        }
    }
    ui.separator();
    if ui.selectable_label(view.fullscreen, FULLSCREEN_BUTTON).clicked() {
        actions.push(OverlayAction::Fullscreen(!view.fullscreen));
    }
    if ui
        .selectable_label(view.swap_control_and_command, SWAP_CONTROL_AND_COMMAND_BUTTON)
        .clicked()
    {
        actions.push(OverlayAction::SwapControlAndCommand(!view.swap_control_and_command));
    }
    ui.separator();
    ui.label(QUALITY_LABEL);
    for (quality, label) in [
        (ImageQuality::Lossless, LOSSLESS_QUALITY),
        (ImageQuality::Balanced, BALANCED_QUALITY),
        (ImageQuality::Fast, FAST_QUALITY),
    ] {
        if ui.selectable_label(view.quality == quality, label).clicked() {
            actions.push(OverlayAction::Quality(quality));
        }
    }
    ui.separator();
    ui.add_enabled(view.sound_streamed, Label::new(SOUND_LABEL));
    for (place, label) in [(SoundPlace::Here, SOUND_HERE), (SoundPlace::Mac, SOUND_ON_MAC)] {
        let button = ui.add_enabled(view.sound_streamed, Button::selectable(view.sound == place, label));
        if button.on_disabled_hover_text(SOUND_NEEDS_HIGH_PERFORMANCE).clicked() {
            actions.push(OverlayAction::Sound(place));
        }
    }
    ui.separator();
    if ui.button(REFRESH_BUTTON).clicked() {
        actions.push(OverlayAction::Refresh);
    }
    if ui.button(DISCONNECT_BUTTON).clicked() {
        actions.push(OverlayAction::Disconnect);
    }
    if let Some(stats) = &view.stats {
        ui.separator();
        ui.label(stats.as_str());
    }
}
