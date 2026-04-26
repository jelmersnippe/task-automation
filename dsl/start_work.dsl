fn open_main_terminal() {
    spawn_terminal("~/dev/task-automation", "git fetch && git pull && cargo build && nvim .")
}

fn open_secondary_terminal() {
    spawn_terminal("~/dev/task-automation")
}

fn start_work() {
    open_main_terminal()
    open_secondary_terminal()
}

register_task("start_work", start_work)
