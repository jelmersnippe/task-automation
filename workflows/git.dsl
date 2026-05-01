fn foreach_worktree(function) {
    print("fetching")
    git.fetch()

    print("getting worktrees")
    var worktrees = git.worktrees()

    var actions = []

    var i = 0
    var length = worktrees.len()
    while (i < length) {
        var worktree = worktrees[i]

        print(worktree)

        actions.push(fn() {function(worktree["directory"])})

        i = i + 1
    }

    parallel(actions)
}

register_task("update_worktrees", fn() {
    print("updating worktrees")
    foreach_worktree(fn(worktree_directory) {
        print("pulling for")
        print(git.in_directory(worktree_directory).current_branch())

        git.in_directory(worktree_directory).pull()
    })
})

register_task("rebase_worktrees", fn() {
    print("rebasing worktrees")
    foreach_worktree(fn(worktree_directory) {
        print("rebasing for")
        print(git.in_directory(worktree_directory).current_branch())

        git.in_directory(worktree_directory).rebase()

        print("pushing for")
        git.in_directory(worktree_directory).push("--force")
    })
})

register_task("test", fn() {
    shell.open({cwd: "~/dev/appdev", cmd: "git status"});
    shell.run({cwd: "~/dev/appdev/ide-client/data/domain-model-editor", cmd: "echo 'hello from shell.run' && sleep 5"});
})
