#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Editor {
    id: String,
    name: String,
    cmd: String,
}

#[tauri::command]
pub async fn open_file(path: String) {
    let file_path = if let Some(last_colon) = path.rfind(':') {
        if last_colon > 1 {
            &path[..last_colon]
        } else {
            &path
        }
    } else {
        &path
    };

    #[cfg(target_os = "windows")]
    {
        let _ = tokio::process::Command::new("cmd")
            .arg("/c")
            .arg("start")
            .arg("")
            .arg(file_path.replace("/", "\\"))
            .spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = tokio::process::Command::new("open").arg(file_path).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = tokio::process::Command::new("xdg-open")
            .arg(file_path)
            .spawn();
    }
}

#[tauri::command]
pub async fn show_in_explorer(path: String) {
    let file_path = if let Some(last_colon) = path.rfind(':') {
        if last_colon > 1 {
            &path[..last_colon]
        } else {
            &path
        }
    } else {
        &path
    };

    #[cfg(target_os = "windows")]
    {
        let _ = tokio::process::Command::new("explorer")
            .arg("/select,")
            .arg(file_path.replace("/", "\\"))
            .spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = tokio::process::Command::new("open")
            .arg("-R")
            .arg(file_path)
            .spawn();
    }
    #[cfg(target_os = "linux")]
    {
        if let Some(parent) = std::path::Path::new(file_path).parent() {
            let _ = tokio::process::Command::new("xdg-open").arg(parent).spawn();
        }
    }
}

#[tauri::command]
pub async fn get_available_editors() -> Vec<Editor> {
    let mut editors = Vec::new();

    // @TODO Jetbrain toolbox scripts support

    #[cfg(target_os = "windows")]
    {
        // VS Code
        if tokio::process::Command::new("where")
            .arg("code.cmd")
            .output()
            .await
            .is_ok_and(|o| o.status.success())
        {
            editors.push(Editor {
                id: "vscode".to_string(),
                name: "VS Code".to_string(),
                cmd: "code.cmd".to_string(),
            });
        }

        // Sublime Text
        if tokio::process::Command::new("where")
            .arg("subl.exe")
            .output()
            .await
            .is_ok_and(|o| o.status.success())
        {
            editors.push(Editor {
                id: "sublime".to_string(),
                name: "Sublime Text".to_string(),
                cmd: "subl.exe".to_string(),
            });
        }

        // Zed
        if tokio::process::Command::new("where")
            .arg("zed.exe")
            .output()
            .await
            .is_ok_and(|o| o.status.success())
        {
            editors.push(Editor {
                id: "zed".to_string(),
                name: "Zed".to_string(),
                cmd: "zed.exe".to_string(),
            });
        }

        // IntelliJ IDEA
        if tokio::process::Command::new("where")
            .arg("idea64.exe")
            .output()
            .await
            .is_ok_and(|o| o.status.success())
        {
            editors.push(Editor {
                id: "intellij".to_string(),
                name: "IntelliJ IDEA".to_string(),
                cmd: "idea64.exe".to_string(),
            });
        }

        // RustRover
        if tokio::process::Command::new("where")
            .arg("rustrover64.exe")
            .output()
            .await
            .is_ok_and(|o| o.status.success())
        {
            editors.push(Editor {
                id: "rustrover".to_string(),
                name: "RustRover".to_string(),
                cmd: "rustrover64.exe".to_string(),
            });
        }

        // WebStorm
        if tokio::process::Command::new("where")
            .arg("webstorm64.exe")
            .output()
            .await
            .is_ok_and(|o| o.status.success())
        {
            editors.push(Editor {
                id: "webstorm".to_string(),
                name: "WebStorm".to_string(),
                cmd: "webstorm64.exe".to_string(),
            });
        }

        // PhpStorm
        if tokio::process::Command::new("where")
            .arg("phpstorm64.exe")
            .output()
            .await
            .is_ok_and(|o| o.status.success())
        {
            editors.push(Editor {
                id: "phpstorm".to_string(),
                name: "PhpStorm".to_string(),
                cmd: "phpstorm64.exe".to_string(),
            });
        }

        // PyCharm
        if tokio::process::Command::new("where")
            .arg("pycharm64.exe")
            .output()
            .await
            .is_ok_and(|o| o.status.success())
        {
            editors.push(Editor {
                id: "pycharm".to_string(),
                name: "PyCharm".to_string(),
                cmd: "pycharm64.exe".to_string(),
            });
        }

        // GoLand
        if tokio::process::Command::new("where")
            .arg("goland64.exe")
            .output()
            .await
            .is_ok_and(|o| o.status.success())
        {
            editors.push(Editor {
                id: "goland".to_string(),
                name: "GoLand".to_string(),
                cmd: "goland64.exe".to_string(),
            });
        }
    }

    #[cfg(target_os = "macos")]
    {
        // Common macOS editor locations / CLI tools
        let mac_editors = vec![
            ("code", "VS Code", "vscode"),
            ("cursor", "Cursor", "cursor"),
            ("subl", "Sublime Text", "sublime"),
            ("zed", "Zed", "zed"),
            ("idea", "IntelliJ IDEA", "intellij"),
            ("rustrover", "RustRover", "rustrover"),
            ("webstorm", "WebStorm", "webstorm"),
            ("phpstorm", "PhpStorm", "phpstorm"),
            ("pycharm", "PyCharm", "pycharm"),
            ("goland", "GoLand", "goland"),
        ];

        for (cmd, name, id) in mac_editors {
            if tokio::process::Command::new("which")
                .arg(cmd)
                .output()
                .await
                .is_ok_and(|o| o.status.success())
            {
                editors.push(Editor {
                    id: id.to_string(),
                    name: name.to_string(),
                    cmd: cmd.to_string(),
                });
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let linux_editors = vec![
            ("code", "VS Code", "vscode"),
            ("cursor", "Cursor", "cursor"),
            ("subl", "Sublime Text", "sublime"),
            ("zed", "Zed", "zed"),
            ("idea", "IntelliJ IDEA", "intellij"),
            ("rustrover", "RustRover", "rustrover"),
            ("webstorm", "WebStorm", "webstorm"),
            ("phpstorm", "PhpStorm", "phpstorm"),
            ("pycharm", "PyCharm", "pycharm"),
            ("goland", "GoLand", "goland"),
        ];

        for (cmd, name, id) in linux_editors {
            if tokio::process::Command::new("which")
                .arg(cmd)
                .output()
                .await
                .is_ok_and(|o| o.status.success())
            {
                editors.push(Editor {
                    id: id.to_string(),
                    name: name.to_string(),
                    cmd: cmd.to_string(),
                });
            }
        }
    }

    editors
}

#[tauri::command]
pub async fn open_in_editor(cmd: String, path: String) {
    let _ = tokio::process::Command::new(cmd)
        .arg("--goto")
        .arg(path.replace("/", "\\"))
        .spawn();
}
