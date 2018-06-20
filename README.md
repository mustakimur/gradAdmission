# admission

Steps to deploy:
Install the nightly rust and checkout the source code
build the release version of the server
create a directory for the server and copy the binary and related files to it
install poppler-util
allow the server to bind to privileged ports setcap cap_net_bind_service=+ep server
create a directory for the database and export it as DATABASE_URL
run the hash program compiled in the first step to create a new hash for your admin account
Edit the migration/init/up.sql to included the hash and slat
Run diesel migration run to create the database
Generate a self-signed certificate openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes
Start the server with ROCKET_ENV=production ./server
