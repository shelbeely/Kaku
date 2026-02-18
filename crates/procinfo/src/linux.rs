#![cfg(target_os = "linux")]
use super::*;

impl From<char> for LocalProcessStatus {
    fn from(s: char) -> Self {
        match s {
            'R' => Self::Run,
            'S' => Self::Sleep,
            'D' => Self::LockBlocked,
            'Z' => Self::Zombie,
            'T' => Self::Stop,
            't' => Self::Tracing,
            'X' => Self::Dead,
            'x' => Self::Dead,
            'K' => Self::Wakekill,
            'W' => Self::Waking,
            'P' => Self::Parked,
            'I' => Self::Idle,
            _ => Self::Unknown,
        }
    }
}

impl LocalProcessInfo {
    pub fn current_working_dir(pid: u32) -> Option<PathBuf> {
        std::fs::read_link(format!("/proc/{}/cwd", pid)).ok()
    }

    pub fn executable_path(pid: u32) -> Option<PathBuf> {
        std::fs::read_link(format!("/proc/{}/exe", pid)).ok()
    }

    pub fn with_root_pid(pid: u32) -> Option<Self> {
        // Read /proc/[pid]/stat for basic process info
        let stat_path = format!("/proc/{}/stat", pid);
        let stat_content = std::fs::read_to_string(&stat_path).ok()?;
        
        // Parse the stat file
        // Format: pid (comm) state ppid ...
        let mut parts = stat_content.splitn(2, '(');
        parts.next()?;
        let rest = parts.next()?;
        let mut parts = rest.splitn(2, ')');
        let name = parts.next()?.to_string();
        let rest = parts.next()?;
        let fields: Vec<&str> = rest.split_whitespace().collect();
        
        // Need at least 20 fields to access index 19 (start_time at position 20 in /proc/pid/stat)
        if fields.len() < 20 {
            return None;
        }
        
        let status = fields.get(0)
            .and_then(|s| s.chars().next())
            .map(LocalProcessStatus::from)
            .unwrap_or(LocalProcessStatus::Unknown);
        
        let ppid = fields.get(1).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
        let start_time = fields.get(19).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
        
        let executable = Self::executable_path(pid).unwrap_or_default();
        let cwd = Self::current_working_dir(pid).unwrap_or_default();
        
        // Read command line arguments
        let cmdline_path = format!("/proc/{}/cmdline", pid);
        let argv = std::fs::read(&cmdline_path)
            .ok()
            .map(|bytes| {
                bytes
                    .split(|&b| b == 0)
                    .filter(|s| !s.is_empty())
                    .map(|s| String::from_utf8_lossy(s).into_owned())
                    .collect()
            })
            .unwrap_or_default();
        
        // Recursively get child processes
        let mut children = HashMap::new();
        if let Ok(entries) = std::fs::read_dir("/proc") {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if let Ok(child_pid) = file_name.parse::<u32>() {
                        if child_pid == pid {
                            continue;
                        }
                        // Check if this process is a child of our target
                        if let Some(child_info) = Self::with_root_pid(child_pid) {
                            if child_info.ppid == pid {
                                children.insert(child_pid, child_info);
                            }
                        }
                    }
                }
            }
        }
        
        Some(LocalProcessInfo {
            pid,
            ppid,
            name,
            executable,
            argv,
            cwd,
            status,
            start_time,
            children,
        })
    }
}
