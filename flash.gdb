set history save on
set confirm off
target extended-remote | openocd -c "gdb_port pipe; log_output openocd.log" -f openocd.cfg
monitor reset halt
load
monitor reset
quit
