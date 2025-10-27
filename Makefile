LOGFILE := run.log

SESSION ?=

run:
ifeq ($(SESSION),)
	@read -p "Enter session: " session; \
	echo "Running with session: $$session"; \
	cargo run -- --session $$session 2>&1 | tee -a $(LOGFILE)
else
	@echo "Running with session: $(SESSION)"; \
	cargo run -- --session $(SESSION) 2>&1 | tee -a $(LOGFILE)
endif

clear-log:
	@echo "" > $(LOGFILE)
	@echo "Log file $(LOGFILE) cleared."

