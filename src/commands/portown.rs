use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use std::process::Command;
use std::sync::Mutex;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::error::Error;

use crate::cli::PortownArgs;

pub fn execute(args: &PortownArgs) -> crate::error::Result<()> {
    use std::time::Instant;
    
    // Record start time for performance measurement
    let start_time = Instant::now();
    
    // 获取TCP/UDP连接信息
    let output = Command::new("netstat")
        .args(["-ano"])
        .output()
        .map_err(|e| Error::Other(format!("Failed to execute netstat: {}", e)))?;
    
    if !output.status.success() {
        return Err(Error::Other("netstat command failed".to_string()));
    }
    
    let netstat_output = String::from_utf8_lossy(&output.stdout);
    
    // 收集所有进程信息，避免重复查询
    let mut pid_cache: HashMap<String, (String, String)> = HashMap::new();
    let mut connections = Vec::new();

    // 解析netstat输出
    // 在连接处理循环中添加深度过滤
    let mut current_depth = 0;
    
    for line in netstat_output.lines() {
        // 应用深度过滤
        if let Some(max_depth) = args.depth {
            if current_depth >= max_depth {
                break;
            }
        }
        current_depth += 1;
    
        // 原有过滤逻辑保持不变
        if args.udp_only && !line.contains("UDP") {
            continue;
        }
        if args.tcp_only && !line.contains("TCP") {
            continue;
        }
        if !(line.contains("TCP") || line.contains("UDP")) {
            continue;
        }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        // Windows netstat output has different column counts for TCP vs UDP
        let min_columns = if line.contains("TCP") { 5 } else { 4 };
        if parts.len() < min_columns {
            continue;
        }
        
        let protocol = if line.contains("TCP") {
            "TCP".to_string()
        } else {
            "UDP".to_string()
        };
        let local_address = parts[1].to_string();
        let foreign_address = parts[2].to_string();
        // UDP connections don't have state in Windows netstat
        let state = if line.contains("TCP") {
            parts[3].to_string()
        } else {
            "-".to_string()
        };
        
        // 根据参数过滤连接状态
        if args.listen && state != "LISTENING" {
            continue;
        }
        if args.established_only && state != "ESTABLISHED" {
            continue;
        }
        
        // PID is in different columns for TCP vs UDP
        let pid = if line.contains("TCP") {
            parts[4].to_string()
        } else {
            parts[3].to_string()
        };
        
        connections.push((protocol, local_address, foreign_address, state, pid));
    }

    // 获取所有进程信息（去重后）
    let unique_pids: HashSet<_> = connections.iter().map(|(_, _, _, _, pid)| pid).collect();
    for pid in unique_pids {
        if !pid_cache.contains_key(pid) {
            match get_process_info(pid) {
                Ok(info) => {
                    pid_cache.insert(pid.to_string(), info);
                },
                Err(_) => {
                    pid_cache.insert(pid.to_string(), ("Unknown".to_string(), "Unknown".to_string()));
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

    // Log command execution time
    crate::utils::log_command_metrics(
        "portown", 
        start_time.elapsed().as_millis(), 
        "success", 
        None
    );
    
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

lazy_static::lazy_static! {
    static ref PROCESS_CACHE: Mutex<HashMap<String, (String, String)>> = Mutex::new(HashMap::new());
}

fn get_process_info(pid: &str) -> crate::error::Result<(String, String)> {
    use std::time::Instant;
use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::shared::ntdef::HANDLE;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION;
use winapi::um::winbase::QueryFullProcessImageNameA;
use winapi::um::psapi::GetModuleFileNameExA;
use winapi::um::handleapi::CloseHandle;


    let start_time = Instant::now();
    
    // 检查缓存
    {
        let cache = PROCESS_CACHE.lock().unwrap();
        if let Some(info) = cache.get(pid) {
            return Ok(info.clone());
        }
    }
    
    // 使用Windows API获取进程信息
    let pid_num: DWORD = pid.parse().unwrap_or(0);
    let process_handle: HANDLE;
    
    // 尝试获取SeDebugPrivilege特权
    unsafe {
        let mut token: winapi::um::winnt::HANDLE = std::ptr::null_mut();
        use winapi::um::processthreadsapi::OpenProcessToken;
        use winapi::um::winbase::LookupPrivilegeValueA;
        
        if OpenProcessToken(
            winapi::um::processthreadsapi::GetCurrentProcess(),
            winapi::um::winnt::TOKEN_ADJUST_PRIVILEGES | winapi::um::winnt::TOKEN_QUERY,
            &mut token
        ) != 0 {
            let mut luid = winapi::um::winnt::LUID { LowPart: 0, HighPart: 0 };
            if LookupPrivilegeValueA(
                std::ptr::null(),
                winapi::um::winnt::SE_DEBUG_NAME.as_ptr() as *const i8,
                &mut luid
            ) != 0 {
                let mut tp = winapi::um::winnt::TOKEN_PRIVILEGES {
                    PrivilegeCount: 1,
                    Privileges: [winapi::um::winnt::LUID_AND_ATTRIBUTES {
                        Luid: luid,
                        Attributes: winapi::um::winnt::SE_PRIVILEGE_ENABLED
                    }]
                };
                winapi::um::securitybaseapi::AdjustTokenPrivileges(
                    token,
                    FALSE,
                    &mut tp,
                    std::mem::size_of::<winapi::um::winnt::TOKEN_PRIVILEGES>() as u32,
                    std::ptr::null_mut(),
                    std::ptr::null_mut()
                );
            }
            winapi::um::handleapi::CloseHandle(token);
        }
        
        process_handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid_num);
        if process_handle.is_null() {
            let _last_error = winapi::um::errhandlingapi::GetLastError();
            /*
            eprintln!(
                "Failed to open process with PID: {} (Error: {})", 
                pid, 
                last_error
            );
            */
            // 即使失败也更新缓存，避免重复尝试
            let mut cache = PROCESS_CACHE.lock().unwrap();
            cache.insert(
                pid.to_string(), 
                ("Unknown".to_string(), "Unknown".to_string())
            );
            return Ok(("Unknown".to_string(), "Unknown".to_string()));
        }
    }
    
    // 获取进程名
    let mut name_buffer = [0u8; 260];
    let mut name = "Unknown".to_string();
    
    unsafe {
        let mut size = name_buffer.len() as DWORD;
        if QueryFullProcessImageNameA(process_handle, 0, name_buffer.as_mut_ptr() as *mut i8, &mut size) != 0 {
            name = String::from_utf8_lossy(
                &name_buffer[..size as usize]
            ).to_string();
            if let Some(last_slash) = name.rfind('\\') {
                name = name[last_slash + 1..].to_string();
            }
            //eprintln!("Successfully got process name for PID {}: {}", pid, name);
        } else {
            //eprintln!("Failed to get process name for PID: {}", pid);
        }
    }
    
    // 获取进程路径
    let mut path = "Unknown".to_string();
    unsafe {
        let mut path_buffer = [0u8; 260];
        if GetModuleFileNameExA(process_handle, std::ptr::null_mut(), path_buffer.as_mut_ptr() as *mut i8, path_buffer.len() as DWORD) != 0 {
            path = String::from_utf8_lossy(&path_buffer).to_string();
            //eprintln!("Successfully got process path for PID {}: {}", pid, path);
        } else {
            let _last_error = winapi::um::errhandlingapi::GetLastError();
            //eprintln!("Failed to get process path for PID: {} (Error: {})", pid, last_error);
        }
        CloseHandle(process_handle);
    }

    // 更新缓存
    {
        let mut cache = PROCESS_CACHE.lock().unwrap();
        cache.insert(pid.to_string(), (name.clone(), path.clone()));
    }

    // 记录执行时间
    crate::utils::log_command_metrics(
        &format!("Get-Process {}", pid), 
        start_time.elapsed().as_millis(), 
        "success", 
        None
    );
    
    Ok((name, path))
}