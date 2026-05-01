fn foreach_worktree(function) {
    print("fetching")
    git.fetch()

    print("getting worktrees")
    var worktrees = git.worktrees()

    var actions = []
    
    print("worktree count")
    var i = 0
    var length = worktrees.len()
    while (i < length) {
        var worktree = worktrees[i]

        print(worktree)

        actions.push(fn() {function(worktree["directory"])})

        i = i + 1
    }

    print("actions")
    print(actions)

    parallel(actions)
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
