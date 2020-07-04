migrate:
	diesel migrate run

load-def:
	diesel print-schema > model/src/schema.rs