migrate:
	diesel migration run

load-def:
	diesel print-schema > model/src/schema.rs