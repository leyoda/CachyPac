#!/bin/bash

# Launcher script pour CachyRust
# Supprime les warnings WGPU/Vulkan pour une expérience utilisateur plus propre

# Supprimer les warnings WGPU et Vulkan
export WGPU_LOG_LEVEL=error
export VK_LOADER_DEBUG=none
export RUST_LOG=cachy_rust=info

# Filtrer tous les warnings WGPU/Vulkan
exec /usr/local/bin/cachy-rust "$@" 2>&1 | grep -v -E "(Unrecognized present mode|No config found|EGL says it can present|wgpu_hal::vulkan::conv.*WARN|wgpu_hal::gles::egl.*WARN)"