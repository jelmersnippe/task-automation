register_task("update_worktrees", fn() {
    var worktrees = git.list_worktrees()

    var i = 0

    while (i < len(worktrees)) {
        var worktree = worktrees[i]

        print(worktree)
        i = i + 1
    }
})
