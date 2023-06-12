varying vec2 v_pos;

#ifdef VERTEX_SHADER
uniform mat3 u_model_matrix;
uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;

attribute vec2 a_pos;

void main() {
    v_pos = a_pos;
    vec3 pos = u_projection_matrix * u_view_matrix * u_model_matrix * vec3(a_pos, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform vec4 u_color;
uniform vec4 u_color_bg;
uniform float u_health;
uniform float u_width;

void main() {
    float len = length(v_pos);
    if (len > 1.0 || len < 1.0 - u_width) {
        discard;
    }

    float angle = atan(v_pos.y, v_pos.x); // Returns an angle from -pi to pi
    angle = angle / 3.14 + 0.5; // 0.0 is now down, and +/- 1.0 is up
    if (angle > 1.0) {
        angle -= 2.0;
    }

    vec4 color = u_color_bg;
    if (abs(angle) <= u_health) {
        color = u_color;
    }

    gl_FragColor = color;
}
#endif
