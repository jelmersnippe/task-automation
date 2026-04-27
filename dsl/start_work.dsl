fn task(name, function) {
    register_task(name, function)
}

task("editor_terminal", fn() {
    spawn_terminal("~/dev/task-automation", "git fetch && git pull && cargo build && nvim .")
})

task("secondary_terminal", fn() {
    spawn_terminal("~/dev/task-automation")
})

task("start_work", fn() {
    run("editor_terminal")
    run("secondary_terminal")
})
