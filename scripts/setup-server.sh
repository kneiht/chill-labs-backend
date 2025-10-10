#!/bin/bash

# Source .env if it exists
if [ -f ".env" ]; then
    set -o allexport
    source .env
    set +o allexport
fi

# Set working directory
cd $REMOTE_DIR

# Install Docker if not already installed
if ! command -v docker &> /dev/null; then
    echo "Installing Docker..."
    # Determine the operating system
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        OS=$ID
    else
        echo "Cannot determine operating system. Using default settings for Debian."
        OS="debian"
    fi

    # Install necessary packages
    apt-get update
    apt-get install -y apt-transport-https ca-certificates curl gnupg lsb-release software-properties-common

    # Remove old Docker repositories if they exist
    rm -f /etc/apt/sources.list.d/docker*.list

    # Remove old Docker repositories from the main sources.list
    if [ -f /etc/apt/sources.list ]; then
        sed -i '/download.docker.com/d' /etc/apt/sources.list
    fi

    # Remove old Docker GPG keys
    rm -f /usr/share/keyrings/docker-archive-keyring.gpg
    if command -v apt-key &> /dev/null; then
        apt-key del 0EBFCD88 2>/dev/null || true
    fi

    # Add Docker GPG key and repository based on operating system
    if [ "$OS" = "ubuntu" ]; then
        # Installation for Ubuntu
        curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
        echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null
    else
        # Installation for Debian and other operating systems
        curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
        echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null
    fi

    # Update and install Docker
    apt-get update -o Acquire::AllowInsecureRepositories=true || true
    apt-get update
    apt-get install -y docker-ce docker-ce-cli containerd.io

    # Start and enable Docker
    systemctl enable docker
    systemctl start docker

    echo "Docker has been installed successfully!"
fi

# Install Docker Compose if not already installed
if ! command -v docker-compose &> /dev/null; then
    echo "Installing Docker Compose..."

    # Determine operating system if not already determined
    if [ -z "$OS" ]; then
        if [ -f /etc/os-release ]; then
            . /etc/os-release
            OS=$ID
        else
            echo "Cannot determine operating system. Using default settings for Debian."
            OS="debian"
        fi
    fi

    # Docker Compose installation method based on operating system
    if [ "$OS" = "ubuntu" ]; then
        # Install Docker Compose plugin on Ubuntu
        apt-get update
        apt-get install -y docker-compose-plugin

        # Create symlink to use docker-compose command
        if [ ! -f /usr/local/bin/docker-compose ]; then
            ln -s /usr/libexec/docker/cli-plugins/docker-compose /usr/local/bin/docker-compose 2>/dev/null || \
            ln -s /usr/lib/docker/cli-plugins/docker-compose /usr/local/bin/docker-compose 2>/dev/null || \
            echo "Cannot create symlink for docker-compose. Using alternative installation method."
        fi
    else
        # Install Docker Compose on Debian
        echo "Installing Docker Compose on Debian..."
        DOCKER_COMPOSE_VERSION=$(curl -s https://api.github.com/repos/docker/compose/releases/latest | grep 'tag_name' | cut -d\" -f4)
        if [ -z "$DOCKER_COMPOSE_VERSION" ]; then
            DOCKER_COMPOSE_VERSION="v2.23.3"  # Default version if cannot fetch from API
        fi
        curl -L "https://github.com/docker/compose/releases/download/${DOCKER_COMPOSE_VERSION}/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
        chmod +x /usr/local/bin/docker-compose
    fi

    # Check if Docker Compose has been installed successfully
    if ! command -v docker-compose &> /dev/null; then
        echo "Using alternative installation method for Docker Compose..."
        curl -L "https://github.com/docker/compose/releases/download/v2.23.3/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
        chmod +x /usr/local/bin/docker-compose
    fi

    echo "Docker Compose has been installed successfully!"
fi

echo "Current working directory: $(pwd)"
echo "Directory contents:"
ls -la


# Stop and remove old containers
echo "Stopping and removing old containers..."

docker-compose down

    # Load new Docker image
    echo "Loading new Docker image..."
    docker load < ${APP_CONTAINER_NAME}-image.tar.gz

# Create directories for Caddy and PostgreSQL
echo "Creating directories for Caddy and PostgreSQL..."
mkdir -p ./caddy_data
mkdir -p ./caddy_config

# Start the application with Docker Compose
echo "Starting the application..."
docker-compose up -d

# Notification about SSL
echo "Caddy will automatically obtain and renew SSL certificates..."

# Check the status of containers
echo "Checking the status of containers..."
docker-compose ps

    # Check PostgreSQL connection
    echo "Checking PostgreSQL connection..."
    sleep 5 # Wait for PostgreSQL to start
    docker exec ${APP_CONTAINER_NAME}-postgres pg_isready -U $POSTGRES_USER
if [ $? -ne 0 ]; then
    echo "Error connecting to PostgreSQL!"
else
    echo "PostgreSQL connection successful!"
fi

# Check logs of ${APP_CONTAINER_NAME} container
echo "Checking logs of ${APP_CONTAINER_NAME} container..."
docker logs $APP_CONTAINER_NAME

# Clean up unused (dangling) images
echo "Cleaning up old images..."
docker image prune -f

# Clean up unused volumes
echo "Cleaning up unused volumes..."
docker volume prune -f

# Clean up old Docker repositories
echo "Cleaning up old Docker repositories..."
rm -f /etc/apt/sources.list.d/docker*.list
if [ -f /etc/apt/sources.list ]; then
    sed -i '/download.docker.com/d' /etc/apt/sources.list
fi

echo "Installation complete!"