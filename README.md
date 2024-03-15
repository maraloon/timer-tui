# Timer CLI

## about
It's a timer
![Screenshot of cli](/files/screenshot.png "Screenshot")

Write in rust with [Ratatui](https://github.com/ratatui-org/ratatui)

## install
```
git clone https://github.com/maraloon/timer-tui.git
cd timer-tui
cargo install --path=.
```

## usage
```
timer-tui 5
timer-tui 5s
timer-tui 5m
timer-tui 5h
```

## tweaking
add alias for sh so u can use `t tea` or `t 5m`
```bash
t() {
    local template=$1
    local message=$2

    if [ $template = "tea" ]; then
        timer-tui 10m
        message="Tea"
    elif [ $template = "bath" ]; then
        timer-tui 6m
        message="Bath"
    else
        timer-tui $1
        if [ -z $message ]; then
            message="It's time to stop!"
        fi
    fi

    notify-send "Timer" $message
}
```

## credentials

Inspired by [timer-bin](https://github.com/caarlos0/timer)
