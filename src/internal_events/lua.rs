use super::InternalEvent;
use metrics::{counter, gauge, timing};
use std::time::Duration;

pub struct LuaEventProcessed {
    pub duration: Duration,
}

impl InternalEvent for LuaEventProcessed {
    fn emit_metrics(&self) {
        timing!("processing_duration", self.duration,
            "component_kind" => "transform",
            "component_type" => "lua",
        );
        counter!("events_processed", 1,
            "component_kind" => "transform",
            "component_type" => "lua",
        );
    }
}

pub struct LuaGcTriggered {
    pub used_memory: usize,
}

impl InternalEvent for LuaGcTriggered {
    fn emit_metrics(&self) {
        gauge!("memory_used", self.used_memory as i64,
            "component_kind" => "transform",
            "component_type" => "lua",
        );
    }
}

pub struct LuaScriptError {
    pub error: rlua::Error,
}

impl InternalEvent for LuaScriptError {
    fn emit_logs(&self) {
        let error = format_error(&self.error);
        error!(message = "Error in lua script; discarding event.", %error, rate_limit_secs = 30);
    }

    fn emit_metrics(&self) {
        counter!("script_error", 1,
            "component_kind" => "transform",
            "component_type" => "lua",
        );
    }
}

fn format_error(error: &rlua::Error) -> String {
    match error {
        rlua::Error::CallbackError { traceback, cause } => format_error(&cause) + "\n" + traceback,
        err => err.to_string(),
    }
}