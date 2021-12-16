# processkeeper
A tiny tool to run linux shell command and keep it runing permanently. it can running as daomon.

# usage
- sudo processkeeper -d "ls -l" // -d to run as daomon, all log will be put at /tmp/processkeeper/<pid>
- sudo processkeeper  "ls -l"   // run as normal command, used with tmux to keep it run off session
