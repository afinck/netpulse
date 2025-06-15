#!/bin/sh

echo "Post install test script" > /tmp/netpulse-post.log


echo "Creating user netpulse..."
if ! id netpulse >/dev/null 2>&1; then
    adduser --system --group --no-create-home netpulse
fi

echo "Creating working directory..."
install -d -o netpulse -g netpulse /var/lib/netpulse

echo "Enabling service..."
systemctl daemon-reexec
systemctl enable netpulse.service
systemctl start netpulse.service

echo "Please install the Ookla Speedtest CLI manually: https://www.speedtest.net/apps/cli"
exit 0
# End of post-installation script