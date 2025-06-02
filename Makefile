migrations := ./src-tauri/src/api/fms_core/db/migrations

_APP_NAME=fuel-monitoring-system
_BUILD_NAME=fuel_monitoring_system
_FMS_VERSION=0.1.0
_TARGET_FOLDER=./src-tauri/target
_BUNDLE_FOLDER=/bundle/deb
_POSTFIX=amd64.deb

_FULL_NAME=${_BUILD_NAME}_${_FMS_VERSION}_${_POSTFIX}
_RELEASE_FOLDER=${_TARGET_FOLDER}/release${_BUNDLE_FOLDER}/${_FULL_NAME}
_DEBUG_FOLDER=${_TARGET_FOLDER}/debug${_BUNDLE_FOLDER}/${_FULL_NAME}

_DOWNLOAD_FOLDER=~/Downloads/${_FULL_NAME}
nikita_password = Vfcnbyj
yuriy_password = 1234
nikita_folder = ${_RELEASE_FOLDER}
yuriy_folder = ${_DOWNLOAD_FOLDER}
_PASSWORD = $(if $(shell whoami | grep -i nikita),${nikita_password},${yuriy_password})
_APP_FOLDER = $(if $(shell whoami | grep -i nikita),${nikita_folder},${yuriy_folder})
_DUMPS_FOLDER=./src-tauri/src/api/fms_core/db/.dumps


help:

create_revision name=name:
	diesel migration generate ${name} --migration-dir ${migrations}

install:
	echo ${_PASSWORD} | sudo dpkg -i ${_RELEASE_FOLDER}

install_debug:
	echo ${_PASSWORD} | sudo dpkg -i $(_DEBUG_FOLDER)

uninstall:
	echo ${_PASSWORD} | sudo dpkg -r ${_APP_NAME}

build:
	pnpm tauri build

run_dev:
	pnpm tauri dev 

build_debug:
	pnpm tauri build --debug

upgrade_db:
	diesel migration run --migration-dir ${migrations}

upgrade_app:
	echo ${_PASSWORD} | sudo -S dpkg -r ${_APP_NAME}
	echo ${_PASSWORD} | sudo -S dpkg -i ${_APP_FOLDER}

downgrade_db:
	diesel migration redo --migration-dir ${migrations}

run_db:
	cd docker && docker compose up --build -d

dump_db:
	mkdir ${_DUMPS_FOLDER} docker exec -it postgres pg_dump -U postgres -d fms > ${_DUMPS_FOLDER}/$(shell date +%Y-%m-%d_%H-%M-%S)_dump.sql

load_latest_dump:
	docker exec -it postgres psql -U postgres -d fms < ${_DUMPS_FOLDER/$(shell ls -t ${_DUMPS_FOLDER} | head -n 1)