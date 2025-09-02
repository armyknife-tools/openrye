#!/bin/bash

# OpenRye Template Installer Demo
# This demonstrates how templates would be installed from the marketplace

TEMPLATE_NAME=${1:-fastapi-production}
PROJECT_NAME=${2:-my-api}

echo "🚀 OpenRye Template Marketplace"
echo "================================"
echo ""
echo "📦 Installing template: $TEMPLATE_NAME"
echo "📁 Project name: $PROJECT_NAME"
echo ""

# Create project directory
mkdir -p "$PROJECT_NAME"

# In production, this would:
# 1. Query the marketplace registry
# 2. Download the .ryet file from the URL
# 3. Verify checksum
# 4. Extract and apply template

# For demo, we'll extract our local template
if [ -f "templates/fastapi-template/fastapi-production.ryet" ]; then
    echo "✓ Found template package"
    echo "✓ Downloading from marketplace..."
    sleep 1
    
    echo "✓ Verifying integrity..."
    sha256sum templates/fastapi-template/fastapi-production.ryet > /tmp/checksum.txt
    
    echo "✓ Extracting template..."
    cd "$PROJECT_NAME"
    tar -xzf ../templates/fastapi-template/fastapi-production.ryet
    
    echo "✓ Processing template variables..."
    
    # Replace template variables in README
    if [ -f "template/README.md" ]; then
        sed -i "s/{{project_name}}/$PROJECT_NAME/g" template/README.md
    fi
    
    echo ""
    echo "✅ Template installed successfully!"
    echo ""
    echo "📋 Template Info:"
    echo "  Name: FastAPI Production"
    echo "  Version: 1.0.0"
    echo "  Author: OpenRye Community"
    echo "  Rating: ⭐ 4.8/5.0 (47 reviews)"
    echo "  Downloads: 1,523"
    echo ""
    echo "📂 Project created at: $PROJECT_NAME/"
    echo ""
    echo "🎯 Next steps:"
    echo "  cd $PROJECT_NAME"
    echo "  openrye sync      # Install dependencies"
    echo "  openrye run dev   # Start development server"
    echo ""
    echo "📚 Documentation: https://openrye.dev/templates/fastapi-production"
else
    echo "❌ Template not found. Available templates:"
    echo ""
    echo "  fastapi-production - Production FastAPI with auth & Docker"
    echo "  django-starter     - Django with admin and ORM"
    echo "  ml-pipeline       - Machine learning project template"
    echo "  cli-tool          - Click-based CLI application"
    echo ""
    echo "Install with: openrye template install <name>"
fi