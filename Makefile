CARGO = cargo
TRUNK = trunk 
RELEASE_FLAGS := --release 

.PHONY: frontend backend watch

all: frontend backend 
	$(CARGO) run -p backend $(RELEASE_FLAGS) 

watch: backend 
	cd frontend/ && $(TRUNK) watch $(RELEASE_FLAGS) &
	$(CARGO) run -p backend $(RELEASE_FLAGS) 

frontend:
	cd frontend/ && $(TRUNK) build $(RELEASE_FLAGS) 

backend:
	$(CARGO) build -p backend $(RELEASE_FLAGS)