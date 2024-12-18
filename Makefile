migrations := ./src-tauri/src/api/fms_core/db/migrations

_APP_NAME="fuel-monitoring-system"
_BUILD_NAME="fuel_monitoring_system"
_FMS_VERSION=0.1.0
_RELEASE_FOLDER="./src-tauri/target/release/bundle/deb/${_BUILD_NAME}_${_FMS_VERSION}_amd64.deb"
_DEBUG_FOLDER="./src-tauri/target/debug/bundle/deb/${_BUILD_NAME}_${_FMS_VERSION}_amd64.deb"

help:

create_revision name=name:
	diesel migration generate ${name} --migration-dir ${migrations}

install:
	sudo dpkg -i ${_RELEASE_FOLDER}

install_debug:
	sudo dpkg -i $(_DEBUG_FOLDER)

uninstall:
	sudo dpkg -r ${_APP_NAME}

build:
	pnpm tauri build

run_dev:
	pnpm tauri dev

build_debug:
	pnpm tauri build --debug

upgrade_db:
	diesel migration run --migration-dir ${migrations}

downgrade_db:
	diesel migration redo --migration-dir ${migrations}

run_db:
	cd docker && docker compose up --build -d