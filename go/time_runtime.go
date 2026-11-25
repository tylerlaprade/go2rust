package main

import (
	"go/ast"
	"strconv"
	"strings"
)

// time, translated from source, leaves its clock and timers to the runtime:
// Sleep, now, runtimeNow, runtimeNano, newTimer, stopTimer and resetTimer
// have no body. writeTimeRuntimeHookBody writes them, and
// generateGoTimeRuntime writes the clock and the timer thread they call.
//
// Timers need a thread that calls their functions, so they exist only in
// goroutine mode. Translating time from source puts a program in that mode:
// time makes channels and starts goroutines.

// writeTimeRuntimeHookBody writes the body of one of time's runtime
// functions, or reports false for any other function.
func writeTimeRuntimeHookBody(out *strings.Builder, fn *ast.FuncDecl, indent string) bool {
	param := func(i int) string {
		return RustLocalIdent(functionParamName(fn, i, "__arg"+strconv.Itoa(i)))
	}
	readInt := func(name string) string {
		var b strings.Builder
		b.WriteString("(*")
		b.WriteString(name)
		WriteBorrowMethod(&b, false)
		b.WriteString(".as_ref().unwrap()).clone()")
		return b.String()
	}
	line := func(text string) {
		out.WriteString(indent)
		out.WriteString(text)
		out.WriteString("\n")
	}
	switch fn.Name.Name {
	case "runtimeNano":
		NeedGoTimeRuntime()
		line("go_time_runtime_nano()")
		return true
	case "now", "runtimeNow":
		NeedGoTimeRuntime()
		line("let __wall = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();")
		line("(__wall.as_secs() as i64, __wall.subsec_nanos() as i32, go_time_runtime_nano())")
		return true
	case "Sleep":
		// A Duration is a newtype over its int64 cell.
		line("let __duration = " + readInt(param(0)) + ";")
		line("let __nanos = " + readInt("__duration.0") + ";")
		line("if __nanos > 0 {")
		line("    std::thread::sleep(std::time::Duration::from_nanos(__nanos as u64));")
		line("}")
		return true
	case "newTimer", "stopTimer", "resetTimer":
		if !NeedsConcurrentWrapper() {
			line("panic!(\"time." + fn.Name.Name + " runs only in goroutine mode\");")
			return true
		}
		NeedGoTimeRuntime()
		switch fn.Name.Name {
		case "newTimer":
			writeTimeNewTimerHookBody(out, fn, indent, param, readInt)
		case "stopTimer":
			line("let __owner = (*" + param(0) + ".lock().unwrap().as_ref().unwrap())." + timerInitFieldName + ".clone();")
			line("go_time_timer_stop(&__owner)")
		case "resetTimer":
			line("let __owner = (*" + param(0) + ".lock().unwrap().as_ref().unwrap())." + timerInitFieldName + ".clone();")
			line("go_time_timer_reset(&__owner, " + readInt(param(1)) + ", " + readInt(param(2)) + ")")
		}
		return true
	}
	return false
}

// timerInitFieldName is Timer.initTimer, the cell that names a timer to the
// timer thread. A Ticker's initTicker is the same cell, because time turns a
// *Ticker into a *Timer by sharing its fields.
var timerInitFieldName = ToSnakeCase("initTimer")

// writeTimeNewTimerHookBody writes newTimer(when, period, f, arg, cp): a new
// Timer whose initTimer is set, registered to call f(arg, 0, delta) at when
// and then every period. A cp that is not nil is the timer's channel, which
// Stop and Reset empty, as the runtime does since Go 1.23.
func writeTimeNewTimerHookBody(out *strings.Builder, fn *ast.FuncDecl, indent string, param func(int) string, readInt func(string) string) {
	timerType := "Timer"
	if fn.Type.Results != nil && len(fn.Type.Results.List) == 1 {
		if star, ok := fn.Type.Results.List[0].Type.(*ast.StarExpr); ok {
			timerType = goTypeToRustBase(star.X)
		}
	}
	f, arg, cp := param(2), param(3), param(4)
	// readInt reads a cell that holds a value; cp's holds None for nil.
	lines := []string{
		"let __when = " + readInt(param(0)) + ";",
		"let __period = " + readInt(param(1)) + ";",
		"let __timer = " + timerType + " { " + timerInitFieldName + ": Arc::new(Mutex::new(Some(true))), ..Default::default() };",
		"let __owner = __timer." + timerInitFieldName + ".clone();",
		"let __drain: Option<Box<dyn Fn() -> bool + Send + Sync>> = if (*" + cp + ".lock().unwrap()).map_or(false, |__cp| __cp != 0) {",
		"    match " + arg + ".lock().unwrap().as_ref().and_then(|__arg| __arg.downcast_ref::<GoChannel<Time>>()) {",
		"        Some(__channel) => {",
		"            let __channel = __channel.clone();",
		"            Some(Box::new(move || __channel.try_recv().is_some()))",
		"        }",
		"        None => None,",
		"    }",
		"} else {",
		"    None",
		"};",
		"let __fire_f = " + f + ".clone();",
		"let __fire_arg = " + arg + ".clone();",
		"go_time_timer_start(&__owner, __when, __period, Box::new(move |__delta: i64| {",
		"    let mut __f_guard = __fire_f.lock().unwrap();",
		"    let __f = __f_guard.as_mut().expect(\"call of nil function\");",
		"    (*__f)(__fire_arg.clone(), Arc::new(Mutex::new(Some(0usize))), Arc::new(Mutex::new(Some(__delta))));",
		"}), __drain);",
		"Arc::new(Mutex::new(Some(__timer)))",
	}
	for _, text := range lines {
		out.WriteString(indent)
		out.WriteString(text)
		out.WriteString("\n")
	}
}

// generateGoTimeRuntime writes the monotonic clock and, in goroutine mode,
// the timers: one thread that sleeps until the earliest timer is due and
// calls its function.
//
// A timer is named by its initTimer cell, which the registry holds so the
// address stays the timer's. A one-shot timer that fires after the program
// dropped it leaves the registry; so do stopped ones, swept as timers start.
// A dropped ticker that was never stopped keeps ticking, where Go's
// collector would free it.
func generateGoTimeRuntime(out *strings.Builder) {
	out.WriteString(`
fn go_time_runtime_nano() -> i64 {
    static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    let start = *START.get_or_init(std::time::Instant::now);
    start.elapsed().as_nanos() as i64 + 1
}
`)
	if !NeedsConcurrentWrapper() {
		return
	}
	out.WriteString(`
type GoTimeTimerOwner = std::sync::Arc<std::sync::Mutex<Option<bool>>>;

struct GoTimeTimer {
    owner: GoTimeTimerOwner,
    generation: u64,
    active: bool,
    when: i64,
    period: i64,
    fire: Box<dyn FnMut(i64) + Send>,
    drain: Option<Box<dyn Fn() -> bool + Send + Sync>>,
}

struct GoTimeTimers {
    timers: std::collections::HashMap<usize, GoTimeTimer>,
    queue: std::collections::BinaryHeap<std::cmp::Reverse<(i64, usize, u64)>>,
    next_generation: u64,
    sweep_at: usize,
    running: bool,
}

fn go_time_timers() -> &'static (std::sync::Mutex<GoTimeTimers>, std::sync::Condvar) {
    static TIMERS: std::sync::OnceLock<(std::sync::Mutex<GoTimeTimers>, std::sync::Condvar)> = std::sync::OnceLock::new();
    TIMERS.get_or_init(|| {
        (
            std::sync::Mutex::new(GoTimeTimers {
                timers: std::collections::HashMap::new(),
                queue: std::collections::BinaryHeap::new(),
                next_generation: 0,
                sweep_at: 64,
                running: false,
            }),
            std::sync::Condvar::new(),
        )
    })
}

fn go_time_timer_key(owner: &GoTimeTimerOwner) -> usize {
    std::sync::Arc::as_ptr(owner) as *const () as usize
}

fn go_time_timer_lock() -> std::sync::MutexGuard<'static, GoTimeTimers> {
    go_time_timers().0.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Schedules the timer at when, with a generation that retires its earlier
/// schedules, and wakes the timer thread to look at it. Reports whether the
/// timer was already scheduled.
fn go_time_timer_schedule(state: &mut GoTimeTimers, key: usize, when: i64, period: i64) -> bool {
    state.next_generation += 1;
    let generation = state.next_generation;
    let timer = state.timers.get_mut(&key).unwrap();
    let was_active = timer.active;
    timer.generation = generation;
    timer.active = true;
    timer.when = when;
    timer.period = period;
    state.queue.push(std::cmp::Reverse((when, key, generation)));
    go_time_timers().1.notify_all();
    was_active
}

/// The timer thread. A timer's function runs with the registry locked, so
/// Stop and Reset see it either before or after the send; time's functions
/// only send without blocking or start a goroutine.
fn go_time_timer_run() {
    let mut state = go_time_timer_lock();
    loop {
        let Some(std::cmp::Reverse((when, key, generation))) = state.queue.peek().copied() else {
            state = go_time_timers().1.wait(state).unwrap_or_else(|poisoned| poisoned.into_inner());
            continue;
        };
        let now = go_time_runtime_nano();
        if when > now {
            let wait = std::time::Duration::from_nanos((when - now) as u64);
            state = go_time_timers().1.wait_timeout(state, wait).unwrap_or_else(|poisoned| poisoned.into_inner()).0;
            continue;
        }
        let state_ref = &mut *state;
        state_ref.queue.pop();
        let mut forget = false;
        if let Some(timer) = state_ref.timers.get_mut(&key) {
            if timer.active && timer.generation == generation && timer.when == when {
                if timer.period > 0 {
                    // Like the runtime, skip the ticks that are already late.
                    let late = now - timer.when;
                    timer.when = timer.when.saturating_add(timer.period.saturating_mul(1 + late / timer.period));
                    state_ref.queue.push(std::cmp::Reverse((timer.when, key, generation)));
                } else {
                    timer.active = false;
                    forget = std::sync::Arc::strong_count(&timer.owner) == 1;
                }
                (timer.fire)(now - when);
            }
        }
        if forget {
            state_ref.timers.remove(&key);
        }
    }
}

fn go_time_timer_start(owner: &GoTimeTimerOwner, when: i64, period: i64, fire: Box<dyn FnMut(i64) + Send>, drain: Option<Box<dyn Fn() -> bool + Send + Sync>>) {
    let key = go_time_timer_key(owner);
    let mut state = go_time_timer_lock();
    if state.timers.len() >= state.sweep_at {
        state.timers.retain(|_, timer| timer.active || std::sync::Arc::strong_count(&timer.owner) > 1);
        state.sweep_at = std::cmp::max(64, state.timers.len() * 2);
    }
    state.timers.insert(key, GoTimeTimer {
        owner: owner.clone(),
        generation: 0,
        active: false,
        when,
        period,
        fire,
        drain,
    });
    go_time_timer_schedule(&mut state, key, when, period);
    if !state.running {
        state.running = true;
        std::thread::spawn(go_time_timer_run);
    }
}

/// Empties a stopped or reset timer's channel, so no value sent before the
/// call is received after it, and reports whether there was one.
fn go_time_timer_drain(timer: &GoTimeTimer) -> bool {
    match &timer.drain {
        Some(drain) => drain(),
        None => false,
    }
}

fn go_time_timer_stop(owner: &GoTimeTimerOwner) -> bool {
    let key = go_time_timer_key(owner);
    let mut state = go_time_timer_lock();
    let Some(timer) = state.timers.get_mut(&key) else {
        return false;
    };
    let was_active = timer.active;
    timer.active = false;
    let drained = go_time_timer_drain(timer);
    was_active || drained
}

fn go_time_timer_reset(owner: &GoTimeTimerOwner, when: i64, period: i64) -> bool {
    let key = go_time_timer_key(owner);
    let mut state = go_time_timer_lock();
    let Some(timer) = state.timers.get(&key) else {
        return false;
    };
    let drained = go_time_timer_drain(timer);
    let was_active = go_time_timer_schedule(&mut state, key, when, period);
    was_active || drained
}
`)
}
