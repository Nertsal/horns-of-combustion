varying vec2 v_vt;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;

void main() {
  v_vt = a_pos;
  gl_Position = vec4(a_pos.xy, 0., 1.);
}
#endif

#ifdef FRAGMENT_SHADER
uniform sampler2D u_texture;
uniform sampler2D u_fireTexture;
uniform vec2 u_camera_pos;
uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
uniform vec2 u_shake;

void main() {
  vec2 uv = (v_vt + 1.) / 2.;
  vec4 inTex = texture2D(u_texture, uv);

  // Tile texture
  vec2 rpos = gl_FragCoord.xy / vec2(128);
  vec3 camera_translation = -inverse(u_projection_matrix * u_view_matrix) * vec3(0.0, 0.0, 1.0);
  vec2 translation = camera_translation.xy / camera_translation.z / 16.0;
  translation += u_shake;
  vec4 tiledTexture = texture2D(u_fireTexture, rpos + translation);

  // Add fire
  vec4 fireColour = vec4(.796, .184, .173, 1.);
  vec3 diff = inTex.xyz - fireColour.xyz;

  if (diff.x * diff.x + diff.y * diff.y + diff.z * diff.z < .01) {
    gl_FragColor = tiledTexture;
  } else {
    gl_FragColor = vec4(0.);
  }
}
#endif