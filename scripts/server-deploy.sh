#!/bin/bash
# Đặt quyền thực thi: chmod +x server-deploy.sh

# Script deploy ứng dụng Axum Chill Teacher
# Sử dụng: ./server-deploy.sh [server_ip] [username] [ssh_port]

# Màu sắc cho output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color


# Read from env or use defaults
SERVER_IP=${SERVER_IP:-${1:-"your_server_ip"}}
USERNAME=${SERVER_USER:-${2:-"root"}}
SSH_PORT=${SSH_PORT:-${3:-"22"}}
DOMAIN=${DOMAIN:-"coaching.chillteacher.com"}


REMOTE_DIR="/opt/english-coaching"

# Đường dẫn tới thư mục gốc của dự án
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FRONTEND_BUILD_DIR="$PROJECT_ROOT/frontend/build"
BACKEND_DIR="$PROJECT_ROOT/backend"


# Check if server_ip is still default
if [ "$SERVER_IP" == "your_server_ip" ]; then
    echo -e "${RED}Please provide the actual server IP address!${NC}"
    echo -e "${YELLOW}Usage: ./scripts/server-deploy.sh [server_ip] [username] [ssh_port]${NC}"
    echo -e "${YELLOW}Or set SERVER_IP, SERVER_USER, SERVER_PASSWORD, DOMAIN in .env${NC}"
    exit 1
fi


# Thiết lập SSH Control Master để tái sử dụng kết nối SSH
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


echo -e "${BLUE}=== SCRIPT DEPLOY AXUM ENGLISH-COACHING ===${NC}"
echo -e "${YELLOW}Server IP: ${SERVER_IP}${NC}"
echo -e "${YELLOW}Username: ${USERNAME}${NC}"
echo -e "${YELLOW}SSH Port: ${SSH_PORT}${NC}"
echo -e "${YELLOW}Domain: ${DOMAIN}${NC}"
echo -e "${YELLOW}Remote Directory: ${REMOTE_DIR}${NC}"
echo ""


# Tạo thư mục trên server
echo -e "${BLUE}Creating directories on server...${NC}"
$SSH_CMD "mkdir -p $REMOTE_DIR && mkdir -p $REMOTE_DIR/static"

if [ $? -ne 0 ]; then
    echo -e "${RED}Failed to create directories on server!${NC}"
    echo -e "${YELLOW}Check SERVER_PASSWORD in .env or install sshpass locally.${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Directories created successfully!${NC}"




# Copy các file cần thiết khác lên server
echo -e "${BLUE}Copy các file lên server...${NC}"
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
    echo -e "${RED}Lỗi khi copy file lên server!${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Copy file thành công!${NC}"






# Hỏi người dùng có build image không
read -p "Bạn có muốn build và copy Docker image lên server không? (y/n): " BUILD_IMAGE
if [[ "$BUILD_IMAGE" =~ ^[Yy]$ ]]; then

    # Step 1: Build Docker image
    echo -e "${BLUE}Build Docker image...${NC}"
    docker build -t english-coaching:latest -f scripts/Dockerfile .

    if [ $? -ne 0 ]; then
        echo -e "${RED}Lỗi khi build Docker image!${NC}"
        exit 1
    fi
    echo -e "${GREEN}✓ Build Docker image thành công!${NC}"

    # Step 2: Save Docker image to file
    echo -e "${BLUE}Save Docker image to file...${NC}"
    cd scripts &&  docker save english-coaching:latest | gzip > english-coaching-image.tar.gz

    if [ $? -ne 0 ]; then
        echo -e "${RED}Lỗi khi lưu Docker image!${NC}"
        exit 1
    fi
    echo -e "${GREEN}✓ Lưu Docker image thành công!${NC}"

    $SCP_CMD english-coaching-image.tar.gz $USERNAME@$SERVER_IP:$REMOTE_DIR/
    if [ $? -ne 0 ]; then
        echo -e "${RED}Lỗi khi copy file Docker image lên server!${NC}"
        exit 1
    fi
    echo -e "${GREEN}✓ Copy Docker image thành công!${NC}"
    cd ..
fi





# Ask user if they want to rebuild static files
read -p "Do you want to build and copy static files to server? (y/n): " BUILD_STATIC
if [[ "$BUILD_STATIC" =~ ^[Yy]$ ]]; then
    echo -e "${BLUE}Building React ...${NC}"
    chmod +x ./scripts/build-react.sh
    ./scripts/build-react.sh

    # Copy static files lên server nếu có
    echo -e "${BLUE}Copy static files lên server...${NC}"
    if [ -d "./frontend/dist" ]; then
        cd ./frontend/dist && tar -czf ../../scripts/static-files.tar.gz .
        cd ../../scripts
        $SCP_CMD static-files.tar.gz $USERNAME@$SERVER_IP:$REMOTE_DIR/
        $SSH_CMD "cd $REMOTE_DIR && tar -xzf static-files.tar.gz -C static/"

        echo -e "${GREEN}✓ Copy static files thành công!${NC}"
    else
        echo -e "${YELLOW}⚠️ Không tìm thấy thư mục static files. Bạn có thể cập nhật sau bằng lệnh 'make update-static'.${NC}"
    fi
fi

# Tạo script cài đặt trên server
echo -e "${BLUE}Setup script cài đặt trên server...${NC}"
$SSH_CMD "chmod +x $REMOTE_DIR/setup-server.sh"

# Chạy script cài đặt trên server
echo -e "${BLUE}Chạy script cài đặt trên server...${NC}"
$SSH_CMD "cd $REMOTE_DIR && ./setup-server.sh"

if [ $? -ne 0 ]; then
    echo -e "${RED}Lỗi khi chạy script cài đặt!${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Cài đặt trên server thành công!${NC}"

# Kiểm tra trạng thái của các container
echo -e "${BLUE}Kiểm tra trạng thái của các container...${NC}"
$SSH_CMD "cd $REMOTE_DIR && docker-compose ps"

    # Check logs of english-coaching container
    echo -e "${BLUE}Check logs of english-coaching container...${NC}"
    $SSH_CMD "cd $REMOTE_DIR && docker logs english-coaching 2>&1 | tail -n 20"

# Kiểm tra logs của container caddy
echo -e "${BLUE}Kiểm tra logs của container caddy...${NC}"
$SSH_CMD "cd $REMOTE_DIR && docker logs caddy 2>&1 | tail -n 20"

# Cleanup
echo -e "${BLUE}Cleanup...${NC}"
# rm -rf ./scripts/english-coaching-image.tar.gz
# rm -rf ./scripts/static-files.tar.gz
docker image prune -f

# Đóng kết nối SSH Control Master
echo -e "${BLUE}Đang đóng kết nối SSH...${NC}"
ssh $SSH_OPTS -p $SSH_PORT -O exit $USERNAME@$SERVER_IP

echo -e "${GREEN}=== DEPLOY HOÀN TẤT THÀNH CÔNG! ===${NC}"
echo -e "${YELLOW}Ứng dụng đang chạy tại:${NC}"
echo -e "${YELLOW}- HTTP: http://${SERVER_IP} (sẽ tự động chuyển hướng sang HTTPS)${NC}"
echo -e "${YELLOW}- HTTPS: https://${SERVER_IP} (Caddy sẽ tự động lấy SSL certificates)${NC}"
echo -e "${YELLOW}- Domain: https://${DOMAIN} (after configuring DNS to point to ${SERVER_IP})${NC}"
echo -e "${YELLOW}To check status, run: ssh -p $SSH_PORT $USERNAME@$SERVER_IP \"cd ${REMOTE_DIR} && docker-compose ps\"${NC}"
echo -e "${YELLOW}To view logs, run: ssh -p $SSH_PORT $USERNAME@$SERVER_IP \"cd ${REMOTE_DIR} && docker-compose logs\"${NC}"
echo -e "${YELLOW}To view Caddy logs, run: ssh -p $SSH_PORT $USERNAME@$SERVER_IP \"cd ${REMOTE_DIR} && docker-compose logs caddy\"${NC}"