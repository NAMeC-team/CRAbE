#!/usr/bin/env bash
set -euo pipefail

CRABE_HOME=$(realpath "$(dirname "$0")")

source "$CRABE_HOME/.env"
if [ -f "$CRABE_HOME/.env.local" ]; then
  source "$CRABE_HOME/.env.local"
fi

declare -r MULTICAST_ADDR="${CRB_MULTICAST_ADDR:-224.5.23.2}"
declare -r MULTICAST_PORT="${CRB_MULTICAST_PORT:-10020}"
declare -r CMD_LISTEN_PORT="${CRB_CMD_LISTEN_PORT:-20011}"
declare -r BLUE_STATUS_PORT="${CRB_BLUE_STATUS_PORT:-30011}"
declare -r YELLOW_STATUS_PORT="${CRB_YELLOW_STATUS_PORT:-30012}"
declare -r SIM_CONTROL_PORT="${CRB_SIM_CONTROL_PORT:-10300}"
declare -r BLUE_CONTROL_PORT="${CRB_BLUE_CONTROL_PORT:-10301}"
declare -r YELLOW_CONTROL_PORT="${CRB_YELLOW_CONTROL_PORT:-10302}"

if [ ! -d "$CRABE_HOME/.grsim" ]; then
  mkdir "$CRABE_HOME/.grsim"
fi


if [ ! -f "$CRABE_HOME/.grsim/.grsim.xml" ]; then
  echo "Config file not found, generating it"
  cat <<EOF > "$CRABE_HOME/.grsim/.grsim.xml"
<?xml version="1.0" encoding="utf-8"?>
<VarXML>
 	<Var name="Communication" type="list">
		<Var name="Vision multicast address" type="string">
		$MULTICAST_ADDR
		</Var>
		<Var name="Vision multicast port" type="int" minval="" maxval="">
		$MULTICAST_PORT
		</Var>
		<Var name="Command listen port" type="int" minval="" maxval="">
		$CMD_LISTEN_PORT
		</Var>
		<Var name="Blue Team status send port" type="int" minval="" maxval="">
		$BLUE_STATUS_PORT
		</Var>
		<Var name="Yellow Team status send port" type="int" minval="" maxval="">
		$YELLOW_STATUS_PORT
		</Var>
		<Var name="Simulation control port" type="int" minval="" maxval="">
		$SIM_CONTROL_PORT
		</Var>
		<Var name="Blue team control port" type="int" minval="" maxval="">
		$BLUE_CONTROL_PORT
		</Var>
		<Var name="Yellow team control port" type="int" minval="" maxval="">
		$YELLOW_CONTROL_PORT
		</Var>
 	</Var>
</VarXML>
EOF
fi

echo -e "executing cmd\ndocker run --net=host -v $CRABE_HOME/.grsim:/home/default --rm --name=grsim-docker -eVNC_PASSWORD=vnc -eVNC_GEOMETRY=1920x1080 robocupssl/grsim vnc &> $CRABE_HOME/.grsim/log"
docker run --net=host -v $CRABE_HOME/.grsim:/home/default --rm --name=grsim-docker -eVNC_PASSWORD=vnc -eVNC_GEOMETRY=1920x1080 robocupssl/grsim vnc &> $CRABE_HOME/.grsim/log
