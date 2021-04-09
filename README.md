# Graphics Pipeline Exercise

## Made with Rust

This is a exercise program on the internals of a GPU pipeline. Based, mainly, on OpenGL. Inspired by the [3D Programming Fundamentals](https://www.youtube.com/playlist?list=PLqCJpWy5Fohe8ucwhksiv9hTF5sfid8lA) series by [ChiliTomatoNoodles](https://www.youtube.com/c/ChiliTomatoNoodle) on Youtube.

### Specifications

- The pipeline receives a set of primitives(triangle or lines), indexed or not, as input;
- Pixel and vertex shaders;
- Primitive clipping;
- Front-face culling;
- Texture mapping;
- Perspective correction.

### Pipeline stages

1) Input assembly
2) Vertex processing;
3) Primitive culling;
4) Viewport transform;
5) Scan conversion;
6) Pixel processing;
7) Screen draw.

### Dependencies

- Rust 1.48.0 or higher;
- SDL2.
