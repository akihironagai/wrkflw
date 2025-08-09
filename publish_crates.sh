#!/bin/bash

# Simple script to publish all wrkflw crates to crates.io in dependency order

set -e

DRY_RUN=${1:-""}

if [[ "$DRY_RUN" == "--dry-run" ]]; then
    echo "🧪 DRY RUN: Testing wrkflw crates publication"
else
    echo "🚀 Publishing wrkflw crates to crates.io"
fi

# Check if we're logged in to crates.io
if [ ! -f ~/.cargo/credentials.toml ] && [ ! -f ~/.cargo/credentials ]; then
    echo "❌ Not logged in to crates.io. Please run: cargo login <your-token>"
    exit 1
fi

# Publication order (respecting dependencies)
CRATES=(
    "models"
    "logging" 
    "utils"
    "matrix"
    "validators"
    "github"
    "gitlab"
    "parser"
    "runtime"
    "evaluator"
    "executor"
    "ui"
    "wrkflw"
)

echo "📦 Publishing crates in dependency order..."

for crate in "${CRATES[@]}"; do
    if [[ "$DRY_RUN" == "--dry-run" ]]; then
        echo "Testing $crate..."
        cd "crates/$crate"
        cargo publish --dry-run --allow-dirty
        echo "✅ $crate dry-run successful"
    else
        echo "Publishing $crate..."
        cd "crates/$crate"
        cargo publish --allow-dirty
        echo "✅ Published $crate"
    fi
    cd - > /dev/null
    
    # Small delay to avoid rate limiting (except for the last crate and in dry-run)
    if [[ "$crate" != "wrkflw" ]] && [[ "$DRY_RUN" != "--dry-run" ]]; then
        echo "   Waiting 10 seconds to avoid rate limits..."
        sleep 10
    fi
done

if [[ "$DRY_RUN" == "--dry-run" ]]; then
    echo "🎉 All crates passed dry-run tests!"
    echo ""
    echo "To actually publish, run:"
    echo "  ./publish_crates.sh"
else
    echo "🎉 All crates published successfully!"
    echo ""
    echo "Users can now install wrkflw with:"
    echo "  cargo install wrkflw"
fi
