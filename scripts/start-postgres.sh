#!/bin/bash

# Thiết lập màu sắc cho output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# --- Configuration ---
CONTAINER_NAME="chillteacher-postgres"
TEST_CONTAINER_NAME="chillteacher-postgres-test"
VOLUME_NAME="chillteacher_postgres_data"
TEST_VOLUME_NAME="chillteacher_postgres_data_test"
BACKUP_DIR="./backend/database/backups" # Define a directory for backups

# --- Script Mode ---
SCRIPT_MODE="development" # Default to development

if [[ "$1" == "--test" ]]; then
    SCRIPT_MODE="test"
    echo -e "${YELLOW}Running in TEST mode.${NC}"
    CURRENT_CONTAINER_NAME="$TEST_CONTAINER_NAME"
    CURRENT_VOLUME_NAME="$TEST_VOLUME_NAME"
    DB_ENV_VAR_NAME="POSTGRES_DB_TEST"
    PORT_ENV_VAR_NAME="POSTGRES_PORT_TEST" # Use test port variable
else
    echo -e "${BLUE}Running in DEVELOPMENT mode.${NC}"
    CURRENT_CONTAINER_NAME="$CONTAINER_NAME"
    CURRENT_VOLUME_NAME="$VOLUME_NAME"
    DB_ENV_VAR_NAME="POSTGRES_DB"
    PORT_ENV_VAR_NAME="POSTGRES_PORT" # Use development port variable
fi

# Load the correct DB name and Port based on mode
eval "CURRENT_POSTGRES_DB=\"\${$DB_ENV_VAR_NAME}\""
eval "CURRENT_POSTGRES_PORT=\"\${$PORT_ENV_VAR_NAME}\""


echo -e "${BLUE}=== Khởi động PostgreSQL Container (${SCRIPT_MODE}) ===${NC}"

# Kiểm tra xem container (đang chạy hoặc đã dừng) có tồn tại không
EXISTING_CONTAINER_ID=$(docker ps -a -q -f name="^${CURRENT_CONTAINER_NAME}$")

if [ -n "$EXISTING_CONTAINER_ID" ]; then
    # Kiểm tra xem container có đang chạy không
    IS_RUNNING=$(docker ps -q -f id="$EXISTING_CONTAINER_ID")

    if [ -n "$IS_RUNNING" ]; then
        echo -e "${YELLOW}Container '$CURRENT_CONTAINER_NAME' (ID: $EXISTING_CONTAINER_ID) đang chạy.${NC}"
        read -p "$(echo -e "${YELLOW}Bạn có muốn giữ container đang chạy này không (mặc định: yes)? (y/n): ${NC}")" KEEP_RUNNING

        if [[ "$KEEP_RUNNING" =~ ^[Nn]$ ]]; then
            echo -e "${YELLOW}Đang dừng container đang chạy (ID: $EXISTING_CONTAINER_ID)...${NC}"
            docker stop "$EXISTING_CONTAINER_ID" >/dev/null || echo -e "${YELLOW}Container đã dừng hoặc không thể dừng.${NC}"

        else
            echo -e "${GREEN}Giữ container đang chạy.${NC}"
            exit 0
        fi
    else
        echo -e "${YELLOW}Container '$CURRENT_CONTAINER_NAME' (ID: $EXISTING_CONTAINER_ID) đã tồn tại nhưng đã dừng.${NC}"
    fi

    echo -e "${YELLOW}Đang xóa container cũ (ID: $EXISTING_CONTAINER_ID)...${NC}"
    docker rm "$EXISTING_CONTAINER_ID" >/dev/null || echo -e "${YELLOW}Container đã được xóa hoặc không thể xóa.${NC}"
    echo -e "${GREEN}✓ Dọn dẹp container cũ hoàn tất.${NC}"
fi

# Khởi động PostgreSQL container mới
echo -e "${YELLOW}Đang khởi động container PostgreSQL mới...${NC}"

# Check if required variables were loaded successfully (using parameter expansion)
if [ -z "$POSTGRES_USER" ] || \
   [ -z "$POSTGRES_PASSWORD" ] || \
   [ -z "$CURRENT_POSTGRES_DB" ] || \
   [ -z "$CURRENT_POSTGRES_PORT" ]; then # Check CURRENT_POSTGRES_PORT
    echo -e "${RED}Lỗi: Một hoặc nhiều biến database (POSTGRES_USER, POSTGRES_PASSWORD, $DB_ENV_VAR_NAME, $PORT_ENV_VAR_NAME) không được tìm thấy trong .env hoặc không được đặt.${NC}"
    if [ "$SCRIPT_MODE" == "test" ] && [ -z "$POSTGRES_DB_TEST" ]; then
        echo -e "${RED}Cụ thể, biến POSTGRES_DB_TEST cho chế độ test chưa được đặt.${NC}"
    elif [ "$SCRIPT_MODE" == "development" ] && [ -z "$POSTGRES_DB" ]; then
        echo -e "${RED}Cụ thể, biến POSTGRES_DB cho chế độ development chưa được đặt.${NC}"
    fi
    # Add specific check for port variable
    [ -z "$CURRENT_POSTGRES_PORT" ] && echo -e "${RED}Cụ thể, biến port $PORT_ENV_VAR_NAME cho chế độ $SCRIPT_MODE chưa được đặt.${NC}"
    exit 1
fi

docker run --name "$CURRENT_CONTAINER_NAME" \
  -e POSTGRES_USER="${POSTGRES_USER?Biến POSTGRES_USER chưa được đặt trong .env}" \
  -e POSTGRES_PASSWORD="${POSTGRES_PASSWORD?Biến POSTGRES_PASSWORD chưa được đặt trong .env}" \
  -e POSTGRES_DB="${CURRENT_POSTGRES_DB}" \
  -p "${CURRENT_POSTGRES_PORT}:5432" \
  -v "$CURRENT_VOLUME_NAME":/var/lib/postgresql/data \
  -d postgres:16-alpine

# Kiểm tra xem container đã khởi động thành công chưa
if [ $? -ne 0 ]; then
    echo -e "${RED}Lỗi: Không thể khởi động container PostgreSQL!${NC}"
    exit 1
fi

echo -e "${GREEN}✓ PostgreSQL đã được khởi động trên cổng $CURRENT_POSTGRES_PORT.${NC}"
echo -e "   Database: $CURRENT_POSTGRES_DB"
echo -e "   Kết nối: postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost:$CURRENT_POSTGRES_PORT/$CURRENT_POSTGRES_DB"
echo -e "   Volume dữ liệu: $CURRENT_VOLUME_NAME"

# Đợi một chút để PostgreSQL sẵn sàng
echo -e "${YELLOW}Đang đợi PostgreSQL sẵn sàng...${NC}"
sleep 5 # Adjust sleep time if needed

# Hỏi người dùng có muốn tạo backup không
read -p "$(echo -e "${YELLOW}Bạn có muốn tạo backup database hiện tại không? (y/n): ${NC}")" CREATE_BACKUP

if [[ "$CREATE_BACKUP" =~ ^[Yy]$ ]]; then
    echo -e "${BLUE}Đang tạo backup database...${NC}"

    # Tạo thư mục backup nếu chưa tồn tại
    mkdir -p "$BACKUP_DIR"

    # Tạo tên file backup với timestamp
    BACKUP_FILENAME="$BACKUP_DIR/backup_${CURRENT_POSTGRES_DB}_$(date +%Y%m%d_%H%M%S).sql"

    # Thực hiện backup bằng pg_dump qua docker exec
    docker exec "$CURRENT_CONTAINER_NAME" pg_dump -U "$POSTGRES_USER" -d "$CURRENT_POSTGRES_DB" > "$BACKUP_FILENAME"

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Backup database thành công! Lưu tại: $BACKUP_FILENAME${NC}"
    else
        echo -e "${RED}Lỗi: Không thể tạo backup database!${NC}"
        # Xóa file backup rỗng hoặc lỗi nếu có
        rm -f "$BACKUP_FILENAME"
        # Optionally exit if backup fails and is critical
        # exit 1
    fi
else
    echo -e "${BLUE}Bỏ qua việc tạo backup.${NC}"
fi

# Hỏi người dùng có muốn xóa tất cả các bảng không
read -p "$(echo -e "${YELLOW}Bạn có muốn XÓA TẤT CẢ các bảng trong database '$CURRENT_POSTGRES_DB' không? (y/n): ${NC}")" DROP_TABLES

if [[ "$DROP_TABLES" =~ ^[Yy]$ ]]; then
    echo -e "${RED}Đang xóa tất cả các bảng trong database '$CURRENT_POSTGRES_DB'...${NC}"

    # Lệnh SQL để tạo các lệnh DROP TABLE
    # Lưu ý: Điều này sẽ xóa tất cả các bảng trong schema 'public'. Hãy cẩn thận!
    DROP_COMMAND=$(cat <<-EOF
SELECT 'DROP TABLE IF EXISTS "' || tablename || '" CASCADE;'
FROM pg_tables
WHERE schemaname = 'public';
EOF
)

    # Thực thi các lệnh DROP TABLE bằng psql qua docker exec
    docker exec "$CURRENT_CONTAINER_NAME" psql -U "$POSTGRES_USER" -d "$CURRENT_POSTGRES_DB" -t -c "$DROP_COMMAND" | \
    docker exec -i "$CURRENT_CONTAINER_NAME" psql -U "$POSTGRES_USER" -d "$CURRENT_POSTGRES_DB" -q

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Đã xóa thành công tất cả các bảng.${NC}"
    else
        echo -e "${RED}Lỗi: Không thể xóa các bảng trong database!${NC}"
        # Optionally exit if dropping tables fails and is critical
        # exit 1
    fi
else
    echo -e "${BLUE}Bỏ qua việc xóa các bảng.${NC}"
fi


echo -e "${GREEN}=== Hoàn tất ===${NC}"
