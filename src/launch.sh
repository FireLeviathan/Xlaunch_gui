

kdesu rm /tmp/.X4-lock
kdesu X :4 -ac -terminate &
sleep 1
DISPLAY=:4 $VARIABLE