//! The menu bar and the application-wide shortcuts.
//!
//! GPUI adds neither on its own. On macOS an app without them has an empty
//! menu bar and no ⌘Q, so this is where disktree becomes a Mac app. GPUI
//! draws no menu bar on Linux or Windows, so there the same actions get the
//! shortcuts their users expect, with ctrl, and nothing else.

use gpui_kit::{App, KeyBinding, Menu, MenuItem, SystemMenuType};

pub use menu_actions::*;

#[allow(
    clippy::derive_partial_eq_without_eq,
    reason = "GPUI's actions! macro writes the derives, not us"
)]
mod menu_actions {
    use gpui_kit::actions;

    actions!(
        disktree,
        [
            /// 退出程序。
            Quit,
            /// 隐藏程序窗口。
            Hide,
            /// 隐藏其他应用。
            HideOthers,
            /// 重新显示所有应用。
            ShowAll,
            /// 关闭当前窗口（会退出程序：只有一个窗口）。
            CloseWindow,
            /// 重新扫描当前根目录。
            Rescan,
            /// 选择要扫描的目录。
            OpenFolder,
            /// 在文件管理器中显示按键所作用的方块。
            ShowInFinder,
            /// 返回访问过的上一个目录。
            GoBack,
            /// 前进（撤销后退）。
            GoForward,
        ]
    );
}

/// Install the menus, the shortcuts and the quit-on-close rule.
pub fn install(cx: &mut App) {
    // A single-window utility has nothing left to show once its window is
    // gone; staying alive would leave a Dock icon with no window behind it.
    cx.on_window_closed(|cx, _| {
        if cx.windows().is_empty() {
            cx.quit();
        }
    })
    .detach();

    cx.on_action(|_: &Quit, cx| cx.quit());
    cx.on_action(|_: &CloseWindow, cx| {
        if let Some(window) = cx.active_window() {
            let _ = window.update(cx, |_, window, _| window.remove_window());
        }
    });
    // `Rescan`, `OpenFolder`, `ShowInFinder` and the history are handled by
    // the window, which owns the scan and the selection; see `views::root`.
    if !cfg!(target_os = "macos") {
        cx.bind_keys([
            KeyBinding::new("ctrl-q", Quit, None),
            KeyBinding::new("ctrl-w", CloseWindow, None),
            KeyBinding::new("ctrl-o", OpenFolder, None),
            KeyBinding::new("ctrl-r", Rescan, None),
            KeyBinding::new("f5", Rescan, None),
            KeyBinding::new("ctrl-shift-r", ShowInFinder, None),
        ]);
        return;
    }
    cx.on_action(|_: &Hide, cx| cx.hide());
    cx.on_action(|_: &HideOthers, cx| cx.hide_other_apps());
    cx.on_action(|_: &ShowAll, cx| cx.unhide_other_apps());
    cx.bind_keys([
        KeyBinding::new("cmd-q", Quit, None),
        KeyBinding::new("cmd-h", Hide, None),
        KeyBinding::new("alt-cmd-h", HideOthers, None),
        KeyBinding::new("cmd-w", CloseWindow, None),
        KeyBinding::new("cmd-r", Rescan, None),
        KeyBinding::new("cmd-o", OpenFolder, None),
        // ⌘R is taken by Rescan, as in a browser; Xcode uses ⌘⇧R-like
        // chords for its reveals, and `o` does the same without a modifier.
        KeyBinding::new("cmd-shift-r", ShowInFinder, None),
        // Finder's and Safari's back and forward; alt-arrows work too.
        KeyBinding::new("cmd-[", GoBack, None),
        KeyBinding::new("cmd-]", GoForward, None),
    ]);
    cx.set_menus([
        Menu::new("disktree").items([
            MenuItem::os_submenu("Services", SystemMenuType::Services),
            MenuItem::separator(),
            MenuItem::action("隐藏 disktree", Hide),
            MenuItem::action("隐藏其他", HideOthers),
            MenuItem::action("全部显示", ShowAll),
            MenuItem::separator(),
            MenuItem::action("退出 disktree", Quit),
        ]),
        Menu::new("文件").items([
            MenuItem::action("打开目录\u{2026}", OpenFolder),
            MenuItem::action("在访达中显示", ShowInFinder),
            MenuItem::separator(),
            MenuItem::action("重新扫描", Rescan),
            MenuItem::separator(),
            MenuItem::action("关闭窗口", CloseWindow),
        ]),
        Menu::new("前往").items([
            MenuItem::action("后退", GoBack),
            MenuItem::action("前进", GoForward),
        ]),
    ]);
}
