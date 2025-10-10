#!/bin/bash

# --- Configuration ---
# Source .env if it exists and export variables
if [ -f ".env" ]; then
    echo -e "\033[0;36mLoading environment variables from .env file...\033[0m"
    set -o allexport
    # shellcheck disable=SC1091
    source .env
    set +o allexport
else
    echo -e "\033[1;33mWarning: .env file not found. Some commands might not work as expected.\033[0m"
fi

# Warning: make sure to set the correct values in .env
echo -e "\033[1;33mWarning: Please ensure all variables are set in .env file.\033[0m"

read -p "Have you set all variables in .env file? (y/n): " ENV_VARS_SET
if [[ "$ENV_VARS_SET" =~ ^[Yy]$ ]]; then
    echo -e "\033[1;32mAll variables are set in .env file.\033[0m"
    echo -e "\033[1;32mContinuing...\033[0m"
else
    echo -e "\033[1;31mError: Please ensure all variables are set in .env file.\033[0m"
    exit 1
fi

# Directories (from env or defaults)
FRONTEND_DIR=${FRONTEND_DIR}
BACKEND_DIR=${BACKEND_DIR}
SCRIPTS_DIR=${SCRIPTS_DIR}
DATABASE_DIR=${DATABASE_DIR}


# Atlas default environment
ATLAS_ENV_DEFAULT="dev"
ATLAS_ENV_CURRENT="$ATLAS_ENV_DEFAULT" # Will be updated by ask_atlas_env

# Colors
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# --- Helper Functions ---
print_header() {
    echo -e "${BLUE}=== App Backend CLI ===${NC}"
    echo
}

ask_atlas_env() {
    read -p "Enter Atlas environment (default: $ATLAS_ENV_DEFAULT): " atlas_env_input
    ATLAS_ENV_CURRENT="${atlas_env_input:-$ATLAS_ENV_DEFAULT}"
    echo -e "${CYAN}Using Atlas environment: $ATLAS_ENV_CURRENT${NC}"
}

_ensure_env_var() {
    local var_name="$1"
    local var_value="${!var_name}" # Indirect expansion
    if [ -z "$var_value" ]; then
        echo -e "${RED}Error: Environment variable $var_name is not set.${NC}"
        echo -e "${YELLOW}Please set it in your .env file or current environment.${NC}"
        return 1
    fi
    return 0
}

# --- Setup ---
setup_scripts() {
    echo -e "${BLUE}Making scripts executable...${NC}"
    chmod +x "$SCRIPTS_DIR"/*.sh
    echo -e "${GREEN}✓ Scripts are now executable${NC}"
}

# --- Backend ---
_prompt_apply_migrations_after_db_start() {
    local db_context="$1" # "dev" or "test"
    echo -en "${YELLOW}Apply database migrations now? (y/N): ${NC}"
    read -r answer
    if [[ "$answer" == "y" || "$answer" == "Y" ]]; then
        echo -e "${BLUE}Applying migrations...${NC}"
        if [[ "$db_context" == "test" ]]; then
            db_apply_test # For test DB
        else
            db_apply # For dev DB
        fi
    else
        echo -e "${YELLOW}Skipping migrations.${NC}"
    fi
}

backend_start_db_base() {
    local mode="$1" # "dev" or "test"
    setup_scripts
    echo -e "${BLUE}Starting PostgreSQL container for ${mode} environment...${NC}"
    if [[ "$mode" == "test" ]]; then
        "$SCRIPTS_DIR/start-postgres.sh" --test
    else
        "$SCRIPTS_DIR/start-postgres.sh"
    fi
    _prompt_apply_migrations_after_db_start "$mode"
}

backend_start_db() {
    backend_start_db_base "dev"
}

backend_start_db_test() {
    backend_start_db_base "test"
}

backend_serve() {
    # Dependency: backend_start_db (implicitly handled by user flow or separate call)
    echo -e "${MAGENTA}Note: Ensure the development database is running. Use 'Start DB (dev)' if needed.${NC}"
    echo -e "${BLUE}Starting Axum server (single run)...${NC}"
    (cd "$BACKEND_DIR" && cargo run --bin backend)
}

backend_watch() {
    tools_install_watch # Dependency
    # Dependency: backend_start_db (implicitly handled by user flow or separate call)
    echo -e "${MAGENTA}Note: Ensure the development database is running. Use 'Start DB (dev)' if needed.${NC}"
    echo -e "${BLUE}Starting Axum server with cargo watch...${NC}"
    (cd "$BACKEND_DIR" && cargo watch -c -q -x "run --bin backend")
}

backend_test() {
    # Dependencies: backend_start_db_test, db_apply_test
    echo -e "${MAGENTA}Note: Ensure the test database is running and migrations are applied.${NC}"
    echo -e "${MAGENTA}Use 'Start DB (test)' (which includes migration prompt) if needed.${NC}"
    echo -e "${BLUE}Starting backend tests...${NC}"
    (cd "$BACKEND_DIR" && cargo test)
}


backend_sqlx_prepare() {
    echo -e "${CYAN}Preparing SQLx for offline mode...${NC}"
    (cd "$BACKEND_DIR" && cargo sqlx prepare)
    echo -e "${GREEN}✓ SQLx prepared successfully${NC}"
}


# --- Database Migrations (Atlas & SQLx) ---
db_atlas_hash() {
    ask_atlas_env
    echo -e "${CYAN}Generating Atlas migration hash (env: $ATLAS_ENV_CURRENT)...${NC}"
    (cd "$DATABASE_DIR" && atlas --env "$ATLAS_ENV_CURRENT" migrate hash)
    echo -e "${GREEN}✓ Atlas hash generated successfully${NC}"
}

db_atlas_schema() {
    ask_atlas_env
    echo -e "${CYAN}Generating schema.sql from DB state (env: $ATLAS_ENV_CURRENT)...${NC}"
    (cd "$DATABASE_DIR" && atlas --env "$ATLAS_ENV_CURRENT" schema inspect > schema.sql)
    echo -e "${GREEN}✓ schema.sql generated successfully${NC}"
}

db_atlas_plan() {
    ask_atlas_env
    read -p "Enter migration name: " migration_name
    if [ -z "$migration_name" ]; then
        echo -e "${RED}Error: Missing 'name' argument.${NC}"
        echo -e "${YELLOW}Usage: Enter a name for your migration.${NC}"
        return 1
    fi
    echo -e "${CYAN}Generating Atlas migration diff named '$migration_name' (env: $ATLAS_ENV_CURRENT)...${NC}"
    (cd "$DATABASE_DIR" && atlas --env "$ATLAS_ENV_CURRENT" migrate diff "$migration_name" --format '{{ sql . "  " }}')
    echo -e "${GREEN}✓ Atlas migration '$migration_name' generated successfully${NC}"
}

db_apply() {
    _ensure_env_var "DATABASE_URL" || return 1
    echo -e "${CYAN}Applying pending migrations to DEV database ($DATABASE_URL)...${NC}"
    (cd "$DATABASE_DIR" && sqlx migrate run --database-url "$DATABASE_URL")
    echo -e "${GREEN}✓ Migrations applied successfully to DEV database${NC}"
}

db_apply_test() {
    _ensure_env_var "DATABASE_URL_TEST" || return 1
    echo -e "${CYAN}Applying pending migrations to TEST database ($DATABASE_URL_TEST)...${NC}"
    (cd "$DATABASE_DIR" && sqlx migrate run --database-url "$DATABASE_URL_TEST")
    echo -e "${GREEN}✓ Migrations applied successfully to TEST database${NC}"
}

db_atlas_status() {
    ask_atlas_env
    echo -e "${CYAN}Checking Atlas migration status (env: $ATLAS_ENV_CURRENT)...${NC}"
    (cd "$DATABASE_DIR" && atlas --env "$ATLAS_ENV_CURRENT" migrate status)
    echo -e "${GREEN}✓ Atlas migration status check complete${NC}"
}

# --- Deployment ---
_get_deploy_params() {
    # Use existing environment variables if set, otherwise prompt with defaults
    read -p "Enter server IP address (default: ${SERVER_IP}): " SERVER_IP_INPUT
    SERVER_IP="${SERVER_IP_INPUT:-${SERVER_IP}}"
    read -p "Enter SSH username (default: ${DEPLOY_USER:-root}): " USERNAME_INPUT
    DEPLOY_USER="${USERNAME_INPUT:-${DEPLOY_USER:-root}}"
    read -p "Enter SSH port (default: ${SSH_DEPLOY_PORT:-22}): " SSH_PORT_INPUT
    SSH_DEPLOY_PORT="${SSH_PORT_INPUT:-${SSH_DEPLOY_PORT:-22}}"
}

deploy_generate_key() {
    echo -e "${CYAN}Generating SSH key...${NC}"
    if [ -f ~/.ssh/id_rsa ]; then
        echo -e "${YELLOW}SSH key already exists at ~/.ssh/id_rsa${NC}"
    else
        ssh-keygen -t rsa -b 4096
        echo -e "${GREEN}✅ SSH key generated successfully!${NC}"
    fi
}

deploy_copy_key() {
    echo -e "${CYAN}Copying SSH key to server...${NC}"
    read -p "Enter server IP address for key copy: " SERVER_IP_COPY
    read -p "Enter SSH username for key copy (default: root): " USER_COPY
    USER_COPY="${USER_COPY:-root}"
    read -p "Enter SSH port for key copy (default: 22): " SSH_PORT_COPY
    SSH_PORT_COPY="${SSH_PORT_COPY:-22}"
    echo -e "${YELLOW}Copying key to ${USER_COPY}@${SERVER_IP_COPY}:${SSH_PORT_COPY}...${NC}"
    ssh-copy-id -p "$SSH_PORT_COPY" "${USER_COPY}@${SERVER_IP_COPY}"
    echo -e "${GREEN}✅ SSH key copied successfully!${NC}"
}

deploy_test_conn() {
    echo -e "${CYAN}Testing SSH connection...${NC}"
    read -p "Enter server IP address for test: " SERVER_IP_TEST_CONN
    read -p "Enter SSH username for test (default: root): " USER_TEST_CONN
    USER_TEST_CONN="${USER_TEST_CONN:-root}"
    read -p "Enter SSH port for test (default: 22): " SSH_PORT_TEST_CONN
    SSH_PORT_TEST_CONN="${SSH_PORT_TEST_CONN:-22}"
    echo -e "${YELLOW}Testing connection to ${USER_TEST_CONN}@${SERVER_IP_TEST_CONN}:${SSH_PORT_TEST_CONN}...${NC}"
    ssh -p "$SSH_PORT_TEST_CONN" "${USER_TEST_CONN}@${SERVER_IP_TEST_CONN}" "echo -e '${GREEN}✅ SSH connection successful!${NC}'"
}

deploy_app() {
    setup_scripts # Dependency
    echo -e "${CYAN}Deploying application to server...${NC}"
    _get_deploy_params
    echo -e "${YELLOW}Deploying to ${DEPLOY_USER}@${SERVER_IP}:${SSH_DEPLOY_PORT}...${NC}"
    "$SCRIPTS_DIR/server-deploy.sh" "$SERVER_IP" "$DEPLOY_USER" "$SSH_DEPLOY_PORT"
    echo -e "${GREEN}✅ Application deployment complete!${NC}"
}

deploy_setup() {
    deploy_generate_key
    deploy_copy_key
    deploy_test_conn
    deploy_app
    echo -e "${BOLD}${GREEN}=== Full deployment setup complete! ===${NC}"
}

# --- Tools & Cleanup ---
tools_install_sweep() {
    echo -e "${CYAN}Checking/Installing cargo-sweep...${NC}"
    if ! command -v cargo-sweep &> /dev/null; then
        echo "cargo-sweep not found. Installing..."
        cargo install cargo-sweep
        echo -e "${GREEN}✓ cargo-sweep installed successfully${NC}"
    else
        echo -e "${GREEN}✓ cargo-sweep is already installed${NC}"
    fi
}

tools_install_watch() {
    echo -e "${CYAN}Checking/Installing cargo-watch...${NC}"
    if ! command -v cargo-watch &> /dev/null; then
        echo "cargo-watch not found. Installing..."
        cargo install cargo-watch
        echo -e "${GREEN}✓ cargo-watch installed successfully${NC}"
    else
        echo -e "${GREEN}✓ cargo-watch is already installed${NC}"
    fi
}

tools_clean_cargo() {
    tools_install_sweep # Dependency
    echo -e "${CYAN}Sweeping unused Rust build artifacts...${NC}"
    (cd "$BACKEND_DIR" && cargo sweep --installed)
    echo -e "${GREEN}✓ Cargo sweep complete${NC}"
}

_clean_action() {
    local force_clean_node_modules=false
    if [[ "$1" == "--force" ]]; then
        force_clean_node_modules=true
    fi

    echo -e "${BLUE}Cleaning backend build artifacts...${NC}"
    (cd "$BACKEND_DIR" && cargo clean)
    echo -e "${GREEN}✓ Backend target directory cleaned${NC}"
}

clean_normal() {
    _clean_action
}

clean_force() {
    _clean_action --force
}

# --- Menu Functions ---
show_backend_menu() {
    while true; do
        echo -e "\n${BOLD}${CYAN}--- Backend (Rust/Axum) ---${NC}"
        local options=(
            "Serve (single run)"
            "Watch (dev mode with auto-reload)"
            "Test backend"
            "Start DB (dev) & Apply Migrations"
            "Start DB (test) & Apply Migrations"
            "SQLx Prepare (for offline mode)"
            "Back to Main Menu"
        )
        COLUMNS=1 # Ensure select options are listed vertically
        PS3="Backend action? "
        select opt in "${options[@]}"; do
            case $REPLY in
                1) backend_serve; break ;;
                2) backend_watch; break ;;
                3) backend_test; break ;;
                4) backend_start_db; break ;;
                5) backend_start_db_test; break ;;
                7) backend_sqlx_prepare; break ;;
                $((${#options[@]}))) return ;;
                *) echo -e "${RED}Invalid option $REPLY${NC}" ;;
            esac
        done
    done
}


show_db_menu() {
    while true; do
        echo -e "\n${BOLD}${CYAN}--- Database (Atlas & SQLx) ---${NC}"
        local options=(
            "Atlas: Generate migration hash"
            "Atlas: Generate schema.sql from DB state"
            "Atlas: Generate new migration (plan/diff)"
            "SQLx: Apply pending migrations (DEV DB)"
            "SQLx: Apply pending migrations (TEST DB)"
            "Atlas: Show migration status"
            "Back to Main Menu"
        )
        COLUMNS=1
        PS3="Database action? "
        select opt in "${options[@]}"; do
            case $REPLY in
                1) db_atlas_hash; break ;;
                2) db_atlas_schema; break ;;
                3) db_atlas_plan; break ;;
                4) db_apply; break ;;
                5) db_apply_test; break ;;
                6) db_atlas_status; break ;;
                $((${#options[@]}))) return ;;
                *) echo -e "${RED}Invalid option $REPLY${NC}" ;;
            esac
        done
    done
}

show_deploy_menu() {
    while true; do
        echo -e "\n${BOLD}${CYAN}--- Deployment ---${NC}"
        local options=(
            "Generate SSH key (if needed)"
            "Copy SSH public key to server"
            "Test SSH connection to server"
            "Deploy backend application"
            "Deploy static frontend files"
            "Run full deployment setup (key, copy, test, app)"
            "Back to Main Menu"
        )
        COLUMNS=1
        PS3="Deployment action? "
        select opt in "${options[@]}"; do
            case $REPLY in
                1) deploy_generate_key; break ;;
                2) deploy_copy_key; break ;;
                3) deploy_test_conn; break ;;
                4) deploy_app; break ;;
                5) deploy_static; break ;;
                6) deploy_setup; break ;;
                $((${#options[@]}))) return ;;
                *) echo -e "${RED}Invalid option $REPLY${NC}" ;;
            esac
        done
    done
}

show_tools_menu() {
    while true; do
        echo -e "\n${BOLD}${CYAN}--- Tools & Cleanup ---${NC}"
        local options=(
            "Install/Check cargo-sweep"
            "Install/Check cargo-watch"
            "Clean unused Rust build artifacts (cargo-sweep)"
            "Clean (backend + optional frontend node_modules)"
            "Clean Force (backend + frontend node_modules)"
            "Back to Main Menu"
        )
        COLUMNS=1
        PS3="Tool/Cleanup action? "
        select opt in "${options[@]}"; do
            case $REPLY in
                1) tools_install_sweep; break ;;
                2) tools_install_watch; break ;;
                3) tools_clean_cargo; break ;;
                4) clean_normal; break ;;
                5) clean_force; break ;;
                $((${#options[@]}))) return ;;
                *) echo -e "${RED}Invalid option $REPLY${NC}" ;;
            esac
        done
    done
}

# --- Main Menu ---
main_menu() {
    while true; do
        print_header
        echo -e "${BOLD}${MAGENTA}Select a category:${NC}"
        COLUMNS=1 # Ensure select options are listed vertically for the main menu as well
        PS3="Enter your choice (or q to quit): "
        local main_options=(
            "Setup Scripts (make executable)"
            "Backend (Rust/Axum)"
            "Database (Atlas & SQLx)"
            "Deployment"
            "Tools & Cleanup"
            "Quit"
        )
        select opt in "${main_options[@]}"; do
            # Check if REPLY is 'q' or 'Q' for quitting, in addition to the numbered option
            if [[ "$REPLY" == "q" || "$REPLY" == "Q" ]]; then
                echo -e "${GREEN}Exiting EnglishCoaching CLI. Goodbye!${NC}"; exit 0
            fi

            case $REPLY in
                1) setup_scripts; break ;;
                2) show_backend_menu; break ;;
                4) show_db_menu; break ;;
                5) show_deploy_menu; break ;;
                6) show_tools_menu; break ;;
                $((${#main_options[@]}))) echo -e "${GREEN}Exiting EnglishCoaching CLI. Goodbye!${NC}"; exit 0 ;;
                *) echo -e "${RED}Invalid option $REPLY. Type 'q' or the number of 'Quit' to exit.${NC}" ;;
            esac
        done
    done
}

# --- Entry Point ---
# Start the main menu
main_menu