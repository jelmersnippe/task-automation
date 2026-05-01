fn setup(primary, secondary) {
    if (tmux.has_session("work") == false) {
        tmux.new_session("work")
            .new_window("editor",   {cwd: primary, cmd: "nvim ."})
            .new_window("shell",    {cwd: primary})
            .new_window("opencode", {cwd: primary, cmd: "opencode ."})
            .kill_window("0")
    }

    if (tmux.has_session("watchers") == false) {
        tmux.new_session("watchers")
            .new_window("main", {cwd: primary})
            .split_pane("main", {cwd: primary})
            .split_pane("main", {cwd: secondary})
            .set_layout("main", "main-vertical")
            .kill_window("0")
    }

    shell.open({cwd: primary, cmd: "tmux attach -t work"})
    shell.open({cwd: primary, cmd: "tmux attach -t watchers"})
}

register_task("setup", fn(primary) {
    setup(primary, primary)
})

register_task("setup_dual", fn(primary, secondary) {
    setup(primary, secondary)
})
