use eframe::egui;
use std::process::Command;
use std::io::Write;

const MONO_NAME: &str = "mono_output";

fn pactl_stdout(args: &[&str]) -> String {
    Command::new("pactl").args(args).output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

fn pactl_ok(args: &[&str]) -> bool {
    Command::new("pactl").args(args).status()
        .map(|s| s.success()).unwrap_or(false)
}

fn unload_remaps() {
    let mods = pactl_stdout(&["list", "short", "modules"]);
    for line in mods.lines() {
        if line.contains("remap-sink") {
            if let Some(id) = line.split_whitespace().next() {
                let _ = Command::new("pactl").args(["unload-module", id]).status();
            }
        }
    }
}

fn move_streams(target: &str) {
    let inputs = pactl_stdout(&["list", "short", "sink-inputs"]);
    for line in inputs.lines() {
        if let Some(id) = line.split_whitespace().next() {
            let _ = Command::new("pactl").args(["move-sink-input", id, target]).status();
        }
    }
}

fn is_mono() -> bool {
    pactl_stdout(&["list", "short", "modules"]).contains("remap-sink")
}

struct AudioSwitcher {
    status: String,
}

impl Default for AudioSwitcher {
    fn default() -> Self {
        let mode = if is_mono() { "Mono" } else { "Stereo" };
        Self { status: format!("Current mode: {}", mode) }
    }
}

impl eframe::App for AudioSwitcher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.heading(egui::RichText::new("Audio Mode").strong().size(24.0));
                ui.add_space(10.0);
                ui.label(&self.status);
                ui.add_space(30.0);

                let btn_label = if is_mono() { "Switch to Stereo" } else { "Switch to Mono" };

                if ui.add(egui::Button::new(btn_label).min_size(egui::vec2(200.0, 50.0))).clicked() {
                    eprintln!("BUTTON CLICKED - is_mono: {}", is_mono());
                    std::io::stderr().flush().ok();

                    if is_mono() {
                        // Switch to stereo
                        unload_remaps();
                        let master = pactl_stdout(&["list", "short", "sinks"])
                            .lines()
                            .filter(|l| !l.contains("mono") && !l.contains("Monitor"))
                            .find_map(|l| l.split_whitespace().nth(1).map(|s| s.to_string()))
                            .unwrap_or_else(|| "alsa_output.pci-0000_00_1b.0.analog-stereo".to_string());
                        pactl_ok(&["set-default-sink", &master]);
                        move_streams(&master);
                        self.status = "Switched to Stereo".into();
                        eprintln!("Switched to Stereo, master: {}", master);
                    } else {
                        // Switch to mono
                        let master = pactl_stdout(&["get-default-sink"]);
                        eprintln!("Master sink: {}", master);
                        unload_remaps();
                        let ok = pactl_ok(&[
                            "load-module", "module-remap-sink",
                            "sink_name=mono_output",
                            &format!("master={}", master),
                            "channels=1", "channel_map=mono"
                        ]);
                        eprintln!("Load module result: {}", ok);
                        if ok {
                            pactl_ok(&["set-default-sink", "mono_output"]);
                            move_streams("mono_output");
                            self.status = "Switched to Mono".into();
                            eprintln!("Switched to Mono");
                        } else {
                            self.status = "Error: failed to create mono sink".into();
                        }
                    }
                }

                ui.add_space(40.0);
                ui.label(egui::RichText::new("Mono / Stereo Switcher").small().color(egui::Color32::GRAY));
            });
        });

        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_resizable(false)
            .with_title("Audio Mode"),
        ..Default::default()
    };
    eframe::run_native("Audio Mode", options, Box::new(|_cc| Ok(Box::new(AudioSwitcher::default()))))
}
