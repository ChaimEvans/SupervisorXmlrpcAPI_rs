use dxr_client::{Call, ClientError};

use crate::res_struct;

pub struct SupervisorXmlrpcBuilder {
    pub supper: dxr_client::ClientBuilder,
}

impl SupervisorXmlrpcBuilder {
    pub fn new(url: dxr_client::Url) -> Self {
        SupervisorXmlrpcBuilder {
            supper: dxr_client::ClientBuilder::new(url),
        }
    }

    pub fn user_agent(mut self, user_agent: &'static str) -> Self {
        self.supper = self.supper.user_agent(user_agent);
        self
    }

    pub fn add_header(
        mut self,
        name: http::header::HeaderName,
        value: http::header::HeaderValue,
    ) -> Self {
        self.supper = self.supper.add_header(name, value);
        self
    }

    pub fn build(self) -> SupervisorXmlrpc {
        SupervisorXmlrpc {
            supper: self.supper.build(),
        }
    }
}

pub struct SupervisorXmlrpc {
    pub supper: dxr_client::Client,
}

impl SupervisorXmlrpc {
    pub async fn get_api_version(self) -> Result<String, ClientError> {
        let call = Call::new("supervisor.getAPIVersion", ());
        self.supper.call(call).await
    }

    pub async fn get_supervisor_version(self) -> Result<String, ClientError> {
        let call = Call::new("supervisor.getSupervisorVersion", ());
        self.supper.call(call).await
    }

    pub async fn get_identification(self) -> Result<String, ClientError> {
        let call = Call::new("supervisor.getIdentification", ());
        self.supper.call(call).await
    }

    pub async fn get_state(self) -> Result<res_struct::GetState, ClientError> {
        let call = Call::new("supervisor.getState", ());
        self.supper.call(call).await
    }

    pub async fn get_pid(self) -> Result<String, ClientError> {
        let call = Call::new("supervisor.getPID", ());
        self.supper.call(call).await
    }

    pub async fn read_log(self, offset: i32, length: i32) -> Result<String, ClientError> {
        let call = Call::new("supervisor.readLog", (offset, length));
        self.supper.call(call).await
    }

    pub async fn clear_log(self) -> Result<bool, ClientError> {
        let call = Call::new("supervisor.clearLog", ());
        self.supper.call(call).await
    }

    pub async fn shutdown(self) -> Result<bool, ClientError> {
        let call = Call::new("supervisor.shutdown", ());
        self.supper.call(call).await
    }

    pub async fn restart(self) -> Result<bool, ClientError> {
        let call = Call::new("supervisor.restart", ());
        self.supper.call(call).await
    }

    pub async fn get_process_info(
        self,
        name: &str,
    ) -> Result<res_struct::GetProcessInfo, ClientError> {
        let call = Call::new("supervisor.getProcessInfo", name);
        self.supper.call(call).await
    }

    pub async fn get_all_process_info(
        self,
    ) -> Result<Vec<res_struct::GetProcessInfo>, ClientError> {
        let call = Call::new("supervisor.getAllProcessInfo", ());
        self.supper.call(call).await
    }

    pub async fn get_all_config_info(self) -> Result<Vec<res_struct::ConfigInfo>, ClientError> {
        let call = Call::new("supervisor.getAllConfigInfo", ());
        self.supper.call(call).await
    }

    pub async fn start_process(self, name: &str, wait: bool) -> Result<bool, ClientError> {
        let call = Call::new("supervisor.startProcess", (name, wait));
        self.supper.call(call).await
    }

    pub async fn start_all_processes(self, wait: bool) -> Result<Vec<bool>, ClientError> {
        let call = Call::new("supervisor.startAllProcesses", wait);
        self.supper.call(call).await
    }

    pub async fn start_process_group(
        self,
        name: &str,
        wait: bool,
    ) -> Result<Vec<bool>, ClientError> {
        let call = Call::new("supervisor.startProcessGroup", (name, wait));
        self.supper.call(call).await
    }

    pub async fn stop_process(self, name: &str, wait: bool) -> Result<bool, ClientError> {
        let call = Call::new("supervisor.stopProcess", (name, wait));
        self.supper.call(call).await
    }

    pub async fn stop_all_processes(self, wait: bool) -> Result<Vec<bool>, ClientError> {
        let call = Call::new("supervisor.stopAllProcesses", wait);
        self.supper.call(call).await
    }

    pub async fn stop_process_group(
        self,
        name: &str,
        wait: bool,
    ) -> Result<Vec<bool>, ClientError> {
        let call = Call::new("supervisor.stopProcessGroup", (name, wait));
        self.supper.call(call).await
    }

    pub async fn signal_process(self, name: &str, signal: &str) -> Result<bool, ClientError> {
        let call = Call::new("supervisor.signalProcess", (name, signal));
        self.supper.call(call).await
    }

    pub async fn signal_process_group(
        self,
        name: &str,
        signal: &str,
    ) -> Result<Vec<bool>, ClientError> {
        let call = Call::new("supervisor.signalProcessGroup", (name, signal));
        self.supper.call(call).await
    }

    pub async fn signal_all_processes(self, signal: &str,) -> Result<Vec<bool>, ClientError> {
        let call = Call::new("supervisor.signalAllProcesses", signal);
        self.supper.call(call).await
    }

    pub async fn send_process_stdin(self, name: &str, chars: &str) -> Result<bool, ClientError> {
        let call = Call::new("supervisor.sendProcessStdin", (name, chars));
        self.supper.call(call).await
    }

    pub async fn send_remote_comm_event(
        self,
        _type: &str,
        data: &str,
    ) -> Result<bool, ClientError> {
        let call = Call::new("supervisor.sendRemoteCommEvent", (_type, data));
        self.supper.call(call).await
    }

    pub async fn reload_config(self) -> Result<Vec<Vec<Vec<String>>>, ClientError> {
        let call = Call::new("supervisor.reloadConfig", ());
        self.supper.call(call).await
    }

    pub async fn add_process_group(self, name: &str) -> Result<bool, ClientError> {
        let call = Call::new("supervisor.addProcessGroup", name);
        self.supper.call(call).await
    }

    pub async fn remove_process_group(self, name: &str) -> Result<bool, ClientError> {
        let call = Call::new("supervisor.removeProcessGroup", name);
        self.supper.call(call).await
    }

    pub async fn read_process_log(
        self,
        name: &str,
        offset: i32,
        length: i32,
    ) -> Result<String, ClientError> {
        let call = Call::new("supervisor.readProcessLog", (name, offset, length));
        self.supper.call(call).await
    }

    pub async fn read_process_stdout_log(
        self,
        name: &str,
        offset: i32,
        length: i32,
    ) -> Result<String, ClientError> {
        let call = Call::new("supervisor.readProcessStdoutLog", (name, offset, length));
        self.supper.call(call).await
    }

    pub async fn read_process_stderr_log(
        self,
        name: &str,
        offset: i32,
        length: i32,
    ) -> Result<String, ClientError> {
        let call = Call::new("supervisor.readProcessStderrLog", (name, offset, length));
        self.supper.call(call).await
    }

    pub async fn tail_process_stdout_log(
        self,
        name: &str,
        offset: i32,
        length: i32,
    ) -> Result<res_struct::TailLog, ClientError> {
        let call = Call::new("supervisor.tailProcessStdoutLog", (name, offset, length));
        self.supper.call(call).await
    }

    pub async fn tail_process_stderr_log(
        self,
        name: &str,
        offset: i32,
        length: i32,
    ) -> Result<res_struct::TailLog, ClientError> {
        let call = Call::new("supervisor.tailProcessStderrLog", (name, offset, length));
        self.supper.call(call).await
    }

    pub async fn clear_process_logs(self, name: &str) -> Result<bool, ClientError> {
        let call = Call::new("supervisor.clearProcessLogs", name);
        self.supper.call(call).await
    }

    pub async fn clear_all_process_logs(self) -> Result<Vec<bool>, ClientError> {
        let call = Call::new("supervisor.clearAllProcessLogs", ());
        self.supper.call(call).await
    }
}
