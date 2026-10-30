use egui::{RichText, Stroke, vec2};

use super::{Dialog, Json, SheetApp};
use crate::{SaveAnswer, i18n, theme};

/// Keep the destructive choice separate from Cancel and the primary Save action.
pub(super) fn save_changes(app: &mut SheetApp, ui: &mut egui::Ui, d: &Dialog, open: &mut bool) {
    let lang = i18n::current(ui.ctx());
    let tokens = theme::Tokens::get(ui.ctx());
    let title = d.values.get("title").and_then(Json::as_str).unwrap_or(lang.tr("this workbook"));
    let quitting = d.values.get("quit").and_then(Json::as_bool) == Some(true);
    let mut answer = None;
    let mut save_as = None;
    egui::Frame::new().inner_margin(20).show(ui, |ui| {
        ui.spacing_mut().item_spacing = vec2(10.0, 10.0);
        ui.label(RichText::new(lang.tr("Save before closing?")).size(19.0).strong());
        ui.label(RichText::new(lang.tr("This workbook has unsaved changes.")).color(tokens.text_dim));
        ui.add_space(4.0);
        egui::Frame::new().fill(tokens.ribbon).stroke(Stroke::new(1.0, tokens.separator)).corner_radius(8).inner_margin(14).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.add(egui::Label::new(RichText::new(title).strong().size(15.0)).truncate()).on_hover_text(title);
        });
        ui.label(RichText::new(lang.tr("If you don't save, your changes will be lost.")).color(tokens.text_dim));
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(2.0);
        ui.horizontal(|ui| {
            if ui
                .add(egui::Button::new(RichText::new(lang.tr("Don't Save")).color(tokens.danger)).min_size(vec2(112.0, 36.0)).corner_radius(6))
                .clicked()
            {
                answer = Some(SaveAnswer::DontSave);
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add(
                        egui::Button::new(RichText::new(lang.tr("Save")).color(tokens.on_accent))
                            .fill(tokens.accent)
                            .min_size(vec2(112.0, 36.0))
                            .corner_radius(6),
                    )
                    .clicked()
                {
                    answer = Some(SaveAnswer::Save);
                }
                if ui.add(egui::Button::new(lang.tr("Cancel")).min_size(vec2(96.0, 36.0)).corner_radius(6)).clicked() {
                    answer = Some(SaveAnswer::Cancel);
                }
                ui.scope(|ui| {
                    ui.spacing_mut().interact_size = vec2(112.0, 36.0);
                    ui.spacing_mut().button_padding = vec2(12.0, 6.0);
                    ui.visuals_mut().widgets.inactive.corner_radius = egui::CornerRadius::same(6);
                    ui.visuals_mut().widgets.hovered.corner_radius = egui::CornerRadius::same(6);
                    ui.visuals_mut().widgets.active.corner_radius = egui::CornerRadius::same(6);
                    ui.menu_button(lang.tr("Save As…"), |ui| {
                        for (format, label) in crate::backstage::SAVE_TYPES {
                            if ui.button(lang.tr(label)).clicked() {
                                save_as = Some(format);
                                ui.close();
                            }
                        }
                    });
                });
            });
        });
    });
    if let Some(format) = save_as
        && app.save_as_format(Some(format))
        && app.session.active().is_some_and(|doc| !doc.is_dirty())
    {
        // Only a full-workbook save may complete the close; CSV/PDF exports keep the prompt.
        *open = !app.answer_save_changes(SaveAnswer::DontSave, quitting, ui.ctx());
    }
    if let Some(answer) = answer
        && app.answer_save_changes(answer, quitting, ui.ctx())
    {
        *open = false;
    }
}
