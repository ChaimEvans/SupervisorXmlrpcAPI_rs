# SupervisorXmlrpcAPI_rs
Rust API client for Supervisor

## Usage
```rust
let Ok(res) = supervisor_xmlrpc::url().get_state();
```
## Methods
```rust
fn get_api_version(self) -> Result<String, ClientError> 
fn get_supervisor_version(self) -> Result<String, ClientError> 
fn get_identification(self) -> Result<String, ClientError> 
fn get_state(self) -> Result<res_struct::GetState, ClientError> 
fn get_pid(self) -> Result<String, ClientError> 
fn read_log(self, offset: i32, length: i32) -> Result<String, ClientError> 
fn clear_log(self) -> Result<bool, ClientError> 
fn shutdown(self) -> Result<bool, ClientError> 
fn restart(self) -> Result<bool, ClientError> 
fn get_process_info(self,name: &str) -> Result<res_struct::GetProcessInfo, ClientError> 
fn get_all_process_info(self) -> Result<Vec<res_struct::GetProcessInfo>, ClientError> 
fn get_all_config_info(self) -> Result<Vec<res_struct::ConfigInfo>, ClientError> 
fn start_process(self, name: &str, wait: bool) -> Result<bool, ClientError> 
fn start_all_processes(self, wait: bool) -> Result<Vec<bool>, ClientError> 
fn start_process_group(self,name: &str,wait: bool) -> Result<Vec<bool>, ClientError> 
fn stop_process(self, name: &str, wait: bool) -> Result<bool, ClientError> 
fn stop_all_processes(self, wait: bool) -> Result<Vec<bool>, ClientError> 
fn stop_process_group(self,name: &str,wait: bool) -> Result<Vec<bool>, ClientError> 
fn signal_process(self, name: &str, signal: &str) -> Result<bool, ClientError> 
fn signal_process_group(self,name: &str,signal: &str) -> Result<Vec<bool>, ClientError> 
fn signal_all_processes(self, signal: &str,) -> Result<Vec<bool>, ClientError> 
fn send_process_stdin(self, name: &str, chars: &str) -> Result<bool, ClientError> 
fn send_remote_comm_event(self,_type: &str,data: &str) -> Result<bool, ClientError> 
fn reload_config(self) -> Result<Vec<Vec<Vec<String>>>, ClientError> 
fn add_process_group(self, name: &str) -> Result<bool, ClientError> 
fn remove_process_group(self, name: &str) -> Result<bool, ClientError> 
fn read_process_log(self,name: &str,offset: i32,length: i32) -> Result<String, ClientError> 
fn read_process_stdout_log(self,name: &str,offset: i32,length: i32) -> Result<String, ClientError> 
fn read_process_stderr_log(self,name: &str,offset: i32,length: i32) -> Result<String, ClientError> 
fn tail_process_stdout_log(self,name: &str,offset: i32,length: i32) -> Result<res_struct::TailLog, ClientError> 
fn tail_process_stderr_log(self,name: &str,offset: i32,length: i32) -> Result<res_struct::TailLog, ClientError> 
fn clear_process_logs(self, name: &str) -> Result<bool, ClientError> 
fn clear_all_process_logs(self) -> Result<Vec<bool>, ClientError>
```