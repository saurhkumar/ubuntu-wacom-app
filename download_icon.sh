#!/bin/bash

# Create a simple icon using ImageMagick or download one
if command -v convert &> /dev/null; then
    # Use ImageMagick to create a simple icon
    convert -size 128x128 radial-gradient:dodgerblue-navy \
            -font DejaVu-Sans-Bold -pointsize 40 -fill white \
            -gravity center -annotate 0 "HW" \
            resources/icon.png
    echo "Icon created at resources/icon.png"
else
    # Download a generic icon if ImageMagick is not available
    echo "Downloading a generic icon..."
    curl -s -o resources/icon.png "https://img.icons8.com/color/96/000000/sphere.png" || \
    wget -q -O resources/icon.png "https://img.icons8.com/color/96/000000/sphere.png"
    
    if [ -f resources/icon.png ]; then
        echo "Icon downloaded to resources/icon.png"
    else
        echo "Failed to download icon. Please install ImageMagick or manually add an icon at resources/icon.png"
    fi
fi
