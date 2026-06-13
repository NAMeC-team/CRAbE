CRAbE - Central AI of NAMeC

Modules :
- crabe
- crabe_ai
- crabe_filter
- crabe_framework
- crabe_guard
- crabe_io
- crabe_math
- crabe_navigation
- crabe_protocol

# Running dockerized grSim
If you do not have grSim installed, you can use the `start-docker-sim.sh` script to run it, assuming you have Docker installed.

To see the grSim window, you will need to run a VNC client. We recommend TigerVNC, that you can install with snap
```sh
sudo snap install tigervnc
```
To connect to the dockerized grSim, run this command and when prompted for a password, enter `vnc`
```sh
tigervnc localhost:5900
```
