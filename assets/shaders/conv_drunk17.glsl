varying vec2 v_vt;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;

void main() {
  v_vt = a_pos;
  gl_Position = vec4(a_pos.xy, 0., 1.);
}
#endif

#ifdef FRAGMENT_SHADER
#define kernel_width 17

uniform sampler2D u_texture;
uniform vec2 u_resolution;
uniform float u_time;

float rand(vec2 co) {
  return fract(sin(dot(co, vec2(12.9898, 78.233))) * 43758.5453);
}

void main() {
  vec2 uv = (v_vt + 1.) / 2.;
  vec2 pixelSize = 1. / u_resolution.xy;

  vec2 _half = vec2(kernel_width / 2);

  vec4 outColour = vec4(0.);
  outColour += texture2D(u_texture, uv + pixelSize * (vec2(0., 0.) - _half));
  outColour += texture2D(u_texture, uv + pixelSize * (vec2(16., 0.) - _half));
  outColour += texture2D(u_texture, uv + pixelSize * (vec2(8., 8.) - _half));
  outColour += texture2D(u_texture, uv + pixelSize * (vec2(0., 16.) - _half));
  outColour += texture2D(u_texture, uv + pixelSize * (vec2(16., 16.) - _half));

  if(rand(uv + u_time) < (outColour * .2).a - 0.01) {
    gl_FragColor = outColour * .25;
  } else {
    gl_FragColor = vec4(0.);
  }
}
#endif