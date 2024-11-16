#version 330 core

// Input from the vertex shader
in vec2 v_uv; // UV coordinates passed from the vertex shader

// Uniform variables
uniform float u_time; // Time variable to create the movement
uniform vec2 u_resolution; // Screen resolution to normalize coordinates

// Output color
out vec4 fragColor;

void main() {
    // Normalize UV coordinates (optional for certain effects)
    vec2 uv = v_uv;
   vec3 backgroundColor = vec3(0.1, 255, 0.1); 
    // Add infinite movement along the x-axis
    uv.x += u_time * 0.1; // Speed of movement

    // Create a simple pattern using sin
    float pattern = 0.5 + 0.5 * sin(uv.x * 10.0 + uv.y * 10.0);

    // Use the pattern to create a color
    vec3 color = vec3(pattern, pattern * 0.5, pattern * 0.8);

    // Output the final color
    fragColor = vec4(backgroundColor, 1.0);
}

