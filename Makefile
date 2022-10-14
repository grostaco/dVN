CARGO = cargo
TRUNK = trunk 
RELEASE_FLAGS := --release 

.PHONY: frontend backend watch deploy

all: frontend backend 
	$(CARGO) run -p backend $(RELEASE_FLAGS) 

watch: backend 
	cd frontend/ && $(TRUNK) watch $(RELEASE_FLAGS) &
	$(CARGO) run -p backend $(RELEASE_FLAGS) 

frontend:
	cd frontend/ && $(TRUNK) build $(RELEASE_FLAGS) 

backend:
	$(CARGO) build -p backend $(RELEASE_FLAGS)

deploy:	
	cd .
	rm -r ./target/deploy/*
	mkdir -p ./target/deploy
	cp -r backend/static target/deploy/
	cp target/release/backend.exe target/deploy/
	tar -czvf target/deploy.tar.gz target/deploy/

	