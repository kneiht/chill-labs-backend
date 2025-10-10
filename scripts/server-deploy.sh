#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Source .env if it exists
if [ -f ".env" ]; then
    set -o allexport
    source .env
    set +o allexport
fi


# Read from env or use defaults
SERVER_IP=${SERVER_IP}
USERNAME=${SERVER_USER:-"root"}
SSH_PORT=${SSH_PORT:-"22"}
DOMAIN=${DOMAIN}

REMOTE_DIR=${REMOTE_DIR}

# Path to project root (from env)
PROJECT_ROOT=${PROJECT_ROOT}
BACKEND_DIR=${BACKEND_DIR}


# Check if server_ip is still default
if [ "$SERVER_IP" == "your_server_ip" ]; then
    echo -e "${RED}Please provide the actual server IP address!${NC}"
    echo -e "${YELLOW}Usage: ./scripts/server-deploy.sh [server_ip] [username] [ssh_port]${NC}"
    echo -e "${YELLOW}Or set SERVER_IP, SERVER_USER, SERVER_PASSWORD, DOMAIN in .env${NC}"
    exit 1
fi


# Set up SSH Control Master to reuse SSH connection
SSH_CONTROL_PATH="/tmp/ssh_mux_%h_%p_%r"
SSH_OPTS="-o ControlMaster=auto -o ControlPath=$SSH_CONTROL_PATH -o ControlPersist=1h -o StrictHostKeyChecking=accept-new"
SSH_CMD="ssh $SSH_OPTS -p $SSH_PORT $USERNAME@$SERVER_IP"
echo -e $SSH_CMD
SCP_CMD="scp -P $SSH_PORT $SSH_OPTS"

# Check if SSH key authentication works
echo -e "${BLUE}Checking SSH connection...${NC}"
if $SSH_CMD -o BatchMode=yes "echo 'SSH key authentication works!'" &>/dev/null; then
    echo -e "${GREEN}✓ SSH key authentication works! No password needed.${NC}"
else
    echo -e "${YELLOW}⚠️ SSH key authentication not working. You may need to enter password.${NC}"
    echo -e "${YELLOW}To set up SSH key authentication, run: ssh-copy-id -p $SSH_PORT $USERNAME@$SERVER_IP${NC}"
    if [ -n "$SERVER_PASSWORD" ]; then
        echo -e "${YELLOW}Using SERVER_PASSWORD from env for authentication.${NC}"
        SSH_CMD="sshpass -p '$SERVER_PASSWORD' ssh $SSH_OPTS -p $SSH_PORT $USERNAME@$SERVER_IP"
        SCP_CMD="sshpass -p '$SERVER_PASSWORD' scp -P $SSH_PORT $SSH_OPTS"
        echo -e $SSH_CMD
    fi
fi


echo -e "${BLUE}=== BACKEND DEPLOY SCRIPT ===${NC}"
echo -e "${YELLOW}Server IP: ${SERVER_IP}${NC}"
echo -e "${YELLOW}Username: ${USERNAME}${NC}"
echo -e "${YELLOW}SSH Port: ${SSH_PORT}${NC}"
echo -e "${YELLOW}Domain: ${DOMAIN}${NC}"
echo -e "${YELLOW}Remote Directory: ${REMOTE_DIR}${NC}"
echo ""


# Create directories on server
echo -e "${BLUE}Creating directories on server...${NC}"
$SSH_CMD "mkdir -p $REMOTE_DIR && mkdir -p $REMOTE_DIR/static"

if [ $? -ne 0 ]; then
    echo -e "${RED}Failed to create directories on server!${NC}"
    echo -e "${YELLOW}Check SERVER_PASSWORD in .env or install sshpass locally.${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Directories created successfully!${NC}"




# Create directories on server
echo -e "${BLUE}Creating directories on server...${NC}"
$SCP_CMD ./scripts/docker-compose.yml $USERNAME@$SERVER_IP:$REMOTE_DIR/
$SCP_CMD ./scripts/setup-server.sh $USERNAME@$SERVER_IP:$REMOTE_DIR/
$SCP_CMD ./scripts/Caddyfile $USERNAME@$SERVER_IP:$REMOTE_DIR/
if [ -f ".env" ]; then
    $SCP_CMD .env $USERNAME@$SERVER_IP:$REMOTE_DIR/
    echo -e "${GREEN}✓ Copied .env file${NC}"
else
    echo -e "${RED}.env file not found. Please create .env with required variables.${NC}"
    exit 1
fi

if [ $? -ne 0 ]; then
    echo -e "${RED}Failed to create directories on server!${NC}"
    echo -e "${YELLOW}Check SERVER_PASSWORD in .env or install sshpass locally.${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Directories created successfully!${NC}"




# Ask user if they want to build image
read -p "Do you want to build and copy Docker image to server? (y/n): " BUILD_IMAGE
if [[ "$BUILD_IMAGE" =~ ^[Yy]$ ]]; then

    # Step 1: Build Docker image
    echo -e "${BLUE}Building Docker image...${NC}"
    docker build -t english-coaching:latest -f scripts/Dockerfile .

    if [ $? -ne 0 ]; then
        echo -e "${RED}Error building Docker image!${NC}"
        exit 1
    fi
    echo -e "${GREEN}✓ Docker image built successfully!${NC}"

    # Step 2: Save Docker image to file
    echo -e "${BLUE}Saving Docker image to file...${NC}"
    cd scripts &&  docker save english-coaching:latest | gzip > english-coaching-image.tar.gz

    if [ $? -ne 0 ]; then
        echo -e "${RED}Error saving Docker image!${NC}"
        exit 1
    fi
    echo -e "${GREEN}✓ Docker image saved successfully!${NC}"

    $SCP_CMD english-coaching-image.tar.gz $USERNAME@$SERVER_IP:$REMOTE_DIR/
    if [ $? -ne 0 ]; then
        echo -e "${RED}Error copying Docker image file to server!${NC}"
        exit 1
    fi
    echo -e "${GREEN}✓ Docker image copied successfully!${NC}"
    cd ..
fi




# Set up installation script on server
echo -e "${BLUE}Setting up installation script on server...${NC}"
$SSH_CMD "chmod +x $REMOTE_DIR/setup-server.sh"

# Run installation script on server
echo -e "${BLUE}Running installation script on server...${NC}"
$SSH_CMD "cd $REMOTE_DIR && ./setup-server.sh"

if [ $? -ne 0 ]; then
    echo -e "${RED}Error running installation script!${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Installation on server successful!${NC}"




# Check status of containers
echo -e "${BLUE}Checking status of containers...${NC}"
$SSH_CMD "cd $REMOTE_DIR && docker-compose ps"

# Check logs of app container
echo -e "${BLUE}Checking logs of backend ${APP_CONTAINER_NAME} container...${NC}"
$SSH_CMD "cd $REMOTE_DIR && docker logs ${APP_CONTAINER_NAME} 2>&1 | tail -n 20"

# Check logs of caddy container
# echo -e "${BLUE}Checking logs of caddy container...${NC}"
# $SSH_CMD "cd $REMOTE_DIR && docker logs caddy 2>&1 | tail -n 20"

# Cleanup
echo -e "${BLUE}Cleanup...${NC}"
# rm -rf ./scripts/english-coaching-image.tar.gz
# rm -rf ./scripts/static-files.tar.gz
docker image prune -f

# Close SSH Control Master connection
echo -e "${BLUE}Closing SSH connection...${NC}"
ssh $SSH_OPTS -p $SSH_PORT -O exit $USERNAME@$SERVER_IP

echo -e "${GREEN}=== DEPLOY COMPLETED SUCCESSFULLY! ===${NC}"
echo -e "${YELLOW}Application is running at:${NC}"
echo -e "${YELLOW}- HTTP: http://${SERVER_IP} (will automatically redirect to HTTPS)${NC}"
echo -e "${YELLOW}- HTTPS: https://${SERVER_IP} (Caddy will automatically obtain SSL certificates)${NC}"
echo -e "${YELLOW}- Domain: https://${DOMAIN} (after configuring DNS to point to ${SERVER_IP})${NC}"
echo -e "${YELLOW}To check status, run: ssh -p $SSH_PORT $USERNAME@$SERVER_IP \"cd ${REMOTE_DIR} && docker-compose ps\"${NC}"
echo -e "${YELLOW}To view logs, run: ssh -p $SSH_PORT $USERNAME@$SERVER_IP \"cd ${REMOTE_DIR} && docker-compose logs\"${NC}"
echo -e "${YELLOW}To view Caddy logs, run: ssh -p $SSH_PORT $USERNAME@$SERVER_IP \"cd ${REMOTE_DIR} && docker-compose logs caddy\"${NC}"