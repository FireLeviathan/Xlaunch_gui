#script to launch VARIABLE in a new xorg server with wine

export WINEPREFIX="/home/fire/.wine"
pkexec rm /tmp/.X4-lock
pkexec X :4 -ac -terminate &
sleep 10
DISPLAY=:4 wine $VARIABLE