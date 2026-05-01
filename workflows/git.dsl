fn foreach_worktree(function) {
    git.fetch()

    var worktrees = git.worktrees()

    var i = 0
    while (i < worktrees.len()) {
        var worktree = worktrees[i]

        print(worktree)

        function(worktree["directory"])

        i = i + 1
    }
}

register_task("update_worktrees", fn() {
    print("updating worktrees")
    foreach_worktree(fn(worktree_directory) {
        git.in_directory(worktree_directory).pull()

        print(git.in_directory(worktree_directory).current_branch())
    })
})

register_task("rebase_worktrees", fn() {
    print("rebasing worktrees")
    foreach_worktree(fn(worktree_directory) {
        git.in_directory(worktree_directory).rebase()

        print(git.in_directory(worktree_directory).current_branch())

        git.in_directory(worktree_directory).push("--force")
    })
})

register_task("test", fn() {
    git.push("--force")
})
