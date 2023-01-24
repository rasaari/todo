Simple task management tool. tasks.toml file is created to the working directory upon creating the first task. When listing tasks, the tasks without any dependencies are shown first.

List all tasks
`todo list`

First column indicates if task is blocked. BLK means blockend and empty means free. Second column is task id. Third lists task dependecies as ids. Last column is the task description.

Add new task
`todo add "task description"`

and if you want to mark what tasks needs to be done first, include debendencies with
`todo add "task with depencies" -d 2 -d 7`
Task above will be dependent of tasks id 2 and 7.

A task can be edited with
`todo edit 2 -t "New description for a task" -d 3`
Both -t or --task and -d or --depends are optional.

When a task is done, it can be closed or removed with
`todo close 1`
`todo remove 1`
For now both does the same thing.
