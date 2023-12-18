use dxr::TryFromValue;
use serde::Serialize;

#[derive(TryFromValue, Debug, Serialize)]
pub struct GetState {
    pub statecode: i32,
    pub statename: String,
}

#[derive(TryFromValue, Debug, Serialize)]
pub struct GetProcessInfo {
    pub name: String,           // 'process name',
    pub group: String,          // 'group name',
    pub description: String,    // 'pid 18806, uptime 0:03:12'
    pub start: i32,             // 1200361776,
    pub stop: i32,              // 0,
    pub now: i32,               // 1200361812,
    pub state: i32,             // 20,
    pub statename: String,      // 'RUNNING',
    pub spawnerr: String,       // '',
    pub exitstatus: i32,        // 0,
    pub logfile: String,        // '/path/to/stdout-log', # deprecated, b/c only
    pub stdout_logfile: String, // '/path/to/stdout-log',
    pub stderr_logfile: String, // '/path/to/stderr-log',
    pub pid: i32,               // 1           1
}

#[derive(TryFromValue, Debug, Serialize)]
pub struct TailLog {
    pub bytes: String,
    pub offset: i32,
    pub overflow: i32,
}

#[derive(TryFromValue, Debug, Serialize)]
pub struct ConfigInfo {
    pub autostart: bool,
    pub directory: String,
    pub uid: String,
    pub command: String,
    pub exitcodes: Vec<i32>,
    pub group: String,
    pub group_prio: i32,
    pub inuse: bool,
    pub killasgroup: bool,
    pub name: String,
    pub process_prio: i32,
    pub redirect_stderr: bool,
    pub startretries: i32,
    pub startsecs: i32,
    pub stdout_capture_maxbytes: i32,
    pub stdout_events_enabled: bool,
    pub stdout_logfile: String,
    pub stdout_logfile_backups: i32,
    pub stdout_logfile_maxbytes: i32,
    pub stdout_syslog: bool,
    pub stopsignal: i32,
    pub stopwaitsecs: i32,
    pub stderr_capture_maxbytes: i32,
    pub stderr_events_enabled: bool,
    pub stderr_logfile: String,
    pub stderr_logfile_backups: i32,
    pub stderr_logfile_maxbytes: i32,
    pub stderr_syslog: bool,
    pub serverurl: String,
}
