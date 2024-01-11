# Use the official Rust image as the base image
FROM rust:1.70 as builder

# Set the working directory
WORKDIR /api

# Copy the source code into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Create a new stage for the smaller runtime image
FROM debian:bullseye-slim

# Set the working directory in the runtime image
WORKDIR /api

# Copy only the necessary files from the builder stage
COPY --from=builder /api/target/release/axum-api .

# Expose the port that the application will run on
EXPOSE 8000

# Run the application
CMD ["./axum-api"]
