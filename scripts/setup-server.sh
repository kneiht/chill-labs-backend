#!/bin/bash
# Set working directory
cd /opt/english-coaching

# Cài đặt Docker nếu chưa có
if ! command -v docker &> /dev/null; then
    echo "Đang cài đặt Docker..."
    # Xác định hệ điều hành
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        OS=$ID
    else
        echo "Không thể xác định hệ điều hành. Đang sử dụng cài đặt mặc định cho Debian."
        OS="debian"
    fi
    
    # Cài đặt các gói cần thiết
    apt-get update
    apt-get install -y apt-transport-https ca-certificates curl gnupg lsb-release software-properties-common
    
    # Xóa các repository Docker cũ nếu có
    rm -f /etc/apt/sources.list.d/docker*.list
    
    # Xóa các repository Docker cũ từ sources.list chính
    if [ -f /etc/apt/sources.list ]; then
        sed -i '/download.docker.com/d' /etc/apt/sources.list
    fi
    
    # Xóa các khóa GPG cũ của Docker
    rm -f /usr/share/keyrings/docker-archive-keyring.gpg
    if command -v apt-key &> /dev/null; then
        apt-key del 0EBFCD88 2>/dev/null || true
    fi
    
    # Thêm Docker GPG key và repository dựa trên hệ điều hành
    if [ "$OS" = "ubuntu" ]; then
        # Cài đặt cho Ubuntu
        curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
        echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null
    else
        # Cài đặt cho Debian và các hệ điều hành khác
        curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
        echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null
    fi
    
    # Cập nhật và cài đặt Docker
    apt-get update -o Acquire::AllowInsecureRepositories=true || true
    apt-get update
    apt-get install -y docker-ce docker-ce-cli containerd.io
    
    # Khởi động và bật Docker
    systemctl enable docker
    systemctl start docker
    
    echo "Docker đã được cài đặt thành công!"
fi

# Cài đặt Docker Compose nếu chưa có
if ! command -v docker-compose &> /dev/null; then
    echo "Đang cài đặt Docker Compose..."
    
    # Xác định hệ điều hành nếu chưa được xác định
    if [ -z "$OS" ]; then
        if [ -f /etc/os-release ]; then
            . /etc/os-release
            OS=$ID
        else
            echo "Không thể xác định hệ điều hành. Đang sử dụng cài đặt mặc định cho Debian."
            OS="debian"
        fi
    fi
    
    # Phương pháp cài đặt Docker Compose dựa trên hệ điều hành
    if [ "$OS" = "ubuntu" ]; then
        # Cài đặt Docker Compose plugin trên Ubuntu
        apt-get update
        apt-get install -y docker-compose-plugin
        
        # Tạo symlink để có thể sử dụng lệnh docker-compose
        if [ ! -f /usr/local/bin/docker-compose ]; then
            ln -s /usr/libexec/docker/cli-plugins/docker-compose /usr/local/bin/docker-compose 2>/dev/null || \
            ln -s /usr/lib/docker/cli-plugins/docker-compose /usr/local/bin/docker-compose 2>/dev/null || \
            echo "Không thể tạo symlink cho docker-compose. Sử dụng phương pháp cài đặt thay thế."
        fi
    else
        # Cài đặt Docker Compose trên Debian
        echo "Đang cài đặt Docker Compose trên Debian..."
        DOCKER_COMPOSE_VERSION=$(curl -s https://api.github.com/repos/docker/compose/releases/latest | grep 'tag_name' | cut -d\" -f4)
        if [ -z "$DOCKER_COMPOSE_VERSION" ]; then
            DOCKER_COMPOSE_VERSION="v2.23.3"  # Phiên bản mặc định nếu không thể lấy từ API
        fi
        curl -L "https://github.com/docker/compose/releases/download/${DOCKER_COMPOSE_VERSION}/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
        chmod +x /usr/local/bin/docker-compose
    fi
    
    # Kiểm tra xem Docker Compose đã được cài đặt thành công chưa
    if ! command -v docker-compose &> /dev/null; then
        echo "Đang sử dụng phương pháp cài đặt thay thế cho Docker Compose..."
        curl -L "https://github.com/docker/compose/releases/download/v2.23.3/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
        chmod +x /usr/local/bin/docker-compose
    fi
    
    echo "Docker Compose đã được cài đặt thành công!"
fi

echo "Thư mục làm việc sau hiện tại: $(pwd)"
echo "Nội dung thư mục:"
ls -la


# Dừng và xóa container cũ
echo "Đang dừng và xóa container cũ..."

docker-compose down

    # Load new Docker image
    echo "Loading new Docker image..."
    docker load < english-coaching-image.tar.gz

# Tạo thư mục cho Caddy và PostgreSQL
echo "Đang tạo thư mục cho Caddy và PostgreSQL..."
mkdir -p ./caddy_data
mkdir -p ./caddy_config

# Khởi động ứng dụng với Docker Compose
echo "Đang khởi động ứng dụng..."
docker-compose up -d

# Thông báo về SSL
echo "Caddy sẽ tự động lấy và gia hạn SSL certificates..."

# Kiểm tra trạng thái của các container
echo "Kiểm tra trạng thái của các container..."
docker-compose ps

    # Check PostgreSQL connection
    echo "Checking PostgreSQL connection..."
    sleep 5 # Wait for PostgreSQL to start
    docker exec english-coaching-postgres pg_isready -U postgres
if [ $? -ne 0 ]; then
    echo "Lỗi kết nối đến PostgreSQL!"
else
    echo "Kết nối PostgreSQL thành công!"
fi

    # Check logs of english-coaching container
    echo "Checking logs of english-coaching container..."
    docker logs english-coaching

# Dọn dẹp các image không sử dụng (dangling)
echo "Đang dọn dẹp các image cũ..."
docker image prune -f

# Dọn dẹp các volume không sử dụng
echo "Đang dọn dẹp các volume không sử dụng..."
docker volume prune -f

# Dọn dẹp các repository cũ của Docker
echo "Đang dọn dẹp các repository cũ của Docker..."
rm -f /etc/apt/sources.list.d/docker*.list
if [ -f /etc/apt/sources.list ]; then
    sed -i '/download.docker.com/d' /etc/apt/sources.list
fi

echo "Cài đặt hoàn tất!"