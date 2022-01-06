#script to launch VARIABLE in a new xorg server

pkexec rm /tmp/.X4-lock
pkexec X :4 -ac -terminate &
sleep 1
DISPLAY=:4 $VARIABLE