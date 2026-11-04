//! Ferro file-manager screens on fresco — R9 spike (headless-verifiable).
//!
//! Renders the Connect and Browser screens through the stock widget
//! renderer into paint-kit surfaces; goldens land in `goldens/` and the
//! interaction model (focus walk -> activate -> action id) is asserted
//! headlessly. `--features live` adds the real-API probe binary.

use ferro_fresco_spike::{browser_tree, connect_tree, demo_entries, ACTION_CONNECT};
use std::path::Path;
use widget_kit::WidgetColors;

fn main() {
    let colors = WidgetColors::default();
    let size = layout_kit::engine::Size2D { width: 420.0, height: 640.0 };

    // ── Connect screen ────────────────────────────────────────────────
    let mut connect = connect_tree();
    connect.compute(size);
    let mut surface = paint_kit::Surface::new(420, 640).expect("surface");
    let mut painter = paint_kit::Painter::default();
    widget_kit::render::paint_tree(&connect, &mut surface, &mut painter, &colors);
    render_conformance::write_bmp(
        Path::new("goldens/connect.bmp"),
        &render_conformance::SurfaceRef::new(surface.data(), surface.width(), surface.height(), surface.stride()),
    ).expect("write connect.bmp");
    println!("connect fnv1a: {}", render_conformance::fnv1a(surface.data()));

    // interaction model: walk focus to the Connect button, activate, take actions
    connect.compute(size);
    let mut button: Option<widget_kit::WidgetRef> = None;
    for _ in 0..12 {
        connect.focus_next();
        if let Some(r) = connect.focus() {
            if let Some(w) = connect.widget(r) {
                if let widget_kit::WidgetKind::Button { action, .. } = &w.kind {
                    if *action == ACTION_CONNECT {
                        button = Some(r);
                        break;
                    }
                }
            }
        }
    }
    let Some(btn) = button else {
        panic!("connect button not reachable through the focus chain");
    };
    connect.activate(btn);
    let actions = connect.take_actions();
    println!("connect actions after activate: {actions:?}");
    assert!(
        actions.contains(&ACTION_CONNECT),
        "action {ACTION_CONNECT} must fire through the focus chain"
    );

    // ── Browser screen ────────────────────────────────────────────────
    let entries = demo_entries();
    let mut browser = browser_tree(&entries);
    browser.compute(size);
    let mut surface2 = paint_kit::Surface::new(420, 640).expect("surface2");
    widget_kit::render::paint_tree(&browser, &mut surface2, &mut painter, &colors);
    render_conformance::write_bmp(
        Path::new("goldens/browser.bmp"),
        &render_conformance::SurfaceRef::new(surface2.data(), surface2.width(), surface2.height(), surface2.stride()),
    ).expect("write browser.bmp");
    println!("browser fnv1a: {}", render_conformance::fnv1a(surface2.data()));
}
