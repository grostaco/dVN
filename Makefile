CARGO = cargo
TRUNK = trunk 
RELEASE_FLAGS := --release 

.PHONY: frontend backend frontend_watch

all: frontend backend 
	$(CARGO) run -p backend $(RELEASE_FLAGS) 

watch: frontend_watch backend 
	$(CARGO) run -p backend $(RELEASE_FLAGS) 

frontend:
	cd frontend/ && $(TRUNK) build $(RELEASE_FLAGS) 

frontend_watch:
	cd frontend/ && $(TRUNK) watch $(RELEASE_FLAGS) &

backend:
	$(CARGO) build -p backend $(RELEASE_FLAGS)