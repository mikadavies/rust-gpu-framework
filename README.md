# Rust GPU Framework
A basic framework for general programming with the GPU in Rust. For simplicity, the framework is targeted at Windows and Linux.

## Structure

The [framework](./src/framework/) folder contains most of the framework for interacting with the GPU through wgpu. The [windowed_app](./src/framework/windowed_app/) folder provides a basic framework to create windowed apps rendering through the GPU.

This framework is currently still a work in progress and is subject to change. It currently only supports windowed applications, but a GPGPU computing framework will be added soon. 
