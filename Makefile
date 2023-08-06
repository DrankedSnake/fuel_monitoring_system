migrations := ./src-tauri/src/api/fms_core/db/migrations

upgrate_db:
	diesel migration run --migration-dir ${migrations}

downgrade_db:
	diesel migration redo --migration-dir ${migrations}

create_revision name=name:
	diesel migration generate ${name} --migration-dir ${migrations}