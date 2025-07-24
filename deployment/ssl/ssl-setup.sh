#!/bin/bash

# Marco 2.0 SSL/TLS Automation Script
# Automated certificate management and security setup

set -e

# Configuration
DOMAIN="marco2.com"
SUBDOMAINS="www.marco2.com api.marco2.com ws.marco2.com"
EMAIL="admin@marco2.com"
WEBROOT="/var/www/certbot"
CERT_PATH="/etc/nginx/ssl"
STAGING=${STAGING:-false}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Logging function
log() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}"
}

warn() {
    echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] WARNING: $1${NC}"
}

error() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ERROR: $1${NC}"
    exit 1
}

# Check if running as root
check_root() {
    if [[ $EUID -ne 0 ]]; then
        error "This script must be run as root"
    fi
}

# Install dependencies
install_dependencies() {
    log "Installing dependencies..."
    
    # Update package list
    apt-get update -qq
    
    # Install required packages
    apt-get install -y \
        software-properties-common \
        apt-transport-https \
        ca-certificates \
        curl \
        gnupg \
        lsb-release \
        openssl \
        cron
    
    # Install Docker if not present
    if ! command -v docker &> /dev/null; then
        log "Installing Docker..."
        curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
        echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null
        apt-get update -qq
        apt-get install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin
        systemctl enable docker
        systemctl start docker
    fi
    
    # Install Docker Compose if not present
    if ! command -v docker-compose &> /dev/null; then
        log "Installing Docker Compose..."
        curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
        chmod +x /usr/local/bin/docker-compose
    fi
}

# Create directory structure
setup_directories() {
    log "Setting up directory structure..."
    
    mkdir -p "$WEBROOT"
    mkdir -p "$CERT_PATH"
    mkdir -p /var/log/marco2-ssl
    mkdir -p /etc/marco2/ssl
    
    # Set proper permissions
    chown -R www-data:www-data "$WEBROOT"
    chmod -R 755 "$WEBROOT"
}

# Generate DH parameters
generate_dhparam() {
    local dhparam_file="$CERT_PATH/dhparam.pem"
    
    if [[ ! -f "$dhparam_file" ]]; then
        log "Generating DH parameters (this may take a while)..."
        openssl dhparam -out "$dhparam_file" 2048
        chmod 600 "$dhparam_file"
    else
        log "DH parameters already exist"
    fi
}

# Create initial nginx configuration for ACME challenge
create_initial_nginx_config() {
    local config_file="/tmp/nginx-acme.conf"
    
    log "Creating initial nginx configuration for ACME challenge..."
    
    cat > "$config_file" << EOF
server {
    listen 80;
    listen [::]:80;
    server_name $DOMAIN $SUBDOMAINS;
    
    location /.well-known/acme-challenge/ {
        root $WEBROOT;
        try_files \$uri =404;
    }
    
    location / {
        return 301 https://\$server_name\$request_uri;
    }
}
EOF

    # Start temporary nginx container
    docker run -d --name nginx-acme \
        -p 80:80 \
        -v "$config_file:/etc/nginx/conf.d/default.conf:ro" \
        -v "$WEBROOT:$WEBROOT:ro" \
        nginx:alpine
}

# Obtain SSL certificates
obtain_certificates() {
    log "Obtaining SSL certificates..."
    
    local staging_flag=""
    if [[ "$STAGING" == "true" ]]; then
        staging_flag="--staging"
        warn "Using Let's Encrypt staging environment"
    fi
    
    # Build domain list
    local domain_args="-d $DOMAIN"
    for subdomain in $SUBDOMAINS; do
        domain_args="$domain_args -d $subdomain"
    done
    
    # Run certbot
    docker run --rm \
        -v "$CERT_PATH:/etc/letsencrypt" \
        -v "$WEBROOT:/var/www/certbot" \
        certbot/certbot certonly \
        --webroot \
        --webroot-path=/var/www/certbot \
        --email "$EMAIL" \
        --agree-tos \
        --no-eff-email \
        $staging_flag \
        $domain_args
    
    if [[ $? -eq 0 ]]; then
        log "Certificates obtained successfully"
    else
        error "Failed to obtain certificates"
    fi
}

# Setup certificate renewal
setup_renewal() {
    log "Setting up automatic certificate renewal..."
    
    # Create renewal script
    cat > /etc/marco2/ssl/renew-certs.sh << 'EOF'
#!/bin/bash

# Marco 2.0 Certificate Renewal Script
log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $1" >> /var/log/marco2-ssl/renewal.log
}

log "Starting certificate renewal check..."

# Attempt renewal
docker run --rm \
    -v /etc/nginx/ssl:/etc/letsencrypt \
    -v /var/www/certbot:/var/www/certbot \
    certbot/certbot renew --quiet

if [[ $? -eq 0 ]]; then
    log "Certificate renewal check completed"
    
    # Reload nginx if certificates were renewed
    if docker ps --filter "name=marco2-nginx" --format "table {{.Names}}" | grep -q marco2-nginx; then
        docker exec marco2-nginx nginx -s reload
        log "Nginx configuration reloaded"
    fi
else
    log "Certificate renewal failed"
fi
EOF

    chmod +x /etc/marco2/ssl/renew-certs.sh
    
    # Add cron job for automatic renewal (twice daily)
    (crontab -l 2>/dev/null; echo "0 */12 * * * /etc/marco2/ssl/renew-certs.sh") | crontab -
    
    log "Automatic renewal configured"
}

# Create SSL configuration
create_ssl_config() {
    log "Creating SSL configuration..."
    
    cat > "$CERT_PATH/ssl-params.conf" << EOF
# SSL Configuration for Marco 2.0
ssl_protocols TLSv1.2 TLSv1.3;
ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384;
ssl_prefer_server_ciphers off;
ssl_session_cache shared:SSL:10m;
ssl_session_timeout 1d;
ssl_session_tickets off;

# OCSP stapling
ssl_stapling on;
ssl_stapling_verify on;
resolver 8.8.8.8 8.8.4.4 valid=300s;
resolver_timeout 5s;

# DH parameters
ssl_dhparam $CERT_PATH/dhparam.pem;

# Security headers
add_header Strict-Transport-Security "max-age=63072000; includeSubDomains; preload" always;
add_header X-Frame-Options "DENY" always;
add_header X-Content-Type-Options "nosniff" always;
add_header X-XSS-Protection "1; mode=block" always;
add_header Referrer-Policy "strict-origin-when-cross-origin" always;
EOF
}

# Validate SSL configuration
validate_ssl() {
    log "Validating SSL configuration..."
    
    local cert_file="$CERT_PATH/live/$DOMAIN/fullchain.pem"
    local key_file="$CERT_PATH/live/$DOMAIN/privkey.pem"
    
    if [[ ! -f "$cert_file" ]]; then
        error "Certificate file not found: $cert_file"
    fi
    
    if [[ ! -f "$key_file" ]]; then
        error "Private key file not found: $key_file"
    fi
    
    # Verify certificate
    openssl x509 -in "$cert_file" -text -noout > /dev/null
    if [[ $? -eq 0 ]]; then
        log "Certificate validation passed"
    else
        error "Certificate validation failed"
    fi
    
    # Check certificate expiration
    local expiry_date=$(openssl x509 -in "$cert_file" -noout -enddate | cut -d= -f2)
    local expiry_epoch=$(date -d "$expiry_date" +%s)
    local current_epoch=$(date +%s)
    local days_until_expiry=$(( ($expiry_epoch - $current_epoch) / 86400 ))
    
    log "Certificate expires in $days_until_expiry days"
    
    if [[ $days_until_expiry -lt 30 ]]; then
        warn "Certificate expires in less than 30 days"
    fi
}

# Setup monitoring
setup_monitoring() {
    log "Setting up SSL monitoring..."
    
    # Create monitoring script
    cat > /etc/marco2/ssl/monitor-ssl.sh << 'EOF'
#!/bin/bash

# Marco 2.0 SSL Monitoring Script
DOMAIN="marco2.com"
CERT_PATH="/etc/nginx/ssl"
LOG_FILE="/var/log/marco2-ssl/monitor.log"

log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $1" >> "$LOG_FILE"
}

# Check certificate expiration
check_expiration() {
    local cert_file="$CERT_PATH/live/$DOMAIN/fullchain.pem"
    
    if [[ -f "$cert_file" ]]; then
        local expiry_date=$(openssl x509 -in "$cert_file" -noout -enddate | cut -d= -f2)
        local expiry_epoch=$(date -d "$expiry_date" +%s)
        local current_epoch=$(date +%s)
        local days_until_expiry=$(( ($expiry_epoch - $current_epoch) / 86400 ))
        
        log "Certificate expires in $days_until_expiry days"
        
        if [[ $days_until_expiry -lt 7 ]]; then
            log "WARNING: Certificate expires in less than 7 days!"
            # Send alert (implement your alerting mechanism here)
        fi
    else
        log "ERROR: Certificate file not found"
    fi
}

# Check SSL configuration
check_ssl_config() {
    if docker ps --filter "name=marco2-nginx" --format "table {{.Names}}" | grep -q marco2-nginx; then
        if docker exec marco2-nginx nginx -t > /dev/null 2>&1; then
            log "Nginx SSL configuration is valid"
        else
            log "ERROR: Nginx SSL configuration is invalid"
        fi
    else
        log "WARNING: Marco2 nginx container is not running"
    fi
}

check_expiration
check_ssl_config
EOF

    chmod +x /etc/marco2/ssl/monitor-ssl.sh
    
    # Add monitoring cron job (daily)
    (crontab -l 2>/dev/null; echo "0 6 * * * /etc/marco2/ssl/monitor-ssl.sh") | crontab -
    
    log "SSL monitoring configured"
}

# Cleanup temporary resources
cleanup() {
    log "Cleaning up temporary resources..."
    
    # Stop and remove temporary nginx container
    if docker ps -a --filter "name=nginx-acme" --format "table {{.Names}}" | grep -q nginx-acme; then
        docker stop nginx-acme > /dev/null 2>&1
        docker rm nginx-acme > /dev/null 2>&1
    fi
}

# Main execution
main() {
    log "Starting Marco 2.0 SSL/TLS setup..."
    
    check_root
    install_dependencies
    setup_directories
    generate_dhparam
    create_initial_nginx_config
    
    # Obtain certificates
    obtain_certificates
    
    # Cleanup temporary nginx
    cleanup
    
    create_ssl_config
    validate_ssl
    setup_renewal
    setup_monitoring
    
    log "SSL/TLS setup completed successfully!"
    log "Certificates are located in: $CERT_PATH"
    log "Automatic renewal is configured"
    log "Monitoring is active"
    
    # Display certificate information
    local cert_file="$CERT_PATH/live/$DOMAIN/fullchain.pem"
    if [[ -f "$cert_file" ]]; then
        echo
        echo "Certificate Information:"
        openssl x509 -in "$cert_file" -noout -subject -issuer -dates
    fi
}

# Trap cleanup on script exit
trap cleanup EXIT

# Run main function
main "$@"
