register_task("update_worktrees", fn() {
    var worktrees = git.worktrees()

    var i = 0

    while (i < len(worktrees)) {
        var worktree = worktrees[i]

        print(worktree)

        var current_branch = git.in_directory(worktree["directory"]).current_branch()
        print(current_branch)

        i = i + 1
    }
})
