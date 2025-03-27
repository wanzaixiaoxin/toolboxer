use std::process::Command;
use std::collections::HashMap;
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::error::Error;

pub fn execute() -> crate::error::Result<()> {
    // 获取所有端口信息
    let output = Command::new("netstat")
        .args(&["-ano"])
        .output()
        .map_err(|e| Error::Other(format!("Command execution error: {}", e)))?;

    if !output.status.success() {
        return Err(Error::Other(
            format!("Command execution failed: {}", String::from_utf8_lossy(&output.stderr))
        ));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.lines().collect();

    // 收集所有进程信息，避免重复查询
    let mut pid_cache: HashMap<String, (String, String)> = HashMap::new();
    let mut connections = Vec::new();

    // 解析连接信息
    for line in lines.iter().skip(4) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            let protocol = parts[0].to_string();
            let local_address = parts[1].to_string();
            let foreign_address = parts[2].to_string();
            let state = if protocol == "UDP" || parts.len() < 4 {
                "N/A".to_string()
            } else {
                parts[3].to_string()
            };
            let pid = if protocol == "UDP" {
                parts[3].to_string()
            } else {
                parts[parts.len() - 1].to_string()
            };

            connections.push((protocol, local_address, foreign_address, state, pid));
        }
    }

    // 获取所有进程信息
    for (_, _, _, _, pid) in &connections {
        if !pid_cache.contains_key(pid) {
            match get_process_info(pid) {
                Ok(info) => {
                    pid_cache.insert(pid.clone(), info);
                },
                Err(_) => {
                    pid_cache.insert(pid.clone(), ("Unknown".to_string(), "Unknown".to_string()));
                }
            }
        }
    }

    // 打印表头
    print_header()?;

    // 打印连接信息
    for (idx, (protocol, local_address, foreign_address, state, pid)) in connections.iter().enumerate() {
        let default = ("Unknown".to_string(), "Unknown".to_string());
        let (name, path) = pid_cache.get(pid).unwrap_or(&default);
        
        // 交替行颜色
        let bg_color = if idx % 2 == 0 { None } else { Some(Color::Ansi256(236)) };
        
        print_connection(
            protocol, 
            local_address, 
            foreign_address, 
            state, 
            pid, 
            name, 
            path, 
            bg_color
        )?;
    }

    Ok(())
}

fn print_header() -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    
    // 设置表头颜色
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)).set_bold(true))?;
    
    // 打印表头
    writeln!(
        &mut stdout, 
        "\n{:<10} {:<25} {:<25} {:<15} {:<8} {:<20} {}", 
        "PROTOCOL", "LOCAL ADDRESS", "FOREIGN ADDRESS", "STATE", "PID", "PROCESS", "PATH"
    )?;
    
    // 重置颜色
    stdout.reset()?;
    
    // 打印分隔线
    writeln!(&mut stdout, "{}", "─".repeat(120))?;
    
    Ok(())
}

fn print_connection(
    protocol: &str,
    local_addr: &str,
    foreign_addr: &str,
    state: &str,
    pid: &str,
    proc_name: &str,
    proc_path: &str,
    bg_color: Option<Color>
) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    
    // 设置背景色（如果有）
    if let Some(color) = bg_color {
        stdout.set_color(ColorSpec::new().set_bg(Some(color)))?;
    }
    
    // 协议颜色
    stdout.set_color(ColorSpec::new()
        .set_fg(Some(match protocol {
            "TCP" => Color::Green,
            "UDP" => Color::Yellow,
            _ => Color::White
        }))
        .set_bold(true)
        .set_bg(bg_color))?;
    write!(&mut stdout, "{:<10} ", protocol)?;
    
    // 本地地址
    stdout.set_color(ColorSpec::new()
        .set_fg(Some(Color::Cyan))
        .set_bg(bg_color))?;
    write!(&mut stdout, "{:<25} ", local_addr)?;
    
    // 远程地址
    stdout.set_color(ColorSpec::new()
        .set_fg(Some(Color::Blue))
        .set_bg(bg_color))?;
    write!(&mut stdout, "{:<25} ", foreign_addr)?;
    
    // 状态
    let state_color = match state {
        "LISTENING" => Color::Yellow,
        "ESTABLISHED" => Color::Green,
        "CLOSE_WAIT" => Color::Red,
        "TIME_WAIT" => Color::Magenta,
        _ => Color::White
    };
    stdout.set_color(ColorSpec::new().set_fg(Some(state_color)).set_bg(bg_color))?;
    write!(&mut stdout, "{:<15} ", state)?;
    
    // PID
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)).set_bg(bg_color))?;
    write!(&mut stdout, "{:<8} ", pid)?;
    
    // 进程名
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bg(bg_color))?;
    write!(&mut stdout, "{:<20} ", proc_name)?;
    
    // 进程路径
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)).set_bg(bg_color))?;
    writeln!(&mut stdout, "{}", proc_path)?;
    
    // 重置颜色
    stdout.reset()?;
    
    Ok(())
}

fn get_process_info(pid: &str) -> crate::error::Result<(String, String)> {
    let output = Command::new("wmic")
        .args(&["process", "where", &format!("processid={}", pid), "get", "name,executablepath"])
        .output()
        .map_err(|e| Error::Other(format!("Command execution error: {}", e)))?;

    if !output.status.success() {
        return Err(Error::Other(
            format!("Command execution failed: {}", String::from_utf8_lossy(&output.stderr))
        ));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    if lines.len() < 2 {
        return Ok((String::from("Unknown"), String::from("Unknown")));
    }

    // 处理 WMIC 输出，提取进程名和路径
    let header_line = lines[0];
    let data_line = lines[1];
    
    // 找出 Name 和 ExecutablePath 在输出中的位置
    let name_pos = header_line.to_lowercase().find("name").unwrap_or(0);
    let path_pos = header_line.to_lowercase().find("executablepath").unwrap_or(header_line.len());
    
    // 提取进程名和路径
    let name = if name_pos < data_line.len() {
        let end = if path_pos < data_line.len() { path_pos } else { data_line.len() };
        data_line[name_pos..end].trim().to_string()
    } else {
        "Unknown".to_string()
    };
    
    let path = if path_pos < data_line.len() {
        data_line[path_pos..].trim().to_string()
    } else {
        "Unknown".to_string()
    };

    Ok((name, path))
}