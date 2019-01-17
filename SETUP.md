add "overlays=analog-codec" to /boot/armbianEnv.txt and reboot
aplay -l
sudo apt-get install mpd mpc
mkdir ~/.config/mpd
cp /usr/share/doc/mpd/mpdconf.example.gz .config/mpd/
cd .config/mpd/
gunzip mpdconf.example.gz
mv mpdconf.example mpd.conf

