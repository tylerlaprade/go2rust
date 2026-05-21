use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct exec_Cmd {
    pub args: Arc<Mutex<Option<Vec<String>>>>,
}

impl std::fmt::Display for exec_Cmd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<exec_Cmd>")
    }
}

impl exec_Cmd {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    fn __go_error(message: String) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        Arc::new(Mutex::new(Some::<Box<dyn StdError + Send + Sync>>(Box::<dyn StdError + Send + Sync>::from(message))))
    }

    fn __go_run_output(&self) -> Result<std::process::Output, std::io::Error> {
        let args = self.args.lock().unwrap().as_ref().cloned().unwrap_or_default();
        if args.is_empty() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "empty command"));
        }
        let mut command = std::process::Command::new(&args[0]);
        command.args(&args[1..]);
        command.output()
    }

    fn __go_write_output(&self, output: &std::process::Output) {
    }

    pub fn output(&self) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        match self.__go_run_output() {
            Ok(output) => {
                let err = if output.status.success() {
                    Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
                } else {
                    Self::__go_error(format!("exit status {}", output.status))
                };
                (Arc::new(Mutex::new(Some::<Vec<u8>>(output.stdout))), err)
            }
            Err(err) => (Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::new()))), Self::__go_error(err.to_string())),
        }
    }

    pub fn run(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        self.start()
    }

    pub fn start(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        match self.__go_run_output() {
            Ok(output) => {
                self.__go_write_output(&output);
                if output.status.success() {
                    Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
                } else {
                    Self::__go_error(format!("exit status {}", output.status))
                }
            }
            Err(err) => Self::__go_error(err.to_string()),
        }
    }

    pub fn wait(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }
}


pub mod exec {
    use super::*;

    pub trait GoStringArg {
        fn into_go_string(self) -> String;
    }

    impl GoStringArg for String {
        fn into_go_string(self) -> String {
            self
        }
    }

    impl<'a> GoStringArg for &'a str {
        fn into_go_string(self) -> String {
            self.to_string()
        }
    }

    impl<'a> GoStringArg for &'a String {
        fn into_go_string(self) -> String {
            self.clone()
        }
    }

    impl GoStringArg for Arc<Mutex<Option<String>>> {
        fn into_go_string(self) -> String {
            self.lock().unwrap().as_ref().cloned().unwrap_or_default()
        }
    }

    pub trait GoExecCommandArgs {
        fn into_exec_args(self) -> Vec<String>;
    }

    impl GoExecCommandArgs for () {
        fn into_exec_args(self) -> Vec<String> {
            Vec::new()
        }
    }

    impl GoExecCommandArgs for Arc<Mutex<Option<Vec<String>>>> {
        fn into_exec_args(self) -> Vec<String> {
            self.lock().unwrap().as_ref().cloned().unwrap_or_default()
        }
    }

    impl<T0: GoStringArg> GoExecCommandArgs for (T0,) {
        fn into_exec_args(self) -> Vec<String> {
            vec![self.0.into_go_string()]
        }
    }

    impl<T0: GoStringArg, T1: GoStringArg> GoExecCommandArgs for (T0, T1) {
        fn into_exec_args(self) -> Vec<String> {
            vec![self.0.into_go_string(), self.1.into_go_string()]
        }
    }

    impl<T0: GoStringArg, T1: GoStringArg, T2: GoStringArg> GoExecCommandArgs for (T0, T1, T2) {
        fn into_exec_args(self) -> Vec<String> {
            vec![self.0.into_go_string(), self.1.into_go_string(), self.2.into_go_string()]
        }
    }

    impl<T0: GoStringArg, T1: GoStringArg, T2: GoStringArg, T3: GoStringArg> GoExecCommandArgs for (T0, T1, T2, T3) {
        fn into_exec_args(self) -> Vec<String> {
            vec![self.0.into_go_string(), self.1.into_go_string(), self.2.into_go_string(), self.3.into_go_string()]
        }
    }

    impl<T0: GoStringArg, T1: GoStringArg, T2: GoStringArg, T3: GoStringArg, T4: GoStringArg> GoExecCommandArgs for (T0, T1, T2, T3, T4) {
        fn into_exec_args(self) -> Vec<String> {
            vec![self.0.into_go_string(), self.1.into_go_string(), self.2.into_go_string(), self.3.into_go_string(), self.4.into_go_string()]
        }
    }

    impl<T0: GoStringArg, T1: GoStringArg, T2: GoStringArg, T3: GoStringArg, T4: GoStringArg, T5: GoStringArg> GoExecCommandArgs for (T0, T1, T2, T3, T4, T5) {
        fn into_exec_args(self) -> Vec<String> {
            vec![self.0.into_go_string(), self.1.into_go_string(), self.2.into_go_string(), self.3.into_go_string(), self.4.into_go_string(), self.5.into_go_string()]
        }
    }

    pub fn command<T0: GoStringArg, T1: GoExecCommandArgs>(_arg0: T0, _arg1: T1) -> Arc<Mutex<Option<exec_Cmd>>> {
        let mut args = vec![_arg0.into_go_string()];
        args.extend(_arg1.into_exec_args());
        Arc::new(Mutex::new(Some::<exec_Cmd>(exec_Cmd { args: Arc::new(Mutex::new(Some::<Vec<String>>(args))), ..Default::default() })))
    }
}


fn main() {
    let mut cmd = exec::command("go".to_string(), ("list".to_string(), "-export".to_string(), "-f".to_string(), "{{.Export}}".to_string(), "pkg".to_string()));
    println!("{}", format!("{}", (*Arc::new(Mutex::new(Some({ let __parts = (*(*cmd.lock().unwrap().as_ref().unwrap()).args.lock().unwrap()).as_ref().cloned().unwrap_or_default(); let __sep = " ".to_string(); __parts.join(&__sep) }))).lock().unwrap().as_ref().unwrap())));
}