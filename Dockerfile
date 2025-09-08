# rust is a local image, change to rust:latest
FROM rust
# the working directory of the container
WORKDIR /app 

# copy all file current directory in host machine to current directory of the container (/app)
COPY . . 

# RUN cargo check

# run the command 
# RUN apt-get update && apt-get install -y curl 

# also run this command
# CMD ["node", "src/index.js"] 

# EXPOSE 3000
