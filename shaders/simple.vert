#version 140

in vec2 position;
out vec2 v_color;

void main() {
  v_color = position;
  gl_Position = vec4(position, 0.0, 1.0);
}
