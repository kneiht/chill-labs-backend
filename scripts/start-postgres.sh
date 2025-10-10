#!/bin/bash

# Source .env if it exists
if [ -f ".env" ]; then
    set -o allexport
    source .env
    set +o allexport
fi

# Set colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# --- Configuration ---
CONTAINER_NAME="${APP_CONTAINER_NAME}-postgres"
TEST_CONTAINER_NAME="${APP_CONTAINER_NAME}-postgres-test"
VOLUME_NAME="${APP_CONTAINER_NAME}_postgres_data"
TEST_VOLUME_NAME="${APP_CONTAINER_NAME}_postgres_data_test"
BACKUP_DIR="$BACKEND_DIR/database/backups" # Define a directory for backups

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


echo -e "${BLUE}=== Starting PostgreSQL Container (${SCRIPT_MODE}) ===${NC}"

# Check if container (running or stopped) exists
EXISTING_CONTAINER_ID=$(docker ps -a -q -f name="^${CURRENT_CONTAINER_NAME}$")

if [ -n "$EXISTING_CONTAINER_ID" ]; then
    # Check if container is running
    IS_RUNNING=$(docker ps -q -f id="$EXISTING_CONTAINER_ID")

    if [ -n "$IS_RUNNING" ]; then
        echo -e "${YELLOW}Container '$CURRENT_CONTAINER_NAME' (ID: $EXISTING_CONTAINER_ID) is running.${NC}"
        read -p "$(echo -e "${YELLOW}Do you want to keep this running container (default: yes)? (y/n): ${NC}")" KEEP_RUNNING

        if [[ "$KEEP_RUNNING" =~ ^[Nn]$ ]]; then
            echo -e "${YELLOW}Stopping running container (ID: $EXISTING_CONTAINER_ID)...${NC}"
            docker stop "$EXISTING_CONTAINER_ID" >/dev/null || echo -e "${YELLOW}Container stopped or could not be stopped.${NC}"

        else
            echo -e "${GREEN}Keeping running container.${NC}"
            exit 0
        fi
    else
        echo -e "${YELLOW}Container '$CURRENT_CONTAINER_NAME' (ID: $EXISTING_CONTAINER_ID) exists but is stopped.${NC}"
    fi

    echo -e "${YELLOW}Removing old container (ID: $EXISTING_CONTAINER_ID)...${NC}"
    docker rm "$EXISTING_CONTAINER_ID" >/dev/null || echo -e "${YELLOW}Container removed or could not be removed.${NC}"
    echo -e "${GREEN}✓ Old container cleanup complete.${NC}"
fi

# Start new PostgreSQL container
echo -e "${YELLOW}Starting new PostgreSQL container...${NC}"

# Check if required variables were loaded successfully (using parameter expansion)
if [ -z "$POSTGRES_USER" ] || \
   [ -z "$POSTGRES_PASSWORD" ] || \
   [ -z "$CURRENT_POSTGRES_DB" ] || \
   [ -z "$CURRENT_POSTGRES_PORT" ]; then # Check CURRENT_POSTGRES_PORT
    echo -e "${RED}Error: One or more database variables (POSTGRES_USER, POSTGRES_PASSWORD, $DB_ENV_VAR_NAME, $PORT_ENV_VAR_NAME) not found in .env or not set.${NC}"
    if [ "$SCRIPT_MODE" == "test" ] && [ -z "$POSTGRES_DB_TEST" ]; then
        echo -e "${RED}Specifically, POSTGRES_DB_TEST variable for test mode not set.${NC}"
    elif [ "$SCRIPT_MODE" == "development" ] && [ -z "$POSTGRES_DB" ]; then
        echo -e "${RED}Specifically, POSTGRES_DB variable for development mode not set.${NC}"
    fi
    # Add specific check for port variable
    [ -z "$CURRENT_POSTGRES_PORT" ] && echo -e "${RED}Specifically, port variable $PORT_ENV_VAR_NAME for $SCRIPT_MODE not set.${NC}"
    exit 1
fi

docker run --name "$CURRENT_CONTAINER_NAME" \
  -e POSTGRES_USER="${POSTGRES_USER?POSTGRES_USER variable not set in .env}" \
  -e POSTGRES_PASSWORD="${POSTGRES_PASSWORD?POSTGRES_PASSWORD variable not set in .env}" \
  -e POSTGRES_DB="${CURRENT_POSTGRES_DB}" \
  -p "${CURRENT_POSTGRES_PORT}:5432" \
  -v "$CURRENT_VOLUME_NAME":/var/lib/postgresql/data \
  -d postgres:16-alpine

# Check if container started successfully
if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Could not start PostgreSQL container!${NC}"
    exit 1
fi

echo -e "${GREEN}✓ PostgreSQL started on port $CURRENT_POSTGRES_PORT.${NC}"
echo -e "   Database: $CURRENT_POSTGRES_DB"
echo -e "   Connection: postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost:$CURRENT_POSTGRES_PORT/$CURRENT_POSTGRES_DB"
echo -e "   Data volume: $CURRENT_VOLUME_NAME"

# Wait a bit for PostgreSQL to be ready
echo -e "${YELLOW}Waiting for PostgreSQL to be ready...${NC}"
sleep 5 # Adjust sleep time if needed

# Ask user if they want to create backup
read -p "$(echo -e "${YELLOW}Do you want to create a backup of the current database? (y/n): ${NC}")" CREATE_BACKUP

if [[ "$CREATE_BACKUP" =~ ^[Yy]$ ]]; then
    echo -e "${BLUE}Creating database backup...${NC}"

    # Create backup directory if it doesn't exist
    mkdir -p "$BACKUP_DIR"

    # Create backup filename with timestamp
    BACKUP_FILENAME="$BACKUP_DIR/backup_${CURRENT_POSTGRES_DB}_$(date +%Y%m%d_%H%M%S).sql"

    # Perform backup using pg_dump via docker exec
    docker exec "$CURRENT_CONTAINER_NAME" pg_dump -U "$POSTGRES_USER" -d "$CURRENT_POSTGRES_DB" > "$BACKUP_FILENAME"

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Database backup successful! Saved at: $BACKUP_FILENAME${NC}"
    else
        echo -e "${RED}Error: Could not create database backup!${NC}"
        # Remove empty or error backup file if any
        rm -f "$BACKUP_FILENAME"
        # Optionally exit if backup fails and is critical
        # exit 1
    fi
else
    echo -e "${BLUE}Skipping backup creation.${NC}"
fi

# Ask user if they want to delete all tables
read -p "$(echo -e "${YELLOW}Do you want to DELETE ALL tables in database '$CURRENT_POSTGRES_DB'? (y/n): ${NC}")" DROP_TABLES

if [[ "$DROP_TABLES" =~ ^[Yy]$ ]]; then
    echo -e "${RED}Deleting all tables in database '$CURRENT_POSTGRES_DB'...${NC}"

    # SQL command to generate DROP TABLE commands
    # Note: This will delete all tables in the 'public' schema. Be careful!
    DROP_COMMAND=$(cat <<-EOF
SELECT 'DROP TABLE IF EXISTS "' || tablename || '" CASCADE;'
FROM pg_tables
WHERE schemaname = 'public';
EOF
)

    # Execute DROP TABLE commands using psql via docker exec
    docker exec "$CURRENT_CONTAINER_NAME" psql -U "$POSTGRES_USER" -d "$CURRENT_POSTGRES_DB" -t -c "$DROP_COMMAND" | \
    docker exec -i "$CURRENT_CONTAINER_NAME" psql -U "$POSTGRES_USER" -d "$CURRENT_POSTGRES_DB" -q

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Successfully deleted all tables.${NC}"
    else
        echo -e "${RED}Error: Could not delete tables in database!${NC}"
        # Optionally exit if dropping tables fails and is critical
        # exit 1
    fi
else
    echo -e "${BLUE}Skipping table deletion.${NC}"
fi


echo -e "${GREEN}=== Complete ===${NC}"
