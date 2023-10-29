# Use the official Rust image from the Docker Hub
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/IoT.money

# Copy the local code to the container
COPY . .

# Build the code
RUN cargo build --release

# Set the default command for the container
CMD ["./target/release/IoT.money"]
