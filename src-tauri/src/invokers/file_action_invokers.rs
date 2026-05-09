use tauri_plugin_opener::OpenerExt;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Editor {
    id: String,
    name: String,
    cmd: String,
}

fn create_command(cmd: &str) -> tokio::process::Command {
    let mut command = tokio::process::Command::new(cmd);
    #[cfg(target_os = "windows")]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        command.creation_flags(CREATE_NO_WINDOW);
    }
    command
}

#[tauri::command]
pub async fn open_file(app: tauri::AppHandle, path: String) {
    let file_path = if let Some(last_colon) = path.rfind(':') {
        if last_colon > 1 {
            &path[..last_colon]
        } else {
            &path
        }
    } else {
        &path
    };

    let _ = app.opener().open_path(file_path, None::<&str>);
}

#[tauri::command]
pub async fn show_in_explorer(app: tauri::AppHandle, path: String) {
    let file_path = if let Some(last_colon) = path.rfind(':') {
        if last_colon > 1 {
            &path[..last_colon]
        } else {
            &path
        }
    } else {
        &path
    };

    let _ = app.opener().reveal_item_in_dir(file_path);
}

#[tauri::command]
pub async fn get_available_editors() -> Vec<Editor> {
    let mut editors = Vec::new();

    // JetBrains toolbox scripts support

    #[cfg(target_os = "windows")]
    {
        let editors_to_check = vec![
            ("vscode", "VS Code", vec!["code.cmd", "vscode:"]),
            ("sublime", "Sublime Text", vec!["subl.exe"]),
            ("zed", "Zed", vec!["zed.exe"]),
            ("intellij", "IntelliJ IDEA", vec!["idea64.exe", "idea.cmd"]),
            (
                "rustrover",
                "RustRover",
                vec!["rustrover64.exe", "rustrover.cmd"],
            ),
            (
                "webstorm",
                "WebStorm",
                vec!["webstorm64.exe", "webstorm.cmd"],
            ),
            (
                "phpstorm",
                "PhpStorm",
                vec!["phpstorm64.exe", "phpstorm.cmd"],
            ),
            ("pycharm", "PyCharm", vec!["pycharm64.exe", "pycharm.cmd"]),
            ("goland", "GoLand", vec!["goland64.exe", "goland.cmd"]),
            (
                "datagrip",
                "DataGrip",
                vec!["datagrip64.exe", "datagrip.cmd"],
            ),
            ("clion", "CLion", vec!["clion64.exe", "clion.cmd"]),
            ("rider", "Rider", vec!["rider64.exe", "rider.cmd"]),
        ];

        for (id, name, commands) in editors_to_check {
            for cmd in commands {
                let is_available = if cmd.ends_with(':') {
                    create_command("reg")
                        .arg("query")
                        .arg(format!("HKEY_CLASSES_ROOT\\{}", cmd.trim_end_matches(':')))
                        .output()
                        .await
                        .is_ok_and(|o| o.status.success())
                } else {
                    create_command("where")
                        .arg(cmd)
                        .output()
                        .await
                        .is_ok_and(|o| o.status.success())
                };

                if is_available {
                    editors.push(Editor {
                        id: id.to_string(),
                        name: name.to_string(),
                        cmd: cmd.to_string(),
                    });
                    break;
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // Common macOS editor locations / CLI tools
        let mac_editors = vec![
            ("vscode", "VS Code", vec!["code", "vscode:"]),
            ("cursor", "Cursor", vec!["cursor"]),
            ("subl", "Sublime Text", vec!["subl"]),
            ("zed", "Zed", vec!["zed"]),
            ("idea", "IntelliJ IDEA", vec!["idea"]),
            ("rustrover", "RustRover", vec!["rustrover"]),
            ("webstorm", "WebStorm", vec!["webstorm"]),
            ("phpstorm", "PhpStorm", vec!["phpstorm"]),
            ("pycharm", "PyCharm", vec!["pycharm"]),
            ("goland", "GoLand", vec!["goland"]),
            ("datagrip", "DataGrip", vec!["datagrip"]),
            ("clion", "CLion", vec!["clion"]),
            ("rider", "Rider", vec!["rider"]),
        ];

        for (id, name, commands) in mac_editors {
            for cmd in commands {
                let is_available = if cmd.ends_with(':') {
                    tokio::process::Command::new("open")
                        .arg("-Ra")
                        .arg("Visual Studio Code") // VS Code specific check
                        .output()
                        .await
                        .is_ok_and(|o| o.status.success())
                } else {
                    tokio::process::Command::new("which")
                        .arg(cmd)
                        .output()
                        .await
                        .is_ok_and(|o| o.status.success())
                };

                if is_available {
                    editors.push(Editor {
                        id: id.to_string(),
                        name: name.to_string(),
                        cmd: cmd.to_string(),
                    });
                    break;
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let linux_editors = vec![
            ("vscode", "VS Code", vec!["code", "vscode:"]),
            ("cursor", "Cursor", vec!["cursor"]),
            ("subl", "Sublime Text", vec!["subl"]),
            ("zed", "Zed", vec!["zed"]),
            ("idea", "IntelliJ IDEA", vec!["idea"]),
            ("rustrover", "RustRover", vec!["rustrover"]),
            ("webstorm", "WebStorm", vec!["webstorm"]),
            ("phpstorm", "PhpStorm", vec!["phpstorm"]),
            ("pycharm", "PyCharm", vec!["pycharm"]),
            ("goland", "GoLand", vec!["goland"]),
            ("datagrip", "DataGrip", vec!["datagrip"]),
            ("clion", "CLion", vec!["clion"]),
            ("rider", "Rider", vec!["rider"]),
        ];

        for (id, name, commands) in linux_editors {
            for cmd in commands {
                let is_available = if cmd.ends_with(':') {
                    tokio::process::Command::new("xdg-settings")
                        .arg("get")
                        .arg("default-url-scheme-handler")
                        .arg(cmd.trim_end_matches(':'))
                        .output()
                        .await
                        .is_ok_and(|o| o.status.success() && !o.stdout.is_empty())
                } else {
                    tokio::process::Command::new("which")
                        .arg(cmd)
                        .output()
                        .await
                        .is_ok_and(|o| o.status.success())
                };

                if is_available {
                    editors.push(Editor {
                        id: id.to_string(),
                        name: name.to_string(),
                        cmd: cmd.to_string(),
                    });
                    break;
                }
            }
        }
    }

    editors
}

#[tauri::command]
pub async fn open_in_editor(app: tauri::AppHandle, cmd: String, path: String) {
    if cmd.ends_with(':') {
        let protocol = cmd.trim_end_matches(':');
        if protocol == "vscode" {
            // vscode://file/{full path to file}:line:column
            let url = format!("vscode://file/{}", path.replace("\\", "/"));

            let _ = app.opener().open_url(url, None::<&str>);
        }
    } else {
        let mut command = create_command(cmd.as_str());

        let is_jetbrains = cmd.contains("idea")
            || cmd.contains("rustrover")
            || cmd.contains("webstorm")
            || cmd.contains("phpstorm")
            || cmd.contains("pycharm")
            || cmd.contains("goland")
            || cmd.contains("datagrip")
            || cmd.contains("clion")
            || cmd.contains("rider");

        if is_jetbrains {
            let mut parts = path.split(':');
            let base_path = parts.next().unwrap_or("");

            let (actual_path, remaining_parts) =
                if cfg!(target_os = "windows") && base_path.len() == 1 {
                    let drive = base_path;
                    let rest = parts.next().unwrap_or("");
                    let full_path = format!("{}:{}", drive, rest);
                    (full_path, parts.collect::<Vec<_>>())
                } else {
                    (base_path.to_string(), parts.collect::<Vec<_>>())
                };

            if let Some(line) = remaining_parts.get(0) {
                command.arg("--line").arg(line);
                if let Some(column) = remaining_parts.get(1) {
                    command.arg("--column").arg(column);
                }
            }

            #[cfg(target_os = "windows")]
            {
                command.arg(actual_path.replace("/", "\\"));
            }

            #[cfg(not(target_os = "windows"))]
            {
                command.arg(actual_path);
            }
        } else {
            command.arg("--goto");
            #[cfg(target_os = "windows")]
            {
                command.arg(path.replace("/", "\\"));
            }

            #[cfg(not(target_os = "windows"))]
            {
                command.arg(path);
            }
        }

        let _ = command.spawn();
    }
}
