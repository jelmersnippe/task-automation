fn run_task(name, task) {
    task()
    print("Ran task: " + name)
}

fn open_main_terminal() {
    spawn_terminal("~/dev/task-automation", "git fetch && git pull && cargo build && nvim .")
}

fn open_secondary_terminal() {
    spawn_terminal("~/dev/task-automation")
}

run_task("open main terminal", open_main_terminal)
run_task("open secondary terminal", open_secondary_terminal)
