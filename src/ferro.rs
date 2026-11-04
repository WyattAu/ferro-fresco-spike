//! Ferro screens composed on fresco widget-kit (R9 spike).

use ui_text_edit::{StringBuffer, TextField as TextEditField};
use widget_kit::{TreeItem, WidgetKind, WidgetTree};

/// Action id: the Connect button on the connect screen.
pub const ACTION_CONNECT: u32 = 1;
/// Action id: Share on the selected file.
pub const ACTION_SHARE: u32 = 2;
/// Action id: Upload to the current folder.
pub const ACTION_UPLOAD: u32 = 3;
/// Action id: Delete the selected file.
pub const ACTION_DELETE: u32 = 4;

/// Connect screen: server URL + credentials + connect button.
pub fn connect_tree() -> WidgetTree {
    let mut t = WidgetTree::new();
    let root = t.push(WidgetKind::Column { gap: 10.0 });

    let brand = t.push(WidgetKind::Label {
        text: "FERRO — native client (fresco spike)".into(),
        px_size: 14.0,
    });
    t.add_child(root, brand);

    let server = t.push(WidgetKind::TextField {
        placeholder: "https://ferro.example.com".into(),
        field: TextEditField::new(StringBuffer::new()),
    });
    t.add_child(root, server);

    let user = t.push(WidgetKind::TextField {
        placeholder: "username".into(),
        field: TextEditField::new(StringBuffer::new()),
    });
    t.add_child(root, user);

    let pass = t.push(WidgetKind::TextField {
        placeholder: "password".into(),
        field: TextEditField::new(StringBuffer::new()),
    });
    t.add_child(root, pass);

    let row = t.push(WidgetKind::Row { gap: 8.0 });
    t.add_child(root, row);
    let connect = t.push(WidgetKind::Button {
        label: "Connect".into(),
        action: ACTION_CONNECT,
    });
    t.add_child(row, connect);

    let status = t.push(WidgetKind::Label {
        text: "status: not connected".into(),
        px_size: 12.0,
    });
    t.add_child(root, status);
    t
}

/// Browser screen: header + scrollable file tree + action row + path bar.
pub fn browser_tree(items: &[TreeItem]) -> WidgetTree {
    let mut t = WidgetTree::new();
    let root = t.push(WidgetKind::Column { gap: 6.0 });

    let header = t.push(WidgetKind::Label {
        text: "Files — /users/admin".into(),
        px_size: 14.0,
    });
    t.add_child(root, header);

    let scroll = t.push(WidgetKind::ScrollPane { scroll_y: 0.0 });
    t.add_child(root, scroll);
    let tree = t.push(WidgetKind::TreeList {
        items: items.to_vec(),
        selected: 0,
    });
    t.add_child(scroll, tree);

    let actions = t.push(WidgetKind::Row { gap: 8.0 });
    t.add_child(root, actions);
    for (label, action) in [
        ("Share", ACTION_SHARE),
        ("Upload", ACTION_UPLOAD),
        ("Delete", ACTION_DELETE),
    ] {
        let b = t.push(WidgetKind::Button {
            label: label.into(),
            action,
        });
        t.add_child(actions, b);
    }

    let pathbar = t.push(WidgetKind::Label {
        text: "path: /users/admin".into(),
        px_size: 11.0,
    });
    t.add_child(root, pathbar);
    t
}

/// Demo tree entries mirroring a real Ferro user root.
pub fn demo_entries() -> Vec<TreeItem> {
    vec![
        TreeItem { label: "dav".into(), depth: 0, has_children: true, expanded: true },
        TreeItem { label: "videos".into(), depth: 1, has_children: true, expanded: true },
        TreeItem { label: "4463.mp4 (17.3 MB)".into(), depth: 2, has_children: false, expanded: false },
        TreeItem { label: "demo.mp4".into(), depth: 2, has_children: false, expanded: false },
        TreeItem { label: "Documents".into(), depth: 0, has_children: true, expanded: false },
        TreeItem { label: "Pictures".into(), depth: 0, has_children: true, expanded: false },
        TreeItem { label: "4463.mp4 (17.3 MB)".into(), depth: 0, has_children: false, expanded: false },
    ]
}
