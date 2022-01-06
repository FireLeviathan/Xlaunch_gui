

kdesu rm /tmp/.X4-lock
export WINEPREFIX="/home/fire/.wine"
kdesu X :4 -ac -terminate &
sleep 1
DISPLAY=:4 wine $VARIABLE